use anchor_lang::prelude::*;
use crate::states::*;
use crate::states::errors::ErrorCode;

pub fn request_loan(ctx: Context<LoanRequests>, token_mint: Pubkey, loan_amount: u64, interest_rate: u64, loan_duration: i64) -> Result<()> {
    let loan_info = &mut ctx.accounts.loan_account;
    let whitelisted_tokens = &ctx.accounts.whitelisted_token_container;
    
    // Check if token is whitelisted
    require!(
        whitelisted_tokens.whitelisted_tokens.contains(&token_mint),
        ErrorCode::TokenNotWhitelisted
    );
    
    // Validate loan parameters
    require!(loan_amount > 0, ErrorCode::InvalidLoanAmount);
    require!(loan_duration > 0, ErrorCode::InvalidLoanDuration);
    require!(interest_rate <= 10000, ErrorCode::InvalidInterestRate);
    
    // Set loan information
    loan_info.loan_id = Clock::get()?.unix_timestamp as u64; // Use timestamp as unique ID
    loan_info.borrower_pubkey = ctx.accounts.caller.key();
    loan_info.token_mint = token_mint;
    loan_info.loan_amount = loan_amount;
    loan_info.interest_rate = interest_rate;
    loan_info.loan_start_timestamp = Clock::get()?.unix_timestamp;
    loan_info.loan_end_timestamp = Clock::get()?.unix_timestamp + loan_duration;
    loan_info.is_repaid = false;
    
    Ok(())
}

// Event emitted when a loan is requested
#[event]
pub struct LoanRequestEvent {
    pub loan_id: u64,
    pub borrower: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub interest_rate: u64,
    pub start_time: i64,
    pub end_time: i64,
} 