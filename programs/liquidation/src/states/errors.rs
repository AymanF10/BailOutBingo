use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only admin can call this function")]
    CallableByAdmin,
    
    #[msg("Token is already whitelisted")]
    TokenAlreadyWhitelisted,
    
    #[msg("Token is not whitelisted")]
    TokenNotWhitelisted,
    
    #[msg("Loan amount must be greater than zero")]
    InvalidLoanAmount,
    
    #[msg("Loan duration must be greater than zero")]
    InvalidLoanDuration,
    
    #[msg("Interest rate is outside the allowed range")]
    InvalidInterestRate,
}
