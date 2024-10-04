use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("aDyNPqmtjnudXDBMEFvYqqfYEVDCoNbU1ZETR26kqdu");

#[derive(Clone)]
pub struct AssetTImeline {
    pub title: String,
    pub date: u64,
    pub description: String
}

#[account]
pub struct Asset {
    pub name: String,
    pub location: String,
    pub attributes: Vec<(String, String)>,
    pub images: Vec<String>,
    pub virtual_link: String,
    pub owners: Vec<User>,
    pub total_owners: i32,
    pub end_date: u64,
    pub amount: u64,
    pub amount_bought: u64,
    pub timeline: Vec<AssetTImeline>
}

#[derive(Accounts)]
pub struct AddAsset<'info> {
    #[account(init, payer = user, space = 8 + Asset::MAX_SIZE, seeds = [b"asset".as_ref(), asset_pubkey.key().as_ref()], bump)] // Make the asset unique using seeds
    pub asset: Account<'info, Asset>,     // The asset account
    #[account(mut)]
    pub user: Signer<'info>,              // The user who signs the transaction
    pub system_program: Program<'info, System>, // System program
}


#[program]
pub mod asset_management {
    use super::*;

    pub fn add_asset(
        ctx: Context<AddAsset>,
        name: String,
        location: String,
        attributes: Vec<(String, String)>,
        images: Vec<String>,
        virtual_link: String,
        total_owners: i32,
        end_date: u64,
        amount: u64,
    ) -> Result<()> {
        let asset = &mut ctx.accounts.asset;

        asset.name = name;
        asset.location = location;
        asset.attributes = attributes;
        asset.images = images;
        asset.virtual_link = virtual_link;
        asset.owners = Vec::new();
        asset.total_owners = total_owners;
        asset.end_date = end_date;
        asset.amount = amount;
        asset.amount_bought = 0;
        asset.timeline = Vec::new();

        msg!("Asset {} added with location: {}, total amount: {}", asset.name, asset.location, asset.amount);

        Ok(())
    }

    pub fn fetch_asset(ctx: Context<FetchAsset>, asset_pubkey: Pubkey) -> Result<Asset> {
        let asset = &ctx.accounts.asset;

        // Return the asset data
        Ok(asset.clone())
    }

    pub fn add_user(
        ctx: Context<AddUser>,
        email: String,          // User's unique email
        ownership_amount: u64,  // Amount of ownership they are buying
    ) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
    
        // Check if the user already exists
        if let Some(user) = asset.owners.iter_mut().find(|u| u.email == email) {
            // If user exists, update ownership
            user.ownership += ownership_amount;
            msg!("Updated ownership for user: {} with new ownership: {}", email, user.ownership);
        } else {
            // If user does not exist, add a new one
            let new_user = User {
                email: email.clone(),
                ownership: ownership_amount,
            };
            asset.owners.push(new_user);
            msg!("Added new user: {} with ownership: {}", email, ownership_amount);
        }
    
        Ok(())
    }


}

#[derive(Clone)]
pub struct User {
    pub email: String,    // Unique email identifier for the user
    pub ownership: u64,   // Ownership percentage or amount
}