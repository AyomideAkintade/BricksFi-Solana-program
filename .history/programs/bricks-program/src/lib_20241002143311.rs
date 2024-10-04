pub mod state;

mod implementations;

use crate::implementations::*;

use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};



declare_id!("aDyNPqmtjnudXDBMEFvYqqfYEVDCoNbU1ZETR26kqdu");

#[program]
pub mod asset_management {
    use super::*;

    // pub fn add_asset(
    //     ctx: Context<AddAsset>,
    //     name: String,
    //     location: String,
    //     attributes: Vec<(String, String)>,
    //     images: Vec<String>,
    //     virtual_link: String,
    //     total_owners: i32,
    //     end_date: u64,
    //     amount: u64,
    // ) -> Result<()> {
    //     let asset = &mut ctx.accounts.asset;

    //     asset.name = name;
    //     asset.location = location;
    //     asset.attributes = attributes;
    //     asset.images = images;
    //     asset.virtual_link = virtual_link;
    //     asset.owners = Vec::new();
    //     asset.total_owners = total_owners;
    //     asset.end_date = end_date;
    //     asset.amount = amount;
    //     asset.amount_bought = 0;
    //     asset.timeline = Vec::new();

    //     msg!("Asset {} added with location: {}, total amount: {}", asset.name, asset.location, asset.amount);

    //     Ok(())
    // }

    // pub fn fetch_asset(ctx: Context<FetchAsset>) -> Result<()> {
    //     let asset = &ctx.accounts.asset;

    //     Ok(())
    // }

    pub fn add_user(ctx: Context<AddUser>, user_id: [u8; 32],) -> Result<()> {
        implementations::add_user(ctx, &user_id)
    }
}

// #[account]
// pub struct Asset {
//     pub name: String,
//     pub location: String,
//     pub attributes: Vec<(String, String)>,
//     pub images: Vec<String>,
//     pub virtual_link: String,
//     pub owners: Vec<User>,
//     pub total_owners: i32,
//     pub end_date: u64,
//     pub amount: u64,
//     pub amount_bought: u64,
//     pub timeline: Vec<AssetTimeline>,
// }

// #[derive(Clone, BorshSerialize, BorshDeserialize)] // Derive Borsh traits
// pub struct AssetTimeline {
//     pub title: String,
//     pub date: u64,
//     pub description: String,
// }

// #[derive(Accounts)]
// pub struct AddAsset<'info> {
//     #[account(init, payer = user, space = Asset::LEN)] // Use Asset::LEN for space allocation
//     pub asset: Account<'info, Asset>,  // The asset account
//     #[account(mut)]
//     pub user: Signer<'info>,  // The user who signs the transaction
//     pub system_program: Program<'info, System>, // System program
// }

// #[derive(Accounts)]
// pub struct FetchAsset<'info> {
//     #[account(mut)]  // Make sure to allow mutable access to the asset
//     pub asset: Account<'info, Asset>,  // The asset account to fetch
// }

// #[derive(Accounts)]
// pub struct AddUser<'info> {
//     #[account(mut)]
//     pub asset: Account<'info, Asset>,  // The asset account to which users are being added
//     pub system_program: Program<'info, System>,
// }

// #[derive(Clone, BorshSerialize, BorshDeserialize)] // Derive Borsh traits
// pub struct User {
//     pub email: String,    // Unique email identifier for the user
//     pub ownership: u64,   // Ownership percentage or amount
// }

// // Implement a constant for the size of the Asset account
// impl Asset {
//     const LEN: usize = 8 +   // Discriminator
//                       4 +  // Name (length)
//                       32 +  // Name (maximum size)
//                       4 +  // Location (length)
//                       32 +  // Location (maximum size)
//                       4 +  // Attributes (length)
//                       4 * 32 * 2 +  // Maximum attributes (assuming max 32 length for both key and value)
//                       4 +  // Images (length)
//                       4 * 64 +  // Maximum images (assuming max 64 length)
//                       4 +  // Virtual link (length)
//                       64 +  // Virtual link (maximum size)
//                       4 +  // Total owners
//                       8 +  // End date
//                       8 +  // Amount
//                       8 +  // Amount bought
//                       4 +  // Timeline length
//                       4 * (8 + 4 + 64); // Each AssetTimeline (size of title, date, description)
// }
