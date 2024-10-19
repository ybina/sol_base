use anchor_lang::prelude::*;
use anchor_spl::{self, token::Token, associated_token::AssociatedToken};
use anchor_spl::token::{self, MintTo, Mint, TokenAccount, Transfer};
use mpl_token_metadata::instructions;

declare_id!("7pzBufvwPaFVQ1NbZL7MRBEkvUisrDdB8yd4bvK6ph3Z");

#[program]
pub mod test4 {
    use mpl_token_metadata::types::DataV2;
    use anchor_lang::solana_program::program::invoke;
    use anchor_lang::solana_program::system_instruction;
    use crate::AmiErrorCode;
    use crate::ami_private::cal_price;
    
    use super::*;

    pub const SOL_VALUE_BASE: f64 = 1_000_000_000.0;
    pub const START_SOL_VIRTUAL_VALUE: f64 = 10.0;
    pub const TOKEN_TOTAL_SUPPLY: u64 = 1500000000;

    pub fn init_program(ctx: Context<InitAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        if counter.is_initialized {
            return Ok(());
        }
        counter.count = 0;
        counter.is_initialized = true;
        //msg!("initialized!");
        Ok(())
    }

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, uri: String) -> Result<()> {
        
        //msg!("---auth_pda:{}", ctx.accounts.authority_pda.key);
        msg!("---mint_pda:{}", ctx.accounts.mint_pda.key());
        msg!("---token_pda:{}", ctx.accounts.token_pda.key());
        // let token_name:String = name.clone();
        // let token_symbol:String = symbol.clone();
        // let (expected_metadata_pda, bump) = Pubkey::find_program_address(
        //     &[
        //         b"metadata",
        //         mpl_token_metadata::ID.as_ref(),
        //         ctx.accounts.mint_pda.key().as_ref()
        //     ],
        //     &mpl_token_metadata::ID
        // );
        // msg!("---expected_metadata_pda:{}", expected_metadata_pda.key());
        // msg!("---expexted_metadata_pda_bump:{}",  bump);

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
            1000000000000000000,
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
        // ---first buy----
        // let sol_amount_lamp: f64 = sol_amount * SOL_VALUE_BASE;
        // // sol trans
        // invoke(
        //     &system_instruction::transfer(
        //         &ctx.accounts.signer.key(),
        //         &ctx.accounts.sol_valt_pda.key(),
        //         sol_amount_lamp as u64,
        //     ),
        //     &[
        //         ctx.accounts.signer.to_account_info(), 
        //         ctx.accounts.sol_valt_pda.to_account_info(), 
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;
        // let end_sol_val = sol_amount;
        // let buy_pri = cal_price(0.0, end_sol_val);
        // let token_amount = sol_amount / buy_pri * 1_000_000_000.0;
        // // spl-token trans
        // let cpi_accounts = Transfer {
        //     from: ctx.accounts.token_pda.to_account_info(),
        //     to: ctx.accounts.target_token_account.to_account_info(),
        //     authority: ctx.accounts.authority_pda.to_account_info(),
        // };
        
        // let cpi_program = ctx.accounts.token_program.to_account_info();
        // let seeds_binding = [auth_signer_seeds];
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(&seeds_binding);
        // token::transfer(cpi_ctx, token_amount as u64)?;

        // emit!(CreateTokenEvent {
        //     sol_amount: sol_amount,
        //     token_amount: token_amount,
        //     sol_price: buy_pri,
        //     token_name:token_name,
        //     token_symbol:token_symbol,
        //     mint_addr: ctx.accounts.mint_pda.key(),
        //     token_addr: ctx.accounts.token_pda.key(),
        // });
        Ok(())
    }


    pub fn buy(ctx: Context<BuyTokenAccounts>, sol_amount_req: f64) -> Result<()> {
        if ctx.accounts.trade_status.trade_status {
            return Err(AmiErrorCode::InvalidTradeStatus.into());
        }
        if sol_amount_req > 5.0 {
            return Err(AmiErrorCode::InvalidAmount.into());
        }
        msg!("sol_valt_pda:{}",ctx.accounts.sol_valt_pda.key());
        msg!("create new token account to signer finished: {}", ctx.accounts.target_token_account.key());
        let start_sol_lamp =  ctx.accounts.sol_valt_pda.to_account_info().lamports();
        let start_sol_val =  start_sol_lamp as f64 / 1_000_000_000.0;
        let sol_amount: f64;

        if sol_amount_req + start_sol_val >= 59.0 {
            sol_amount = 59.0 - sol_amount_req;
            ctx.accounts.trade_status.trade_status = true
        } else {
            sol_amount = sol_amount_req
        }

        let trans_sol_lamp = sol_amount * SOL_VALUE_BASE;
        let auth_bump = ctx.bumps.authority_pda;
        let auth_signer_seeds: &[&[u8]] = &[b"authority_pda", &[auth_bump]];
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &ctx.accounts.sol_valt_pda.key(),
                trans_sol_lamp as u64,
            ),
            &[
                ctx.accounts.signer.to_account_info(), 
                ctx.accounts.sol_valt_pda.to_account_info(), 
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        msg!("buy transfer sol finished:{}", sol_amount);
        msg!("sol_valt_balance:{}", ctx.accounts.sol_valt_pda.lamports());
        let end_sol_val = start_sol_val + sol_amount;
        let buy_pri = cal_price(start_sol_val, end_sol_val);

        let buy_amount = sol_amount / buy_pri * 1_000_000_000.0;
        msg!("start_sol:{}, end_sol:{}, buy_pri:{}, buy_amount:{}", start_sol_val, end_sol_val, buy_pri, buy_amount);
        let cpi_accounts = Transfer {
            from: ctx.accounts.token_pda.to_account_info(),
            to: ctx.accounts.target_token_account.to_account_info(),
            authority: ctx.accounts.authority_pda.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let seeds_binding = [auth_signer_seeds];
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(&seeds_binding);
        token::transfer(cpi_ctx, buy_amount as u64)?;
        emit!(TradeTokenEvent{
            trade_type: true,
            sol_amount: sol_amount,
            token_amount: buy_amount,
            sol_price: buy_pri,
            mint_addr: ctx.accounts.mint_pda.key(),
            token_addr: ctx.accounts.token_pda.key(),
        });
        Ok(())
    }

    pub fn sell(ctx: Context<SellTokenAccounts>, sol_amount: f64) -> Result<()> {
        if sol_amount > 5.0 {
            return Err(AmiErrorCode::InvalidAmount.into());
        }
        msg!("sol_valt addr: {}", ctx.accounts.sol_valt_pda.key());
        msg!("sell amount: {}", sol_amount);
        msg!("start sol_valt_balance:{}", ctx.accounts.sol_valt_pda.lamports());
        // not an empty account
        if ctx.accounts.trade_status.get_lamports() > 0 && ctx.accounts.trade_status.to_account_info().data_len() > 0{
            // have launched to raydium, can not sell here
            if ctx.accounts.trade_status.trade_status {
                return Err(AmiErrorCode::InvalidTradeStatus.into());
            }
        }
        //let auth_bump = ctx.bumps.authority_pda;
        //let auth_signer_seeds: &[&[u8]] = &[b"authority_pda", &[auth_bump]];

        let total_sol_lamp = ctx.accounts.sol_valt_pda.to_account_info().lamports();
        if sol_amount > (total_sol_lamp as f64 / SOL_VALUE_BASE) {
            return Err(AmiErrorCode::InsufficientFunds.into());
        }
        
        let start_sol_lamp =  ctx.accounts.sol_valt_pda.to_account_info().lamports();
        let start_sol_val =  start_sol_lamp as f64 / SOL_VALUE_BASE;
        let end_sol_val = start_sol_val + sol_amount;
        let price = cal_price(start_sol_val, end_sol_val);

        let token_required = sol_amount / price * 1_000_000_000.0;
        msg!("sell token required:{}", sol_amount / price);
        // transfer signer's spl token to program token_pda 
        let cpi_accounts = Transfer {
            from:ctx.accounts.target_token_account.to_account_info(), 
            to: ctx.accounts.token_pda.to_account_info(),
            authority:ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, token_required as u64)?;
        let sol_amount_lamp = sol_amount * 1_000_000_000.0;
        msg!("request sol:{}", sol_amount_lamp);
        // transfer sol of token_pda to signer
        msg!("SELL: start transfer Sol to siger:{}", sol_amount);
        // invoke_signed(
        //     &system_instruction::transfer(
        //         &ctx.accounts.token_pda.key(), 
        //         &ctx.accounts.signer.key(), 
        //         sol_amount_lamp as u64),
        //     &[
        //         ctx.accounts.token_pda.to_account_info(),
        //         ctx.accounts.signer.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     &[auth_signer_seeds]
        // )?;
        // invoke(
        //     &system_instruction::transfer(
        //         &ctx.accounts.sol_valt_pda.key(), 
        //         &ctx.accounts.signer.key(), 
        //         sol_amount_lamp as u64),
        //     &[
        //         ctx.accounts.token_pda.to_account_info(),
        //         ctx.accounts.signer.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;
        let sol_valt_pda_info = &ctx.accounts.sol_valt_pda.to_account_info();
        let signer_info = &ctx.accounts.signer.to_account_info();
        **sol_valt_pda_info.try_borrow_mut_lamports()? -= sol_amount_lamp as u64;
        **signer_info.try_borrow_mut_lamports()? += sol_amount_lamp as u64;
        msg!("after sell sol_valt balance:{}", ctx.accounts.sol_valt_pda.lamports());
        emit!(TradeTokenEvent{
            trade_type: false,
            sol_amount: sol_amount,
            token_amount: token_required,
            sol_price: price,
            mint_addr: ctx.accounts.mint_pda.key(),
            token_addr: ctx.accounts.token_pda.key(),
        });
        Ok(())
    }

}

mod ami_private {
    #[inline(never)]
    pub fn cal_price(start_sol: f64, end_sol: f64) -> f64 {
        
        let area_a = start_sol * start_sol * start_sol;
        let area_b = end_sol * end_sol * end_sol;
        let mut l  = (start_sol + end_sol) / 2.0;
        let mut r = end_sol;
        // 积分精度目标
        let target = 0.0000001;

        for _ in 0..100 {
            let x = (l + r) / 2.0;
            let area_x = x * x * x;
            let diff_a =  area_x - area_a;
            let diff_b = area_b - area_x;
            let diff = diff_b - diff_a;
            if diff.abs() < target {
                return round_to_10_decimal_price(x)
            } else {
                if diff < 0.0 {
                    r = x;
                } else {
                    l = x;
                }
            }
        }
        -1.0
    }

    fn round_to_10_decimal_price(value: f64) -> f64 {
        let pri: f64 = (value * value) / 8350000000.0 + 0.000000023;
        (pri * 10000000000.0).round() / 10000000000.0
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
    pub mint_pda: Box<Account<'info, Mint>>,

    ///CHECK:
    #[account(
        mut,
        seeds = [b"account_counter".as_ref()],
        bump,
    )]
    pub counter: Box<Account<'info, Counter>>,

    ///CHECK:
    #[account(
        init,
        payer=signer,
        associated_token::mint = mint_pda,
        associated_token::authority = authority_pda,
    )]
    pub token_pda: Box<Account<'info, TokenAccount>>,

    ///CHECK:
    #[account(
        mut,
        seeds=[b"metadata", token_metadata_program.key().as_ref(), mint_pda.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,

    ///CHECK:
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_pda,
        associated_token::authority = signer,
    )]
    pub target_token_account: Box<Account<'info, TokenAccount>>,

    ///CHECK:
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>,
    pub token_program: Program<'info,Token>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct BuyTokenAccounts<'info> {

    ///CHECK:
    #[account(
        mut,
        seeds = [b"authority_pda".as_ref()],
        bump
    )]
    pub authority_pda: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
    )]
    pub mint_pda: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
    )]
    pub token_pda: Account<'info, TokenAccount>,

    ///CHECK:
    #[account(
        init_if_needed,
        payer=signer,
        space = 8 + 32 + std::mem::size_of::<AccountInfo>() + 8,
        seeds=[b"sol_valt", token_pda.key().as_ref()],
        bump
    )]
    pub sol_valt_pda: AccountInfo<'info>,

    ///CHECK:
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_pda,
        associated_token::authority = signer,
    )]
    pub target_token_account: Box<Account<'info, TokenAccount>>,

    ///CHECK:
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + std::mem::size_of::<TokenTradeStatus>() + 8,
        seeds = [b"trade_status".as_ref(), token_pda.key().as_ref()],
        bump,
    )]
    pub trade_status: Box<Account<'info, TokenTradeStatus>>,

    ///CHECK:
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    #[account(
        mut
    )]
    pub signer:Signer<'info>,
}

#[derive(Accounts)]
pub struct SellTokenAccounts<'info> {

    ///CHECK:
    #[account(
        mut,
        seeds = [b"authority_pda".as_ref()],
        bump
    )]
    pub authority_pda: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut
    )]
    pub mint_pda: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut
    )]
    pub token_pda: Account<'info, TokenAccount>,

    ///CHECK:
    #[account(
        mut,
        seeds=[b"sol_valt", token_pda.key().as_ref()],
        bump
    )]
    pub sol_valt_pda: AccountInfo<'info>,

    // ///CHECK:
    #[account(
        mut,
        associated_token::mint = mint_pda,
        associated_token::authority = signer,
    )]
    pub target_token_account: Account<'info, TokenAccount>,

    ///CHECK:
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + std::mem::size_of::<TokenTradeStatus>() + 8,
        seeds = [b"trade_status".as_ref(), token_pda.key().as_ref()],
        bump,
    )]
    pub trade_status: Box<Account<'info, TokenTradeStatus>>,

    ///CHECK:
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    #[account(
        mut
    )]
    pub signer:Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub is_initialized: bool,
}

#[account]
pub struct TokenTradeStatus {
    pub trade_status: bool,
}

#[error_code]
pub enum AmiErrorCode {
    #[msg("Insufficient funds in the pool")]
    InsufficientFunds,
    #[msg("Conversion error")]
    ConversionError,
    #[msg("Invalid amount request")]
    InvalidAmount,
    #[msg("can not trade on bonding curve")]
    InvalidTradeStatus
}

#[event]
pub struct CreateTokenEvent {
    pub sol_amount: f64,
    pub token_amount: f64,
    pub sol_price: f64,
    pub token_name: String,
    pub token_symbol: String,
    pub mint_addr: Pubkey,
    pub token_addr: Pubkey,
}

#[event]
pub struct TradeTokenEvent {
    pub trade_type: bool,
    pub sol_amount: f64,
    pub token_amount: f64,
    pub sol_price: f64,
    pub mint_addr: Pubkey,
    pub token_addr: Pubkey,
}