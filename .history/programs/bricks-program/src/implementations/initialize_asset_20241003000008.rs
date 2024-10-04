use anchor_lang::prelude::*;

use crate::{state::{AssetState, AssetTimeline}, utils};


pub fn initialize_asset(
    ctx: Context<InitializeAsset>, 
    name: [u8; 32],
    location: [u8; 64],
    attributes: Vec<(u32, u32)>,
    images: Vec<[u8; 128]>,
    virtual_link: [u8; 128],
    end_date_timestamp: u64,
    value: u64,
    timeline: Vec<AssetTimeline>,
) -> Result<()> {
    let asset_state = &mut ctx.accounts.asset_account;



    Ok(())
}


#[derive(Accounts)]
pub struct InitializeAsset<'info> {
    #[account(init, payer = user, space = 1500)]
    pub asset_account: Account<'info, AssetState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}