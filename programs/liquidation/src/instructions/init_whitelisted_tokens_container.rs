use anchor_lang::prelude::*;
use crate::states::*;

pub fn whitelisted_tokens_container_init(ctx: Context<WhitelistedTokenContainerInit>) -> Result<()> {
   let out_ctx = &mut ctx.accounts.whitelisted_token_container;
   let actual_admin = &ctx.accounts.admin_account;
   let caller = &ctx.accounts.admin;

   //storing the whitelisted token container details
   out_ctx.whitelisted_tokens_bump = ctx.bumps.whitelisted_token_container;
   out_ctx.whitelisted_tokens = Vec::new();

   Ok(())
}
