use anchor_lang::prelude::*;
use anchor_spl::{self, token::Token, associated_token::AssociatedToken};
use anchor_spl::token::{MintTo, Mint, TokenAccount};
use mpl_token_metadata::instructions;

declare_id!("DCZKxJycaXxy1uhnJYdZUMisCAXDmWkmY1QkFdgoJVka");

pub const RAYDIUM_V3_PROGRAM_DEV_ADDR: Pubkey = pubkey!("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");
pub const WSOL_MINT_ADDR: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const AMM_CONFIG_ADDR_DEV: Pubkey = pubkey!("CQYbhr6amxUER4p5SC44C63R4qw4NFc9Z4Db9vF4tZwG");


#[program]
pub mod test4 {
    use mpl_token_metadata::types::DataV2;
    use crate::local_create_pool::create_raydium_pool;

    use super::*;

    pub fn init_program(ctx: Context<InitAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        if counter.is_initialized {
            return Ok(());
        }
        counter.count = 0;
        counter.is_initialized = true;
        msg!("initialized!");
        Ok(())
    }

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, uri: String) -> Result<()> {
        msg!("---auth_pda:{}", ctx.accounts.authority_pda.key);
        msg!("---mint_pda:{}", ctx.accounts.mint_pda.key());
        msg!("---token_pda:{}", ctx.accounts.token_pda.key());
        let (expected_metadata_pda, bump) = Pubkey::find_program_address(
            &[
                b"metadata",
                mpl_token_metadata::ID.as_ref(),
                ctx.accounts.mint_pda.key().as_ref()
            ],
            &mpl_token_metadata::ID
        );
        msg!("---expected_metadata_pda:{}", expected_metadata_pda.key());
        msg!("---expexted_metadata_pda_bump:{}",  bump);

        let auth_bump = ctx.bumps.authority_pda;
        let auth_signer_seeds: &[&[u8]] = &[b"authority_pda", &[auth_bump]];
        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo{
                    authority: ctx.accounts.authority_pda.to_account_info(),
                    mint:ctx.accounts.mint_pda.to_account_info(),
                    to:ctx.accounts.token_pda.to_account_info()}
            ).with_signer(&[auth_signer_seeds]),
            1000000000000,
        )?;

        instructions::CreateMetadataAccountV3Cpi::new(
            &ctx.accounts.token_metadata_program.to_account_info(),
            instructions::CreateMetadataAccountV3CpiAccounts {
                payer: &ctx.accounts.signer.to_account_info(),
                metadata: &ctx.accounts.metadata.to_account_info(),
                mint: &ctx.accounts.mint_pda.to_account_info(),
                mint_authority: &ctx.accounts.authority_pda.to_account_info(),
                update_authority: (&ctx.accounts.authority_pda.to_account_info(),false),
                system_program: &ctx.accounts.system_program.to_account_info(),
                rent: Some(&ctx.accounts.rent.to_account_info()),
            },
            instructions::CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name:name,
                    symbol:symbol,
                    uri:uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection:None,
                    uses:None
                },
                is_mutable: false,
                collection_details:None,
            }
        ).invoke_signed(&[auth_signer_seeds])?;
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("---tx end counter:{}", counter.count);

        // transfer 0.5 sol from signer to token_pda
        let lamports_to_transfer = 0.5 * 1_000_000_000u64 as f64; // 0.5 SOL in lamports
        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.token_pda.to_account_info(),
        };

        let transfer_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_instruction);
        anchor_lang::system_program::transfer(transfer_ctx, lamports_to_transfer as u64)?;
        
        let create_pool_str = CreateRaydiumPool {
            amm_program: ctx.accounts.amm_program.to_account_info(),
            amm_pool: ctx.accounts.amm_.to_account_info(),
            amm_authority: ctx.accounts.amm_authority.to_account_info(),
            amm_open_orders: ctx.accounts.amm_open_orders.to_account_info(),
            lp_mint: ctx.accounts.lp_mint.to_account_info(),
            coin_mint: ctx.accounts.mint_pda.to_account_info(),  // 使用mint_pda作为coin代币
            pc_mint: ctx.accounts.wsol_mint.to_account_info(),   // 使用WSOL作为pc代币
            coin_vault: ctx.accounts.coin_vault.to_account_info(),
            pc_vault: ctx.accounts.pc_vault.to_account_info(),
            target_orders: ctx.accounts.target_orders.to_account_info(),
            amm_config: ctx.accounts.amm_config.to_account_info(),
            fee_destination: ctx.accounts.fee_destination.to_account_info(),
            market_program: ctx.accounts.market_program.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            user_wallet: ctx.accounts.signer.to_account_info(),
            user_token_coin: ctx.accounts.token_pda.to_account_info(),
            user_token_pc: ctx.accounts.wsol_token_account.to_account_info(),  // 用户的WSOL账户
            user_token_lp: ctx.accounts.user_token_lp.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            sysvar_rent: ctx.accounts.sysvar_rent.to_account_info(),
        };
        // create raydium pool
        create_raydium_pool(create_pool_str, 1, 1, 2);
        Ok(())
    }
}

pub mod local_create_pool {
    use anchor_lang::solana_program::program::invoke_signed;
    use crate::CreateRaydiumPool;
    use anchor_lang::prelude::*;
    use raydium_contract_instructions::amm_instruction;

    pub fn create_raydium_pool(create_pool_str: CreateRaydiumPool,
        nonce: u8,
        init_pc_amount: u64,
        init_coin_amount: u64,) -> Result<()> {
        
            let opentime = Clock::get()?.unix_timestamp as u64;

            let initialize_ix = amm_instruction::initialize2(

                create_pool_str.amm_program.key,
                create_pool_str.amm_pool.key,
                create_pool_str.amm_authority.key,
                create_pool_str.amm_open_orders.key,
                create_pool_str.lp_mint.key,
                create_pool_str.coin_mint.key,
                create_pool_str.pc_mint.key,
                create_pool_str.coin_vault.key,
                create_pool_str.pc_vault.key,
                create_pool_str.amm_config.key,
                create_pool_str.amm_config.key,
                create_pool_str.fee_destination.key,
                create_pool_str.market_program.key,
                create_pool_str.market.key,

            //  change this to PDA address
            create_pool_str.user_wallet.key,
            create_pool_str.user_token_coin.key,
            create_pool_str.user_token_pc.key,
            create_pool_str.user_token_lp.key,
                nonce,
             opentime,
                init_pc_amount,
                init_coin_amount,
            )?;
            
            let create_pool_accounts = [
                create_pool_str.amm_program.clone(),
                create_pool_str.amm_pool.clone(),
                create_pool_str.amm_authority.clone(),
                create_pool_str.amm_open_orders.clone(),
                create_pool_str.lp_mint.clone(),
                create_pool_str.coin_mint.clone(),
                create_pool_str.pc_mint.clone(),
                create_pool_str.coin_vault.clone(),
                create_pool_str.pc_vault.clone(),
                create_pool_str.target_orders.clone(),
                create_pool_str.amm_config.clone(),
                create_pool_str.fee_destination.clone(),
                create_pool_str.market_program.clone(),
                create_pool_str.market.clone(),
                create_pool_str.user_wallet.to_account_info().clone(),
                create_pool_str.user_token_coin.clone(),
                create_pool_str.user_token_pc.clone(),
                create_pool_str.user_token_lp.clone(),
                create_pool_str.token_program.to_account_info().clone(),
                create_pool_str.system_program.to_account_info().clone(),
                create_pool_str
                    .associated_token_program
                    .to_account_info()
                    .clone(),
                    create_pool_str.sysvar_rent.to_account_info().clone(),
            ];
            invoke_signed(&initialize_ix, &create_pool_accounts, &[])?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateRaydiumPool<'info> {
    /// CHECK: Safe
    pub amm_program: AccountInfo<'info>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
    /// CHECK: Safe. The associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Safe. System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe. Rent program
    pub sysvar_rent: Sysvar<'info, Rent>,
    /// CHECK: Safe. 
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"amm_associated_seed"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_pool: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        seeds = [b"amm authority"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"open_order_associated_seed"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"lp_mint_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe. Coin mint account
    #[account(
        owner = token_program.key()
    )]
    pub coin_mint: AccountInfo<'info>,
    /// CHECK: Safe. Pc mint account
    #[account(
        owner = token_program.key()
    )]
    pub pc_mint: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"coin_vault_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub coin_vault: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"pc_vault_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub pc_vault: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"target_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub target_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        seeds = [b"amm_config_account_seed"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_config: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(mut)]
    pub fee_destination: AccountInfo<'info>,
    /// CHECK: Safe. OpenBook program.
    pub market_program: AccountInfo<'info>,
    /// CHECK: Safe. OpenBook market. OpenBook program is the owner.
    #[account(
        owner = market_program.key(),
    )]
    pub market: AccountInfo<'info>,
    /// CHECK: Safe. The user wallet create the pool
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    /// CHECK: Safe. The user coin token
    #[account(
        mut,
        owner = token_program.key(),
    )]
    pub user_token_coin: AccountInfo<'info>,
    /// CHECK: Safe. The user pc token
    #[account(
        mut,
        owner = token_program.key(),
    )]
    pub user_token_pc: AccountInfo<'info>,
    /// CHECK: Safe. The user lp token
    #[account(
        mut,
        seeds = [
            &user_wallet.key().to_bytes(),
            &token_program.key().to_bytes(),
            &lp_mint.key.to_bytes(),
            ],
        bump,
    )]
    pub user_token_lp: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitAccounts<'info> {
    ///CHECK:
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + std::mem::size_of::<Counter>() + 8,
        seeds = [b"account_counter".as_ref()],
        bump,
    )]
    pub counter: Box<Account<'info, Counter>>,

    ///CHECK:
    #[account(
        init,
        payer=signer,
        seeds = [b"authority_pda".as_ref()],
        space = 8 + 32 + std::mem::size_of::<AccountInfo>() + 8,
        bump
    )]
    pub authority_pda: AccountInfo<'info>,

    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    ///CHECK:
    #[account(
        mut,
        seeds = [b"authority_pda".as_ref()],
        bump
    )]
    pub authority_pda: AccountInfo<'info>,
    ///CHECK:
    #[account(
        init,
        payer=signer,
        seeds = [b"mint".as_ref(), &counter.count.to_le_bytes()],
        bump,
        mint::decimals = 9,
        mint::authority = authority_pda,
    )]
    pub mint_pda:Account<'info, Mint>,

    ///CHECK:
    #[account(mut)]
    pub counter: Box<Account<'info, Counter>>,

    ///CHECK:
    #[account(
        init,
        payer=signer,
        associated_token::mint = mint_pda,
        associated_token::authority = authority_pda,
    )]
    pub token_pda: Account<'info, TokenAccount>,

    ///CHECK:
    #[account(
        mut,
        seeds=[b"metadata", token_metadata_program.key().as_ref(), mint_pda.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    ///CHECK:
    pub raydium_amm_program: UncheckedAccount<'info>,

    ///CHECK:
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub rent:Sysvar<'info, Rent>,
    //--------Raydium------
    ///CHECK:
    #[account(address = RAYDIUM_V3_PROGRAM_DEV_ADDR)]
    pub amm_program:UncheckedAccount<'info>,
    /// CHECK: Safe. 
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"amm_associated_seed"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_pool: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        seeds = [b"amm authority"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_authority: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"open_order_associated_seed"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_open_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"lp_mint_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Safe. Coin mint account
    #[account(
        owner = token_program.key()
    )]
    pub coin_mint: AccountInfo<'info>,
    /// CHECK: Safe. Pc mint account
    #[account(
        owner = token_program.key()
    )]
    pub pc_mint: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"coin_vault_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub coin_vault: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"pc_vault_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub pc_vault: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        mut,
        seeds = [
            amm_program.key.as_ref(),
            market.key.as_ref(),
            b"target_associated_seed"
        ],
        bump,
        seeds::program = amm_program.key
    )]
    pub target_orders: AccountInfo<'info>,
    /// CHECK: Safe
    #[account(
        seeds = [b"amm_config_account_seed"],
        bump,
        seeds::program = amm_program.key
    )]
    pub amm_config: AccountInfo<'info>,
    /// CHECK: Safe. OpenBook program.
    pub market_program: AccountInfo<'info>,
    /// CHECK: Safe. OpenBook market. OpenBook program is the owner.
    #[account(
        owner = market_program.key(),
    )]
    pub market: AccountInfo<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub is_initialized: bool,
}