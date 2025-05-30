//Accounts: Making and storing details of an account of a project

use anchor_lang::prelude::*;

//Administrator Account
#[account]
#[derive(InitSpace)]  // initspace calculates the space required for an account to be created 
pub struct Administrator {
    pub admin_pubkey: Pubkey,
    pub admin_bump: u8,
}

// Account for the whitelisted tokens (store and track)
#[account]
#[derive(InitSpace)]
pub struct WhitelistedTokenContainer {
    #[max_len(100)]
    pub whitelisted_tokens: Vec<Pubkey>,
    pub whitelisted_tokens_bump: u8,
}

// Staker user info
#[account]
#[derive(InitSpace)]
pub struct StakerUserInfo {
    pub user_pubkey: Pubkey,
    pub token_mint: Pubkey,
    pub staker_amount: u64,
    pub staker_timestamp: i64,
}

// Staked user info container



