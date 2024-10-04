use anchor_lang::prelude::*;

use crate::state::UserState;
use crate::utils::ErrorCode;

pub fn add_user(ctx: Context<AddUser>, user_id: [u8; 16]) -> Result<()>{
    
    let (user_account, _bump) = Pubkey::find_program_address(
        &[b"user", ctx.accounts.user.key().as_ref(), user_id.as_ref()], 
        ctx.program_id,
    );

    if &ctx.accounts.user_account.key() != &user_account {
        return Err(ErrorCode::InvalidAccount.into());
    }

    let key = user_account;
    
    let user_state = &mut ctx.accounts.user_account;

    if user_state.key != Pubkey::default() {
        return Err(ErrorCode::UserAlreadyExists.into());
    }

    user_state.key = key;
    user_state.id = user_id;
    user_state.owned_assets = Vec::new();
    user_state.ownership_amounts = Vec::new();

    msg!("User with key {} added", user_state.key);

    Ok(())
}


#[derive(Accounts)]
#[instruction(user_id: [u8; 16])]
pub struct AddUser<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + UserState::MAX_SIZE, 
        seeds = [b"user", user.key().as_ref(), user_id.as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserState>, 
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
