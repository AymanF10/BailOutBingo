use anchor_lang::prelude::*;
mod states;
mod instructions;
use states::*;
use instructions::*;

declare_id!("A6vAPiZS3Y5y7PQGvHE2TaqHKZZgYPV1VdJufp56yjx7");

#[program]
pub mod liquidation {
    use super::*;

    pub fn initialize_admin(ctx: Context<AdministratorInit>, admin_pubkey: Pubkey) -> Result<()> {
        admin_initialize(ctx, admin_pubkey)?;
        Ok(())
    }

    #[access_control(WhitelistedTokenContainerInit::check_caller_is_admin(&ctx))]
    pub fn initialize_whitelisted_token_container(mut ctx: Context<WhitelistedTokenContainerInit>) -> Result<()> {
        whitelisted_tokens_container_init(&mut ctx)?;
        Ok(())
    }
    
    #[access_control(AddWhitelistedToken::check_caller_is_admin(&ctx))]
    pub fn add_token_to_whitelist(ctx: Context<AddWhitelistedToken>, token_mint: Pubkey) -> Result<()> {
        add_whitelisted_token(ctx, token_mint)?;
        Ok(())
    }
    
    pub fn initialize_staker(ctx: Context<Staker>, token_mint: Pubkey, amount: u64) -> Result<()> {
        stake(ctx, token_mint, amount)?;
        Ok(())
    }
    
    pub fn request_loan(ctx: Context<LoanRequests>, token_mint: Pubkey, loan_amount: u64, interest_rate: u64, loan_duration: i64) -> Result<()> {
        request_loan(ctx, token_mint, loan_amount, interest_rate, loan_duration)?;
        Ok(())
    }
}


