use anchor_lang::prelude::*;

declare_id!("aDyNPqmtjnudXDBMEFvYqqfYEVDCoNbU1ZETR26kqdu");

#[program]
pub mod access_verification {
    use super::*;

    pub fn add_user(ctx: Context<AddUser>, user: Pubkey) -> Result<()> {
        let user_data = &mut ctx.accounts.user_account;
        user_data.users.push(user); // Add the user's public key
        msg!("User added: {:?}", user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,  // Account that stores the list of users
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub users: Vec<Pubkey>, // A list of authorized users
}
