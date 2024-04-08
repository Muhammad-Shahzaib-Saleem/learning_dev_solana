use anchor_lang::prelude::*;

declare_id!("2L6VsThq74Qe33NTBUbrf14imsJGoJrWGQp5YzJa7JaW");

#[program]
pub mod day_3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
