use anchor_lang::prelude::*;

declare_id!("9nZJMqhVh4oHpLgzQJvnwguyy93KkWKyB2rK82RdSr3R");

#[program]
pub mod day3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
