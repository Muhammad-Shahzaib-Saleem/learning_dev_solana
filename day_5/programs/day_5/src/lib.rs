use anchor_lang::prelude::*;

declare_id!("JBaKPeGdDnddc1KyopkKxYN8i7Pw7qfkm2kHWQfRSzW4");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
