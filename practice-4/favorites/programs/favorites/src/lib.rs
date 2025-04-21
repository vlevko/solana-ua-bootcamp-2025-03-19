use anchor_lang::prelude::*;

// Our program's address! You don't need to change it.
// This matches the private key in the target/deploy directory
declare_id!("2g6ian786fDjXe83mWTRUBmXASDJbsdJtnDe51RpMEzo");

// Anchor programs always use
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// What we will put inside the Favorites PDA
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
}

// When people call the set_favorites instruction, they will need to provide the accounts that will
// be modified. This keeps Solana fast!
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,
}

// Our Solana program!
#[program]
pub mod favorites {
    use super::*;

    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context
            .accounts
            .favorites
            .set_inner(Favorites { number, color });
        Ok(())
    }

    // Our second instruction handler! It updates the user's favorite number and/or color
    pub fn update_favorites(context: Context<UpdateFavorites>, new_number: Option<u64>, new_color: Option<String>) -> Result<()> {
        let favorites = &mut context.accounts.favorites;

        if let Some(number) = new_number {
            msg!("Updating number to {}", number);
            favorites.number = number;
        }

        if let Some(color) = new_color {
            msg!("Updating color to {}", color);
            favorites.color = color;
        }

        Ok(())
    }

    // We can also add a get_favorites instruction to get the user's favorite number and color
}
