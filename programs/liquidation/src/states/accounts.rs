use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Administrator {
    pub admin_pubkey: Pubkey,
    pub admin_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct WhitelistedTokenContainer {
    #[max_len(100)]
    pub whitelisted_tokens: Vec<Pubkey>,
    pub whitelisted_tokens_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct StakerUserInfo {
    pub user_pubkey: Pubkey,
    pub token_mint: Pubkey,
    pub staker_amount: u64,
    pub staker_timestamp: i64,
}
