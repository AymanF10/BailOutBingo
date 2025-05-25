use anchor_lang::prelude::*;
use crate::states::*;

//Instruction to initialize the admin account
pub fn admin_initialize(ctx: Context<AdministratorInit>, admin_pubkey: Pubkey) -> Result<()> {
    let our_ctx = &mut ctx.accounts.admin_account;

    //store admin details
    our_ctx.admin_pubkey = admin_pubkey;
    our_ctx.admin_bump = ctx.bumps.admin_account;
    
    Ok(())
}