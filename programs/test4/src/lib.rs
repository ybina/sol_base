use anchor_lang::prelude::*;
use anchor_spl::{self, token::Token, associated_token::AssociatedToken};
use anchor_spl::token::{MintTo, Mint, TokenAccount};
//use anchor_spl::token::{InitializeMint, MintTo, Mint, TokenAccount};
//use mpl_token_metadata::types::DataV2;

declare_id!("7wEstvdML27vJt3LsNmoopaQip9GxGyKb9GPNHNFSZLs");

#[program]
pub mod test4 {
    use std::borrow::Borrow;

    // use anchor_lang::system_program;
    //use anchor_spl::{token::{initialize_mint, InitializeMint, mint_to, MintTo, transfer, Transfer, burn, Burn, freeze_account, FreezeAccount, close_account, CloseAccount, thaw_account, ThawAccount, set_authority, SetAuthority, spl_token::instruction::AuthorityType}, associated_token, metadata::{create_metadata_accounts_v3, create_master_edition_v3}};
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

    pub fn create_token(ctx: Context<CreateToken>, decimals:u8, amount:u64) -> Result<()> {

        // system_program::create_account(
        //     CpiContext::new(
        //         ctx.accounts.system_program.to_account_info(), 
        //         system_program::CreateAccount {
        //             from: ctx.accounts.signer.to_account_info(), 
        //             to: ctx.accounts.mint_token.to_account_info()}
        //     ), 
        //     10_000_000, 
        //     82,
        //     &anchor_spl::token::ID,
        // )?;
        // anchor_spl::token::initialize_mint(
        //     CpiContext::new(
        //     ctx.accounts.token_program.to_account_info(),
        //     InitializeMint{
        //         mint:ctx.accounts.mint_token.to_account_info(),
        //         rent:ctx.accounts.rent.to_account_info()}
        // ), 
        // decimals,
        // // 铸造权限设为程序PDA
        // ctx.accounts.authority_pda.key,
        // // 不设置冻结权限
        // None)?;
        
        // anchor_spl::associated_token::create(
        //     CpiContext::new(
        //         ctx.accounts.associate_token_program.to_account_info(), 
        //         anchor_spl::associated_token::Create { 
        //             payer: ctx.accounts.signer.to_account_info(), 
        //             associated_token: ctx.accounts.token_pda.to_account_info(), 
        //             authority: ctx.accounts.authority_pda.to_account_info(),
        //             mint: ctx.accounts.mint_token.to_account_info(), 
        //             system_program: ctx.accounts.system_program.to_account_info(), 
        //             token_program: ctx.accounts.token_program.to_account_info() 
        //         }
        //     ).with_signer(&[&[b"token_pda", &ctx.accounts.counter.count.to_le_bytes()]])
        // )?;
        let bump_seed = ctx.bumps.mint_token;
        
        let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[bump_seed]]];
        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo{
                    authority: ctx.accounts.authority_pda.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    to:ctx.accounts.token_pda.to_account_info()}
            ).with_signer(signer_seeds), 
            amount
        )?;
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
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
    #[account()]
    pub authority_pda: AccountInfo<'info>,
    ///CHECK:
    #[account(
        init,
        payer=signer,
        seeds = [b"mint".as_ref(), &counter.count.to_le_bytes()],
        // space = 8 + 32 + std::mem::size_of::<Mint>() + 8,
        bump,
        mint::decimals = 9,
        mint::authority = authority_pda,
    )]
    pub mint_token:Account<'info, Mint>,

    ///CHECK:
    #[account(mut)]
    pub counter: Box<Account<'info, Counter>>,

    ///CHECK:
    #[account(
        init,
        payer=signer,
        associated_token::mint = mint_token,
        associated_token::authority = authority_pda,
    )]
    pub token_pda: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer:Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}

#[account]
pub struct Counter {
    pub count: u64,
    pub is_initialized: bool,
}