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

#[account]
#[derive(InitSpace)]
pub struct loanInfo {
    pub loan_id: u64,
    pub borrower_pubkey: Pubkey,
    pub token_mint: Pubkey,
    pub loan_amount: u64,
    pub interest_rate: u64, 
    pub loan_start_timestamp: i64,
    pub loan_end_timestamp: i64,
    pub is_repaid: bool,
}
