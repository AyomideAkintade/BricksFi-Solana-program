use anchor_lang::prelude::*;



#[derive(Accounts)]

pub struct InitializeAsset<'info> {
    pub asset_account: Account<'info, AssetState>
}