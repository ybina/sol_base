use anchor_lang::prelude::*;
use anchor_spl::{token::Token, associated_token::AssociatedToken};
use anchor_spl::token::{InitializeMint, MintTo};
//use mpl_token_metadata::types::DataV2;

declare_id!("8S57MEuBNMeCtV1sgbWJhms7ESWArx6kLpppCy7MLZBR");

#[program]
pub mod test4 {
    use anchor_lang::system_program;
    //use anchor_spl::{token::{initialize_mint, InitializeMint, mint_to, MintTo, transfer, Transfer, burn, Burn, freeze_account, FreezeAccount, close_account, CloseAccount, thaw_account, ThawAccount, set_authority, SetAuthority, spl_token::instruction::AuthorityType}, associated_token, metadata::{create_metadata_accounts_v3, create_master_edition_v3}};
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>,decimals:u8,amount:u64) -> Result<()> {

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount { 
                    from: ctx.accounts.signer.to_account_info(), 
                    to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, 
            82, 
            ctx.accounts.token_program.key
        )?;
        anchor_spl::token::initialize_mint(
            CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeMint{
                mint:ctx.accounts.mint_token.to_account_info(),
                rent:ctx.accounts.rent.to_account_info()}
        ), 
        decimals, 
        ctx.accounts.signer.key, 
        Some(ctx.accounts.signer.key))?;
        
        anchor_spl::associated_token::create(
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                anchor_spl::associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;

        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                MintTo{
                    authority:ctx.accounts.signer.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    to:ctx.accounts.token_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}
