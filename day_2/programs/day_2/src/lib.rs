use anchor_lang::prelude::*;

declare_id!("2dpricY6iQ7B5B65SDp5erm8psr1gqeo6Qz96ZDDKt2V");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        let sub_result = a - b;
        let add_result = a + b;
        let mul_result = a * b;
        let div_result = a / b;

        msg!("You said {:?}", message);
        msg!("You sent {} {}", a, b);

        msg!("Value of calculations {}", sub_result);
        msg!("Value of calculations {}", add_result);
        msg!("Value of calculations {}", mul_result);
        msg!("Value of calculations {}", div_result);

        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
