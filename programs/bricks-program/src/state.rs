use anchor_lang::prelude::*;

#[account]
pub struct AssetState {
    pub key: Pubkey,
    pub id: [u8; 16],
    pub name: [u8; 32],
    pub location: [u8; 64],
    pub attributes: Vec<KeyValue>,
    pub images: Vec<[u8; 128]>,
    pub virtual_link: [u8; 128],
    pub num_owners: u16,
    pub end_date_timestamp: u64,
    pub value: f64,
    pub value_bought: f64,
    pub timeline: Vec<AssetTimeline>,
    pub created_at: u64,
    pub updated_at: u64
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AssetTimeline {
    pub title: [u8; 32],
    pub timestamp: u64,
    pub description: [u8; 128],
}


// impl Default for AssetState {
//     fn default() -> AssetState {
//         AssetState {
//             key: Pubkey::default(),
//             name: String::new(),
//             location: String::new(),
//             attributes: Vec::new(),
//             images: Vec::new(),
//             virtual_link: String::new(),
//             num_owners: 0,
//             end_date_timestamp: 0,
//             value: 0,
//             value_bought: 0,
//             timeline: Vec::new(),
//             created_at: todo!(),
//             updated_at: todo!(),
//         }
//     }
// }

#[account]
pub struct UserState {
    pub key: Pubkey,
    pub id: [u8; 16],
    pub owned_assets: Vec<Pubkey>, 
    pub ownership_amounts: Vec<f64>
}


impl UserState {
    pub const MAX_SIZE: usize = 32 
        + 32 
        + 10 * 32
        + 10 * 8;
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct KeyValue {
    pub key: [u8; 32],  // fixed-size key, e.g., "color"
    pub value: [u8; 32],  // fixed-size value, e.g., "red"
}
