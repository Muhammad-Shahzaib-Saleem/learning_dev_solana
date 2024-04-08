use anchor_lang::prelude::*;

declare_id!("2L6VsThq74Qe33NTBUbrf14imsJGoJrWGQp5YzJa7JaW");

#[program]
pub mod day_3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    //Fn for addition
    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    //fn for subtraction
    pub fn sub(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }

    //fn for multiplication
    pub fn mul(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let multiplication = a * b;
        msg!("Multiplication is {}", multiplication);

        Ok(())
    }

    //fn for division
    pub fn div(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let division = a / b;
        msg!("Division is {}", division);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
