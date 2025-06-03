use anchor_lang::prelude::*;
use crate::states::*;

pub fn staker_initialize(ctx: Context<Staker>, token_mint: Pubkey, amount: u64) -> Result<()> {
    let our_ctx = &mut ctx.accounts.staker_user_info;
    let whitelist = &ctx.accounts.whitelisted_token_container; // no add or substraction of the whitelisted token container, hence no need of mut.
    
    // Check if token is whitelisted
    require!(
        whitelist.whitelisted_tokens.contains(&token_mint),
        crate::states::errors::ErrorCode::TokenNotWhitelisted
    );
    
    // Store staker details
    our_ctx.user_pubkey = ctx.accounts.caller.key();
    our_ctx.token_mint = token_mint;
    our_ctx.staker_amount = amount;
    our_ctx.staker_timestamp = Clock::get()?.unix_timestamp;

    Ok(())
}