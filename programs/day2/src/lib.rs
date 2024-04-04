use anchor_lang::prelude::*;

declare_id!("HAJKvHJon8JGyrRqdiLf2Zs1czVvyKGwp2WEhrT2sjtn");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
