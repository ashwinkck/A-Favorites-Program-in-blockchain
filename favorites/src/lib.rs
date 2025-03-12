use anchor_lang::prelude::*;

// Our program's address! Matches the key in the target/deploy directory
declare_id!("3UTFnMiomQruvMw6VgUpfMJSxkN5ztxbBM1Xm9MHGKqR");

// Anchor programs always use 8 bytes for the discriminator
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// Our Solana program!
#[program]
pub mod favorites {
    use super::*;

    // Instruction handler: Sets the user's favorite number, color, and hobbies
    pub fn set_favorites(
        context: Context<SetFavorites>, 
        number: u64, 
        color: String, 
        hobbies: Vec<String>
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);

        // Validate hobbies length
        require!(hobbies.len() <= 5, CustomError::TooManyHobbies);
        for hobby in &hobbies {
            require!(hobby.len() <= 50, CustomError::HobbyTooLong);
        }

        msg!(
            "User {}'s favorite number is {}, favorite color is: {}",
            user_public_key, number, color
        );

        msg!("User's hobbies are: {:?}", hobbies);

        // Store values inside PDA
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

// Struct for storing user favorites inside the PDA
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)] // Max 50 bytes for color string
    pub color: String,

    #[max_len(5, 50)]  // Max 5 hobbies, each with max 50 characters
    pub hobbies: Vec<String>, 
}

// Accounts required for the `set_favorites` instruction
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

// Custom error codes
#[error_code]
pub enum CustomError {
    #[msg("A hobby exceeds 50 characters.")]
    HobbyTooLong,
    
    #[msg("You can have a maximum of 5 hobbies.")]
    TooManyHobbies,
}