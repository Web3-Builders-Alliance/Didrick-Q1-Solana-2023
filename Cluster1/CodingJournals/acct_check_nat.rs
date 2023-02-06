use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // You can verify the program ID from the instruction is in fact
    //      the program ID of your program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId);
    };

    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you
    //      try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.

    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized);
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount);
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId);
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    };

    Ok(())
}

/*
//Code Journal Summary:

What are the concepts (borrowing, ownership, vectors etc)?
    - Bringing paths into scope with the `use` keyword (modules & namespaces), macro calls, variable declaration and mutability,
    arrays, structs, iterators, constants, referencing/borrowing, ownership, control flow (with if statements), functions/arguements, methods,
    and likely some others!

What is the organization?
    - First thing we do is bring our needed crates (can be thought of as imports in other languages?) into scope.
    Then we declare the program entry point (process_instruction) to start program execution. After is the `process_instruction` function.
    We first pass in the inputs to the function and explicity define the return type, ProgramResult. We then verify
    the program ID is correct, make sure at least 4 accounts, and then iterate through the accounts and assign
    them to variables. Last step in terms of the organization in this function is making the necessary account checks
    which we cover in the question below.


What is the contract doing? What is the mechanism?
    - This program performs checks on different accounts to make sure they fulfil certain requirements.
    This specific program requires 4 accounts (payer, account to create, account to change, and system program), but
    actually performs checks on only 3 out of the 4 (doesnt perform any check on the payer account...).
    For the account to be created, it makes sure it has not yet been initialized by checking to see if it has 0 lamports.
    The account to be changed has a check to make sure it has already been initialized previously by checking to make sure it contains
    some amount of lamports. We also learned from the token program that we can only debit accounts that the program owns, so we also
    have a check to make sure the program_id matches the account to change owner. The last check makes sure the system program account
    is indeed correct (we check by making sure IDs match). If all checks pass, it return OK(). If not, it returns the corresponding error.

How could it be better? More efficient? Safer?
    - One huge thing popped out to me. This doesn't utilize the Rust concept of ownership and
    returning from a function without the semi-colon! All these if statements use "return ... ;",
    when in actuality we can delete the `return` keyword, remove the semi-colon, and this code will still
    behave the same way as if it were using `returns` and semi-colons.

    ... Actually looking back at our PaulX escrow code, I'm not sure if the above statement is accurate... I see
    that in processor.rs, we are still using `return` and semi-colons in the conditionals. My knowledge of when we can replace
    the `return` + `;` with just the line of code without the semi-colon might be flawed...

    Another change we can do to make this more efficient is bundle the logic for checking the `IncorrectProgramID`
    into one logical conditional check. It is inefficient to have to go through 3 separate conditionals to check for the
    same error. We can put it into one conditional, where that conditional checks all the cases of `IncorrectProgramID`.

    I'm also thinking there might be a way to make the iteration of accounts more efficient? Possibly by using some built in
    destructuring which could destructure the accounts in a faster way that iteration?

*/
