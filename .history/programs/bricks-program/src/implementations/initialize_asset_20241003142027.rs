use anchor_lang::prelude::*;

use crate::{state::*, utils};


pub fn initialize_asset(
    ctx: Context<InitializeAsset>, 
    name: [u8; 32],
    location: [u8; 64],
    attributes: Vec<KeyValue>,
    images: Vec<[u8; 128]>,
    virtual_link: [u8; 128],
    end_date_timestamp: u64,
    value: u64,
    timeline: Vec<AssetTimeline>,
) -> Result<()> {

    msg!("Initializing asset with asset_name: {:?}", name);
    msg!("End date timestamp: {}", end_date_timestamp);
    msg!("Value: {}", value);


    let key = ctx.accounts.asset_account.key;

    let asset_state = &mut ctx.accounts.asset_account;

    asset_state.key = key;
    asset_state.name = name;
    asset_state.location = location;
    asset_state.attributes = attributes;
    asset_state.images = images;
    asset_state.virtual_link = virtual_link;
    asset_state.num_owners = 0;
    asset_state.end_date_timestamp = end_date_timestamp;
    asset_state.value = value;
    asset_state.value_bought = 0;
    asset_state.timeline = timeline;

    let time: u64 = utils::current_timestamp();

    if end_date_timestamp <= time {
        return Err(utils::ErrorCode::InvalidEndDate.into());
    }


    asset_state.created_at = time;
    asset_state.updated_at = time;

    msg!("Initialized asset {:?} with {}", name, key);

    Ok(())
}


#[derive(Accounts)]
#[instruction(asset_name: [u8; 32])]
pub struct InitializeAsset<'info> {
    #[account(
        init, 
        payer = user, 
        space = 1500, 
        seeds = [b"asset", user.key().as_ref(), asset_name.as_ref()],
        bump
    )]
    pub asset_account: Account<'info, AssetState>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}