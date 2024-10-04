use anchor_lang::prelude::*;

pub fn add_user(ctx: Context<AddUser>) -> Result<()>{
    let user_state = ctx.accounts.user_account.load();

    
}


#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(init, payer = user, space = 8 + UserState::MAX_SIZE)]
    pub user_account: Account<'info, UserState>, 
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl UserState {
    pub const MAX_SIZE: usize = 32 
        + 32 
        + 10 * 32
        + 10 * 8;
}