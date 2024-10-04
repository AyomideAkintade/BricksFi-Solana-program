use anchor_lang::prelude::*;

use crate::state::*;
use crate::utils::ErrorCode;

pub fn buy_asset(ctx: Context<BuyAsset>, asset_key: Pubkey, buying: u64) -> Result<()> {
    
    let asset = &mut ctx.accounts.asset;


    if asset.value_bought + buying > asset.value {
        return Err(ErrorCode::InsufficientValue.into());
    }

    let payer = &ctx.accounts.payer;
    let recipient = &ctx.accounts.recipient;

    let payer_lamports = **payer.try_borrow_lamports()?;
    msg!("Payer balance before transfer: {}", payer_lamports);

    // Get the balance of the recipient in lamports
    let recipient_lamports = **recipient.try_borrow_lamports()?;
    msg!("Recipient balance before transfer: {}", recipient_lamports);

    // Ensure the payer has enough lamports to cover the amount
    if payer_lamports < amount {
        return Err(ErrorCode::InsufficientFunds.into());
    }


    let sol_transfer = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key,
        &ctx.accounts.recipient.key,
        buying,
    );
    anchor_lang::solana_program::program::invoke(
        &sol_transfer,
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.recipient.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;


    let user = &mut ctx.accounts.user;

    if let Some(asset_index) = user.owned_assets.iter().position(|&key| key == asset_key) {
        let mut current_amount: u64 = user.ownership_amounts[asset_index];
        current_amount += buying;
        user.ownership_amounts[asset_index] = current_amount;
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
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub payer: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub asset: Account<'info, AssetState>,
    #[account(mut)]
    pub user: Account<'info, UserState>, 
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
}

