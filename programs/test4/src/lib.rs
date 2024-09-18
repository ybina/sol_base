use anchor_lang::prelude::*;
use anchor_spl::{self, token::Token, associated_token::AssociatedToken};
use anchor_spl::token::{MintTo, Mint, TokenAccount};
use mpl_token_metadata::instructions;

declare_id!("GuYrycnKrFddawdYgJBqSajswksuEqTxYaqrsGiBCXPu");

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

        let metadata_bump = ctx.bumps.metadata;
        msg!("---ctx metadata_bump: {}", metadata_bump);
        let binding = ctx.accounts.mint_pda.key();
        let mint_pda_key = binding.as_ref();
        let metadata_signer_seeds: &[&[u8]] = &[b"metadata", mpl_token_metadata::ID.as_ref(), mint_pda_key, &[metadata_bump]];

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
        ).invoke_signed(&[auth_signer_seeds, metadata_signer_seeds])?;
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("---tx end counter:{}", counter.count);
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
    pub rent:Sysvar<'info, Rent>
}

#[account]
pub struct Counter {
    pub count: u64,
    pub is_initialized: bool,
}