use anchor_lang::prelude::*;
use crate::states::accounts::*;

//initializing the administrators account
#[derive(Accounts)]
#[instruction(admin_pubkey: Pubkey)] // enables anchor to retrieve the instruction argument even before the instruction is called 
pub struct AdministratorInit<'info> {
    
    //signer
    #[account(mut)]
    pub deployer: Signer<'info>,

    //creating the administrator account
    #[account(
        init, // the init constrain creates an account, assignimg space for an account
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
    #[account(mut)]
    pub caller: Signer<'info>, //caller is anyone calling the instruction
    
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