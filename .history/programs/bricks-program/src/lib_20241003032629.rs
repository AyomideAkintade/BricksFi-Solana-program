pub mod state;
pub mod utils;

mod implementations;

use crate::implementations::*;

use anchor_lang::prelude::*;
use crate::state::*;
//use borsh::{BorshSerialize, BorshDeserialize};



declare_id!("6jX84M53GKsYW5Mt7sq9CZk9WwEDQLhbmyuxXPVc7WBB");

#[program]
pub mod asset_management {

    use super::*;

    pub fn add_user(ctx: Context<AddUser>, user_id: [u8; 32],) -> Result<()> {
        implementations::add_user(ctx, user_id)
    }

    pub fn initialize_asset(
        ctx: Context<InitializeAsset>, 
        name: [u8; 32],
        location: [u8; 64],
        attributes: Vec<KeyValueStore>,
        images: Vec<[u8; 128]>,
        virtual_link: [u8; 128],
        end_date_timestamp: u64,
        value: u64,

    ) -> Result<()> {
        Ok(())
       //implementations::initialize_asset(ctx, name, location, attributes, images,  virtual_link, end_date_timestamp, value, timeline)
    }

    pub fn buy_asset(ctx: Context<BuyAsset>, asset_key: Pubkey, buying: u64) -> Result<()> {
        implementations::buy_asset(ctx, asset_key, buying)
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
