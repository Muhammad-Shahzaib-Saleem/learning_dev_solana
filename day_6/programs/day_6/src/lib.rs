use anchor_lang::prelude::*;

declare_id!("7eDz2gxjRcJsKWh9stx8jyYUQTwMRZmDDM22DnpLsbQm");

#[program]
pub mod day_6 {

    use super::*;

    use std::collections::HashMap;

    pub fn initialize(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        struct Person {
            my_name: String,
            my_age: u64,
        }

        let mut person1 = Person {
            my_name: name,
            my_age: age,
        };

        msg!("Name {} and age {}", person1.my_name, person1.my_age);

        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("Name {} and age {}", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn age_checker(_ctx: Context<Initialize>, _age: u64) -> Result<()> {
        //If statement assiging to variable

        // let result = if _age >= 18 {
        //     msg!("You are 18 year old or above")
        // } else {
        //     msg!("You are below 18 years old")
        // };

        //If-else statement
        // if _age >= 18 {
        //     msg!("You are 18 year old or above");
        // } else {
        //     msg!("You are below 18 years old");
        // }

        //control flow construct match

        match _age {
            1 => {
                msg!("Age is 1");
            }
            2 | 3 => {
                msg!("The is age 2 or 3");
            }
            4..=6 => {
                msg!("The age is between 4 to 6");
            }
            _ => {
                msg!("The age is something else");
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
