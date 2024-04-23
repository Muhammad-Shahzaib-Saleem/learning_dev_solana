use anchor_lang::prelude::*;

declare_id!("7eDz2gxjRcJsKWh9stx8jyYUQTwMRZmDDM22DnpLsbQm");

#[program]
pub mod day_6 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
