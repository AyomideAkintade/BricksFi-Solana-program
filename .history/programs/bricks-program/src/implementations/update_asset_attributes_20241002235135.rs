use anchor_lang::prelude::*;
use crate::state::AssetTimeline;

pub fn update_asset_timeline(ctx: Context<UpdateAssetTImeline>, timeline: Vec<AssetTimeline>) -> Result<()> {
    Ok(())
}


#[derive(Accounts)]
pub struct UpdateAssetTImeline<'info> {

}