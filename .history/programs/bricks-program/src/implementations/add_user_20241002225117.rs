use anchor_lang::prelude::*;

use crate::state::UserState;

pub fn add_user(ctx: Context<AddUser>, user_id: [u8; 32]) -> Result<()>{
    let user_state = &mut ctx.accounts.user_account;

    if user_state.key != Pubkey::default() {
        return Err(ErrorCode::UserAlreadyExists.into());
    }

    user_state.key = ctx.accounts.user.key(); // Set the key to the user's public key
    user_state.id = user_id; // Set the user ID (this could be a hash of the user's email or another identifier)
    user_state.owned_assets = Vec::new(); // Initialize empty asset ownership vector
    user_state.ownership_amounts = Vec::new(); // Initialize empty ownership amounts vector

    msg!("User with key {} added", user_state.key);

    Ok(())
}


#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(init, payer = user, space = 8 + UserState::MAX_SIZE)]
    pub user_account: Account<'info, UserState>, 
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
