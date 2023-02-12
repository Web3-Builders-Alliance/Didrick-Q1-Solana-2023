use anchor_lang::prelude::*;
use lever::cpi::accounts::SetPowerStatus;
use lever::program::Lever;
use lever::{self, PowerStatus};


declare_id!("ABoYG2GWbzLgnnGhK2pUGNupzKoYe7UGk2idrAXbstAS");


#[program]
mod hand {
    use super::*;
    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> anchor_lang::Result<()> {
        
        // Hitting the switch_power method on the lever program
        //
        lever::cpi::switch_power(
            CpiContext::new(
                
                ctx.accounts.lever_program.to_account_info(), 

                // Using the accounts context struct from the lever program
                //
                let cpi_accounts = SetPowerStatus {
                    power: ctx.accounts.power.to_account_info(),
                };
            ), 
            name
        )
    }
}


#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
    pub lever_program: Program<'info, Lever>,
}

/*
//Code Journal Summary:

What are the concepts (borrowing, ownership, vectors etc)?
    - Bringing paths into scope with the `use` keyword (modules), Traits, macro calls, variable declaration and mutability, 
    structs with derivate attributes, binding with "let", Contexts with Accounts, use of external programs/libraries.

What is the organization?
    - First we bring the needed crates into scope using the `use` keyword (anchor_lang, but also the lever programs/crates).
    After we declare the program ID, we see there is a main module "hand", and a "PullLever" struct which is annotated with
    the accounts derived macro trait. This struct contains two fields, the power (a mutable account) and a program. 
    The "hand" module defines a function "pull_lever" which utilizes the "switch_power" method from the "lever::cpi" module. 

What is the contract doing? What is the mechanism?
    - After bringing in the necessary crates/modules into scope, this contract then declares the unique identifier/program ID. 
    The "hand" module is defined and implemented as a program using the #[program] trait. The "pull_lever" function defined takes
    in two parameters, a PullLever struct and a string name. It then uses these two arguements to create a new CPIContext object
    constructed using the lever_program and power (which is passed into SetPowerStatus then assigned to the var "cpi_accounts")
    from the PullLever struct, and name. The PullLever struct is given the Accounts derived trait so it can be used as an account,
    and contains a mutable account (power), and an immutable program (lever_program).

How could it be better? More efficient? Safer?
    - I can't think of how this piece of code can be more efficient, but for safety/security I think this piece of code can be 
    improved by adding error messages/error handling for potential exceptions that might arise when executing certain methods that
    are passed in, like the "lever::cpi::switch_power" method, SetPowerStatus, or CpiContext::new. 

*/
