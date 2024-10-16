use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("65Ygpueqx7mibKjfhqeR1oBrcgF3bmp923PvEUpmeu98");

#[program]
pub mod test4 {
    use super::*;

    pub fn proxy_initialize(
        ctx: Context<ProxyInitialize>,
        sqrt_price_x64: u128,
        open_time: u64,
    ) -> Result<()> {
        instructions::proxy_initialize(ctx, sqrt_price_x64, open_time)
    }
    
}

