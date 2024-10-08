use anchor_lang::prelude::*;

use std::str::FromStr;
use crate::state::*;
use crate::utils::ErrorCode;

pub fn convert_sol_to_lamports(amount_in_sol: f64) -> u64 {
    (amount_in_sol * 1_000_000_000.0) as u64
}

pub fn buy_asset(ctx: Context<BuyAsset>, asset_key: Pubkey, buying: f64) -> Result<()> {
    
    let asset = &mut ctx.accounts.asset;

    let buying_in_lamports = convert_sol_to_lamports(buying);

    msg!("Buying_Lamports {:?}", buying_in_lamports);

    if asset.value_bought + buying > asset.value {
        return Err(ErrorCode::InsufficientValue.into());
    }

    let payer = &ctx.accounts.payer;

    let payer_lamports = **payer.try_borrow_lamports()?;
    msg!("Payer balance before transfer: {}", payer_lamports);


    if payer_lamports < buying_in_lamports {
        return Err(ErrorCode::InsufficientFunds.into());
    }

    msg!("Recipient Recived {:?}, Recipient Allowed {:?}", {&ctx.accounts.recipient.key()}, &Pubkey::from_str("6epEHHWCeLYYqiprybDARQsXoG8cbmNDNVGHMnLy1z9t").unwrap());

    if &ctx.accounts.recipient.key() != &Pubkey::from_str("6epEHHWCeLYYqiprybDARQsXoG8cbmNDNVGHMnLy1z9t").unwrap() {
        return Err(ErrorCode::InvalidRecipient.into());        
    }


    let sol_transfer = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.payer.key,
        &ctx.accounts.recipient.key,
        buying_in_lamports,
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
        let mut current_amount: f64 = user.ownership_amounts[asset_index];
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

