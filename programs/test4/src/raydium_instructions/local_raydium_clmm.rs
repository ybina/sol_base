// use anchor_lang::prelude::*;
// use solana_program::instruction::Instruction;
// use anchor_lang::solana_program::program::invoke_signed;

// pub mod local_raydium_clmm {
//     anchor_lang::declare_id!("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");
// }

// pub mod local_raydium_instruction {
//     use borsh::{BorshDeserialize, BorshSerialize};

//     #[derive(BorshSerialize, BorshDeserialize)]
//     pub struct CreatePool {
//         pub sqrt_price_x64: u128,
//         pub open_time: u64,
//     }

//     impl CreatePool {
//         pub fn data(&self) -> Vec<u8> {
//             let mut data = vec![];
//             self.serialize(&mut data).unwrap();
//             data
//         }
//     }
// }

// pub fn cpi_create_pool<'info>(
//     sqrt_price_x64: u128,
//     open_time: u64,
//     pool_creator:AccountInfo<'info>,
//     amm_config:AccountInfo<'info>,
//     token_mint0: AccountInfo<'info>,
//     token_mint1: AccountInfo<'info>,
//     token_program0: AccountInfo<'info>,
//     token_program1: AccountInfo<'info>,
//     auth_pda_seeds: &[&[u8]],
// ) -> Result<()> {
//     //  createPool accounts
//     let accounts = vec![
//         AccountMeta::new(pool_creator.key(), true),
//         AccountMeta::new_readonly(amm_config.key(), false),
//         AccountMeta::new_readonly(token_mint0.key(), false),
//         AccountMeta::new_readonly(token_mint1.key(), false),
//         AccountMeta::new_readonly(token_program0.key(), false),
//         AccountMeta::new_readonly(token_program1.key(), false),
//         // AccountMeta::new_readonly(cpi_accounts.system_program.key(), false),
//         // AccountMeta::new_readonly(cpi_accounts.rent.key(), false),
//     ];

//     // createPool CPI call
//     let create_pool_ix = Instruction {
//         program_id: local_raydium_clmm::ID,
//         accounts,
//         data: local_raydium_instruction::CreatePool {
//             sqrt_price_x64,
//             open_time,
//         }.data(),
//     };

//     invoke_signed(
//         &create_pool_ix, 
//         &[
//             pool_creator.clone(),
//             amm_config.clone(),
//             token_mint0.clone(),
//             token_mint1.clone(),
//             token_program0.clone(),
//             token_program1.clone(),
//         ], 
//         &[auth_pda_seeds],
//     )?;
//     Ok(())
// }

