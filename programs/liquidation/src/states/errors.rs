use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only admin can call this function")]
    CallableByAdmin,
}
