use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::{self, token::Token, associated_token::AssociatedToken};
use anchor_spl::token::{MintTo, Mint, TokenAccount};
use mpl_token_metadata::instructions;

pub mod raydium_instructions;
use raydium_instructions::local_raydium_clmm::cpi_create_pool;
// use anchor_spl::token_interface::{Mint, TokenInterface};

pub const RAYDIUM_V3_PROGRAM_DEV_ADDR: &str= "devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH";
pub const WSOL_MINT_ADDR: &str = "So11111111111111111111111111111111111111112";
pub const AMM_CONFIG_ADDR_DEV: &str = "CQYbhr6amxUER4p5SC44C63R4qw4NFc9Z4Db9vF4tZwG";

declare_id!("JC4rGhSfS4YEv7Ejo35waBuZ71JJ6puPUXpJ6ERRcDxJ");

#[program]
pub mod test4 {
    use mpl_token_metadata::types::DataV2;
    
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
        msg!("transfer 0.5 sol to token_pda finished");
        

        // -----
        let sqrt_price_x64: u128 = 1583337266871351588490;
        let open_time: u64 = 1728543531;
        cpi_create_pool(
            sqrt_price_x64, 
            open_time, 
            ctx.accounts.token_pda.to_account_info(), 
            ctx.accounts.amm_config.clone(), 
            ctx.accounts.mint_pda.to_account_info(), 
            ctx.accounts.wsol_mint.clone(), 
            ctx.accounts.token_program.to_account_info(), 
            ctx.accounts.token_program.to_account_info(),
            auth_signer_seeds)?;
        // -----
        msg!("all tx finished");
        Ok(())
    }
    
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
    #[account(address = Pubkey::from_str(WSOL_MINT_ADDR).unwrap())]
    pub wsol_mint: AccountInfo<'info>,

    ///CHECK:
    #[account(
        init,
        payer=signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = authority_pda,
    )]
    pub wsol_pda: Account<'info, TokenAccount>,
    ///CHECK:
    #[account(address = Pubkey::from_str(AMM_CONFIG_ADDR_DEV).unwrap())]
    pub amm_config: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds=[b"metadata", token_metadata_program.key().as_ref(), mint_pda.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    ///CHECK:
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub rent:Sysvar<'info, Rent>,
    ///CHECK:
    #[account(address = Pubkey::from_str(RAYDIUM_V3_PROGRAM_DEV_ADDR).unwrap())]
    pub raydium_clmm_program: UncheckedAccount<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub is_initialized: bool,
}
