use anchor_lang::prelude::*;


declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");


#[program]
pub mod anchor_program_example {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

/*
//Code Journal Summary:

Initial Thoughts?
    - This is absolutely mind-blowing compared to the length of the Native Rust code for the same functionality. 
    Who knew accounts checks could be this simple... WOW!

What are the concepts (borrowing, ownership, vectors etc)?
    - Bring into scope (modules and use statements), macro calls (derive, program), custom type (CheckingAccounts), 
    function parameters, Structs/fields, traits (account, program, derive), lifetimes( 'info )


What is the organization?
    - This code is very neatly organized, and easy to follow. First we bring the needed crates into scope using the 
    `use` keyword, which brings in all sub-crates of the "anchor_lang::prelude" parent crate. At a high level, the 
    code is organized/abstracted into a module called "anchor_program_example". This module contains one function "check_accounts"
    and is annotated with the "program" macro trait. The parameter passed into this function is a struct "CheckingAccounts"
    which has the derive(Accounts) macro attached. Inside the struct is 4 fields for the different accounts: signer, account to change,
    account to create, and system program. The accounts to create/change have the mutable trait attached.



What is the contract doing? What is the mechanism?

    - In terms of the functionality, this program does the same thing as it's Native counterpart, but just abstracts many things away
    to make it easier for the developer. This program performs checks on different accounts to make sure they fulfil certain requirements.
    This specific program requires 4 accounts (payer, account to create, account to change, and system program).
    If all checks pass, it returns OK(). The mechanism  by which is does this is thanks to the traits/macros like #program and #derive(Accounts).
    Many of these explicit checks that we had to make on Accounts in Native Rust are abstracted away and performed behind the scenes in Anchor.


How could it be better? More efficient? Safer?
    - This code is already so abstracted, it's hard to think of ways to make it more efficient/safer. Off the top of my head, for security, it could
    be beneficial to incorporate more detailed error messages so users/devs can understand if there's a problem with one of the accounts, the error
    can specify which specific account does not pass the checks (this is assuming custom error messages aren't already baked into Anchor. If they are,
    then Anchor should win some Awards...)

*/
