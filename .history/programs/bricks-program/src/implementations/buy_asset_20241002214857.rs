use anchor_lang::{prelude::*, system_program};

use crate::state::*;

pub fn buy_asset(ctx: Context<BuyAsset>, asset_key: Pubkey, buying: u64) -> Result<()> {
    let asset = &mut ctx.accounts.asset;
    
    asset.value_bought += buying;
    asset.num_owners += 1;

    let user = &mut ctx.accounts.user;
    user.owned_assets.push(asset_key);
    user.ownership_amounts.push(buying);



    Ok(())
}

#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub asset: Account<'info, AssetState>,
    #[account(mut)]
    pub user: Account<'info, UserState>, 
    pub system_program: Program<'info, System>,
}

