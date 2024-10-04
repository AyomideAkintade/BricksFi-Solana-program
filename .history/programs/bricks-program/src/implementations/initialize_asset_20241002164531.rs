use anchor_lang::prelude::*;


#[derive(Accounts)]

pub struct InitializeAsset<'info> {
    #[account(init, payer = user, space = 1500)]
    pub asset_account: Account<'info, AssetState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}