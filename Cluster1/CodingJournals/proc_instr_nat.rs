use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Attempt to serialize the BPF format to our struct
    //  using Borsh
    //
    let instruction_data_object = InstructionData::try_from_slice(&instruction_data)?;

    msg!("Welcome to the park, {}!", instruction_data_object.name);
    if instruction_data_object.height > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    name: String,
    height: u32,
}

/*
//Code Journal Summary:

What are the concepts (borrowing, ownership, vectors etc)?
    - Similar to the other coding journals for Native Rust, the concepts are: bringing paths into scope with the `use` keyword (modules),
    Traits (derive...), macro calls (entrypoints!, msg!), functions/function arguements, referencing/borrowing, ownership, Enums (ProgramResult),
    arrays, structs (InstructionData), conditional logic flow (if/else)

What is the organization?
    - This code is organized first by bringing the crates into scope, defining the entrypoint, then building out the process_instruction
    function. Main logic is inside the process_instruction function, also the entry point to the program. Aside from the program ID,
    this function takes in the accounts and instruction_data as well, and returns the ProgramResult Enum. Inside the function,
    the InstructionData object is created using the passed in `instruction_data` and assigned to a variable. The logical checks are performed
    for the height, and the corresponding messages are returned. After the function definition, we also ssee the InstructionData struct defined
    at the end. It is defined with the `name` and `height` attributes, and has the `derive` trait which gives it extended/shared behavior/functionality.

What is the contract doing? What is the mechanism?
    - This piece of code is converting the binary instruction data into an `InstructionData` object using the Borsch (de)serialization.
    This `InstructionData` object's name is used for the Welcome message, and then the height attribute is then checked. If the height
    is greater than 5, a message is printed that "you are tall enough to ride". If the height is 5 or less, the "NOT tall enough" message is printed.

How could it be better? More efficient? Safer?
    - One improvement that can be made is reducing the number of calls to "msg!". Right now it's two calls, but if we utilized some
    sort of string concatentation, we can just have one call to "msg!". Another improvement (I think it would be an improvement, but
    I'm not 100% sure, it might just be another way of doing things) is instead of using the if-else logic which introduces two separate
    branches, we can make use of conditional expressions (control flow with "if let") to perform the checking of the height.

*/
