use anchor_lang::prelude::*;

declare_id!("aDyNPqmtjnudXDBMEFvYqqfYEVDCoNbU1ZETR26kqdu");

#[program]
pub mod access_verification {
    use super::*;

    pub fn initialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.users = Vec::new();  // Initialize an empty list of users
        Ok(())
    }

    pub fn add_user(ctx: Context<AddUser>, user: Pubkey) -> Result<()> {
        let user_data = &mut ctx.accounts.user_account;
        user_data.users.push(user); // Add the user's public key
        msg!("User added: {:?}", user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUserAccount<'info> {
    #[account(init, payer = user, space = 8 + 32 * 100)] // Space for 100 users (adjust accordingly)
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
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
