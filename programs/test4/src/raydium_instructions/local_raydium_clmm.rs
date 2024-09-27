use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use anchor_lang::solana_program::{
    program::invoke,
    program::invoke_signed,
};

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
    ctx: Context<'_, '_, '_, 'info, CallCreatePool<'info>>,
    sqrt_price_x64: u128,
    open_time: u64,
    token_mint0: AccountInfo<'info>,
    token_mint1: AccountInfo<'info>,
    token_vault0: AccountInfo<'info>,
    token_vault1: AccountInfo<'info>,
    
) -> Result<()> {
    // 定义 createPool 调用所需的账户元数据
    let accounts = vec![
        AccountMeta::new(ctx.accounts.pool_creator.key(), true),
        AccountMeta::new_readonly(ctx.accounts.amm_config.key(), false),
        AccountMeta::new(ctx.accounts.pool_state.key(), false),
        AccountMeta::new_readonly(token_mint0.key(), false),
        AccountMeta::new_readonly(token_mint1.key(), false),
        AccountMeta::new(token_vault0.key(), false),
        AccountMeta::new(token_vault1.key(), false),
        AccountMeta::new(ctx.accounts.observation_state.key(), false),
        AccountMeta::new(ctx.accounts.tick_array_bitmap.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program0.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program1.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
    ];

    // 构建 createPool 的 CPI 调用
    let create_pool_ix = Instruction {
        program_id: ctx.accounts.raydium_clmm_program.key(),
        accounts,
        data: local_raydium_instruction::CreatePool {
            sqrt_price_x64,
            open_time,
        }.data(),
    };

    // CPI 调用
    invoke(
        &create_pool_ix,
        &[
            ctx.accounts.pool_creator.to_account_info(),
            ctx.accounts.amm_config.to_account_info(),
            ctx.accounts.pool_state.to_account_info(),
            token_mint0.to_account_info(),
            token_mint1.to_account_info(),
            token_vault0.to_account_info(),
            token_vault1.to_account_info(),
            ctx.accounts.observation_state.to_account_info(),
            ctx.accounts.tick_array_bitmap.to_account_info(),
            ctx.accounts.token_program0.to_account_info(),
            ctx.accounts.token_program1.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.raydium_clmm_program.to_account_info(),
        ]
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct CallCreatePool<'info> {
    /// CPI 调用账户配置
    #[account(mut)]
    pub pool_creator: AccountInfo<'info>,
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    pub pool_state: AccountInfo<'info>,
    pub token_mint0: AccountInfo<'info>,
    pub token_mint1: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault0: AccountInfo<'info>,
    #[account(mut)]
    pub token_vault1: AccountInfo<'info>,
    #[account(mut)]
    pub observation_state: AccountInfo<'info>,
    #[account(mut)]
    pub tick_array_bitmap: AccountInfo<'info>,
    pub token_program0: AccountInfo<'info>,
    pub token_program1: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// 外部 Raydium 程序 ID
    #[account(address = local_raydium_clmm::ID)]
    pub raydium_clmm_program: AccountInfo<'info>, 

}

