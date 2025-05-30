use anchor_lang::prelude::*;
use crate::states::*;

pub fn whitelisted_tokens_container_init(ctx: &mut Context<WhitelistedTokenContainerInit>) -> Result<()> {
   let out_ctx = &mut ctx.accounts.whitelisted_token_container;
   let actual_admin = &ctx.accounts.admin_account;
   let caller = &ctx.accounts.caller;

   //storing the whitelisted token container details
   out_ctx.whitelisted_tokens_bump = ctx.bumps.whitelisted_token_container;
   out_ctx.whitelisted_tokens = Vec::new();

   Ok(())
}

pub fn add_whitelisted_token(ctx: Context<AddWhitelistedToken>, token_mint: Pubkey) -> Result<()> {
    let container = &mut ctx.accounts.whitelisted_token_container;
    
    // Check if token is already whitelisted
    require!(
        !container.whitelisted_tokens.contains(&token_mint),
        crate::states::errors::ErrorCode::TokenAlreadyWhitelisted
    );
    
    // Add token to whitelist
    container.whitelisted_tokens.push(token_mint);
    
    Ok(())
}
