use anchor_lang::prelude::*;
use crate::states::*;

pub fn whitelisted_tokens_container_init(ctx: &mut Context<WhitelistedTokenContainerInit>) -> Result<()> {
   let out_ctx = &mut ctx.accounts.whitelisted_token_container;


   //storing the whitelisted token container details
   out_ctx.whitelisted_tokens_bump = ctx.bumps.whitelisted_token_container;
   out_ctx.whitelisted_tokens = Vec::new(); // vec::new is used to create a new vector.

   Ok(())
}

pub fn add_whitelisted_token(ctx: Context<AddWhitelistedToken>, token_mint: Pubkey) -> Result<()> {
    let container = &mut ctx.accounts.whitelisted_token_container;
    
    // Check if token is already whitelisted
    require!(
        !container.whitelisted_tokens.contains(&token_mint), // we use contains only on vectors.
        crate::states::errors::ErrorCode::TokenAlreadyWhitelisted
    );
    
    // Add token to whitelist
    container.whitelisted_tokens.push(token_mint); // push is used to add an element to the vector.
    
    Ok(())
}
