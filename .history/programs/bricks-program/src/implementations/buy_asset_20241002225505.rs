use anchor_lang::{prelude::*, system_program};

use crate::state::*;
use crate::utils::ErrorCode;

pub fn buy_asset(ctx: Context<BuyAsset>, asset_key: Pubkey, buying: u64) -> Result<()> {
    
    let asset = &mut ctx.accounts.asset;


    if asset.value_bought + buying > asset.value {
        return Err(ErrorCode::InsufficientValue.into());
    }

    let sol_transfer = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key,
        &ctx.accounts.recipient.key,
        buying,
    );
    anchor_lang::solana_program::program::invoke(
        &sol_transfer,
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.recipient.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    let user = &mut ctx.accounts.user;

    if let Some(asset_index) = user.owned_assets.iter().position(|&key| key == asset_key) {
        user.ownership_amounts[asset_index] += buying;
    } else {
        user.owned_assets.push(asset_key);
        user.ownership_amounts.push(buying);
        
        asset.num_owners += 1;
    }
    
    asset.value_bought += buying;



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

