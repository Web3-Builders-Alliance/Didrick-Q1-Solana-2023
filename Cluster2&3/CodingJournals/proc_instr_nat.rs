use anchor_lang::prelude::*;

declare_id!("DgoL5J44aspizyUs9fcnpGEUJjWTLJRCfx8eYtUMYczf");

#[program]
pub mod processing_instructions {
    use super::*;

    // With Anchor, we just put instruction data in the function signature!
    //
    pub fn go_to_park(
        ctx: Context<Park>,
        name: String,
        height: u32,
    ) -> Result<()> {
        
        msg!("Welcome to the park, {}!", name);
        if height > 5 {
            msg!("You are tall enough to ride this ride. Congratulations.");
        } else {
            msg!("You are NOT tall enough to ride this ride. Sorry mate.");
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Park {}

/*
//Code Journal Summary:

What are the concepts (borrowing, ownership, vectors etc)?
    - The concepts used in this piece of code are modules (and organization using mod keyword), enums using the declare_id! macro, 
    structs, ownership, borrowing, lifetime (<'info> syntax), error handling using the Result type, msg! macro,traits using Context struct 
    and ProgramAccount trait (Anchor library).

What is the organization?
    - This code defines a Rust module containing a solana program ("processing_instructions"), and is annotated with the #[program] macro. 
    The module also contains a struct ("Park") annotated with the #[derive(Accounts)] macro. The "go_to_park" function takes a Context<Park> struct, 
    which provides access to the accounts and instructions necessary to run the program. This function also takes two other arguments, "name" and "height". 
    When the function is called, it outputs a welcome message, and another conditional message saying whether the client is tall enough to ride this ride.

What is the contract doing? What is the mechanism?
    - This code defines an on-chain Solana program named "processing_instructions", which contains a single method "go_to_park". For input, this method
    takes in a name and a height parameter (as well as Context<Park> struct explained above), and returns a result. The function will first print a welcome message,
    and then will print a conditional message to indicate if the given height is tall enough for the person to ride the ride. We can also notice that this program 
    has a single empty account struct, "Park". The mechanism is fairly trivial; it's simple logic that compares the height of the person with the minimum required height
    of the ride. 

How could it be better? More efficient? Safer?
    - Because of the simplicity and efficieny that Anchor provides (combined with the pretty simple logic this program is using), this piece of code already
    seems to be efficient and safe. However, one thing I can think of to possibly make this code more "safe" is to add more useful error handling messages
    and input validation. 

*/
