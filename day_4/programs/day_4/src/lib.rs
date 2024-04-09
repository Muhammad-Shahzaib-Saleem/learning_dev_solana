use anchor_lang::prelude::*;

declare_id!("2koyqKZzmow5SpAPmvUzxabDh1PbW5Kt2AkcEZAiSgkS");

#[program]
pub mod day_4 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
