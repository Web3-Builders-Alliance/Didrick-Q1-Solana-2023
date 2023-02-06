use borsh::BorshDeserialize;
use lever::SetPowerStatus;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

entrypoint!(pull_lever);

fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                        // Our lever program's ID
        &set_power_status_instruction,                    // Passing instructions through
        vec![AccountMeta::new(power.key.clone(), false)], // Just the required account for the other program
    );

    invoke(&ix, &[power.clone()])
}

/*
//Code Journal Summary:

What are the concepts (borrowing, ownership, vectors etc)?
    - Bringing paths into scope with the `use` keyword (modules), Traits, macro calls, variable declaration and mutability,
    arrays, structs, enums, STD types (iterators/vec), referencing/borrowing, ownership, functions/arguements, error handling with "?"

What is the organization?
    - This code is organized first by bringing the crates into scope, defining the entrypoint, then creting the `pull_lever`
    function. All the main logic is inside the pull_lever function, which is the entry point to the program. Aside from the program ID,
    this function takes in the accounts and instruction_data as well, and returns the ProgramResult Enum. Inside the function,
    the account_iter and accounts are assigned to variables, as well as the SetPowerStatus object which holds the instruction data.
    Final step is the Instruction object is created and then invoked! This is the general organization of this code.

What is the contract doing? What is the mechanism?
    - This piece of code seems to be creating a new instructions using the lever program and invoking that instruction.
    More specifically, it iterates through the accounts and assigns the first two accounts to `power` and `lever_program`
    respectively. A new `SetPowerStatus` object is created that holds the instruction data, and then that is used to create
    a new `Instruction` object (along with power metadata and lever_program key). Lastly, the instruction is invoked!

    It seems like the point of this code is for a caller to be able to execute an instruction which lives in a different program
    by passing in the necessary data (accounts and instruction data) so that the Instruction object can be reconstructed, and then invoked.


How could it be better? More efficient? Safer?
    - Because this code is pretty succinct, I can't think of too many ways to improve this code...
    One thing that pops to my head is the use of `.clone()` a few times. I know that .clone() makes
    a deep copy in the heap, which is more expensive (in terms of speed and cost). Perhaps instead of creating
    deep copies, we can use references when possible, which would be more efficient in terms of speed and resource usage.

*/
