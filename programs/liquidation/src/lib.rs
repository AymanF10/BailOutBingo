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
    pub fn initialize_whitelisted_token_container(ctx: Context<WhitelistedTokenContainerInit>) -> Result<()> {
        whitelisted_tokens_container_init(ctx)?;
        Ok(())
    }
}


