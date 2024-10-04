use anchor_lang::prelude::*;

use crate::state::UserState;

pub fn add_user(ctx: Context<AddUser>) -> Result<()>{
    let user_state = &mut ctx.accounts.user_account

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