use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;

pub mod local_raydium_clmm {
    anchor_lang::declare_id!("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH"); // 外部程序的 Program ID
}

pub mod local_raydium_instruction {
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(BorshSerialize, BorshDeserialize)]
    pub struct CreatePool {
        pub sqrt_price_x64: u128,
        pub open_time: u64,
    }

    impl CreatePool {
        pub fn data(&self) -> Vec<u8> {
            let mut data = vec![];
            self.serialize(&mut data).unwrap();
            data
        }
    }
}

pub fn cpi_create_pool<'info>(
    cpi_accounts: CallCreatePool<'info>,
    sqrt_price_x64: u128,
    open_time: u64,
    token_mint0: AccountInfo<'info>,
    token_mint1: AccountInfo<'info>,
    token_vault0: AccountInfo<'info>,
    token_vault1: AccountInfo<'info>,
    // token_pda_seeds: &[&[u8]],
    auth_pda_seeds: &[&[u8]]
    
) -> Result<()> {
    //  createPool accounts
    let accounts = vec![
        AccountMeta::new(cpi_accounts.pool_creator.key(), true),
        AccountMeta::new_readonly(cpi_accounts.amm_config.key(), false),
        AccountMeta::new(cpi_accounts.pool_state.key(), false),
        AccountMeta::new_readonly(token_mint0.key(), false),
        AccountMeta::new_readonly(token_mint1.key(), false),
        AccountMeta::new(token_vault0.key(), false),
        AccountMeta::new(token_vault1.key(), false),
        AccountMeta::new(cpi_accounts.observation_state.key(), false),
        AccountMeta::new(cpi_accounts.tick_array_bitmap.key(), false),
        AccountMeta::new_readonly(cpi_accounts.token_program0.key(), false),
        AccountMeta::new_readonly(cpi_accounts.token_program1.key(), false),
        AccountMeta::new_readonly(cpi_accounts.system_program.key(), false),
        AccountMeta::new_readonly(cpi_accounts.rent.key(), false),
    ];

    // createPool CPI call
    let create_pool_ix = Instruction {
        program_id: cpi_accounts.raydium_clmm_program.key(),
        accounts,
        data: local_raydium_instruction::CreatePool {
            sqrt_price_x64,
            open_time,
        }.data(),
    };

    invoke_signed(
        &create_pool_ix, 
        &[
            cpi_accounts.pool_creator.to_account_info(),
            cpi_accounts.amm_config.to_account_info(),
            cpi_accounts.pool_state.to_account_info(),
            token_mint0.to_account_info(),
            token_mint1.to_account_info(),
            token_vault0.to_account_info(),
            token_vault1.to_account_info(),
            cpi_accounts.observation_state.to_account_info(),
            cpi_accounts.tick_array_bitmap.to_account_info(),
            cpi_accounts.token_program0.to_account_info(),
            cpi_accounts.token_program1.to_account_info(),
            cpi_accounts.system_program.to_account_info(),
            cpi_accounts.rent.to_account_info(),
            cpi_accounts.raydium_clmm_program.to_account_info(),
        ], 
        &[auth_pda_seeds],
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CallCreatePool<'info> {
    /// CPI call Accounts
    ///CHECK:
    #[account(mut)]
    pub pool_creator: AccountInfo<'info>,
    ///CHECK:
    pub amm_config: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    ///CHECK:
    pub token_mint0: AccountInfo<'info>,
    ///CHECK:
    pub token_mint1: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_vault0: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_vault1: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub observation_state: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    pub tick_array_bitmap: AccountInfo<'info>,
    ///CHECK:
    pub token_program0: AccountInfo<'info>,
    ///CHECK:
    pub token_program1: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    ///CHECK:  Raydium Program ID
    #[account(address = local_raydium_clmm::ID)]
    pub raydium_clmm_program: AccountInfo<'info>, 
}

