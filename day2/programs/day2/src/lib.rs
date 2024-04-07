use anchor_lang::prelude::*;

declare_id!("B5CzA4FsgGJC851U6FvrMipayKZ3ogdxsn4apjBAqX8i");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
