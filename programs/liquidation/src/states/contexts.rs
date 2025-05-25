use anchor_lang::prelude::*;
use crate::states::accounts::*;

//initializing the administrators account
#[derive(Accounts)]
#[instruction(admin_pubkey: Pubkey)]
pub struct AdministratorInit<'info> {
    
    //signer
    #[account(mut)]
    pub deployer: Signer<'info>,

    //administrator account
    #[account(
        init,
        payer = deployer,
        space = 8 + Administrator::INIT_SPACE,
        seeds = [b"admin", admin_pubkey.as_ref()],
        bump,
    )]
    pub admin_account: Account<'info, Administrator>,
    pub system_program: Program<'info, System>,
}

//whitelisting token container
#[derive(Accounts)]
pub struct WhitelistedTokenContainerInit<'info>{
    //signer
    #[account(mut,
        constraint = admin_account.admin_pubkey == admin.key() @ crate::states::errors::ErrorCode::CallableByAdmin
    )]
    pub admin: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"admin", admin_account.admin_pubkey.as_ref()],
        bump = admin_account.admin_bump
    )]
    pub admin_account: Account<'info, Administrator>,

    //whitelisted token container
    #[account(
        init,
        payer = admin,
        space = 8 + WhitelistedTokenContainer::INIT_SPACE,
        seeds = [b"whitelisted_token_container"], 
        bump,
    )]
    pub whitelisted_token_container: Account<'info, WhitelistedTokenContainer>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistedTokenContainerInit<'info> {
    pub fn check_caller_is_admin(ctx: &Context<WhitelistedTokenContainerInit>) -> Result<()> {
        require!(
            ctx.accounts.admin_account.admin_pubkey == ctx.accounts.admin.key(),
            crate::states::errors::ErrorCode::CallableByAdmin
        );
        Ok(())
    }
}