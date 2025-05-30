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
        payer = caller,
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
            ctx.accounts.admin_account.admin_pubkey == ctx.accounts.caller.key(),
            crate::states::errors::ErrorCode::CallableByAdmin
        );
        Ok(())
    }
}

// Add token to whitelist context
#[derive(Accounts)]
pub struct AddWhitelistedToken<'info> {
    //signer
    #[account(mut)]
    pub caller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"admin", admin_account.admin_pubkey.as_ref()],
        bump = admin_account.admin_bump
    )]
    pub admin_account: Account<'info, Administrator>,
    
    #[account(
        mut,
        seeds = [b"whitelisted_token_container"],
        bump = whitelisted_token_container.whitelisted_tokens_bump
    )]
    pub whitelisted_token_container: Account<'info, WhitelistedTokenContainer>,
}

impl<'info> AddWhitelistedToken<'info> {
    pub fn check_caller_is_admin(ctx: &Context<AddWhitelistedToken>) -> Result<()> {
        require!(
            ctx.accounts.admin_account.admin_pubkey == ctx.accounts.caller.key(),
            crate::states::errors::ErrorCode::CallableByAdmin
        );
        Ok(())
    }
}

// Staking user info
#[derive(Accounts)]
#[instruction(token_mint: Pubkey, amount: u64)]
pub struct Staker<'info> {
    //signer
    #[account(mut)]
    pub caller: Signer<'info>,

    //staker user info
    #[account(
        init,
        payer = caller,
        space = 8 + StakerUserInfo::INIT_SPACE,
        seeds = [b"staker", caller.key().as_ref(), token_mint.as_ref()],
        bump,
    )]
    pub staker_user_info: Account<'info, StakerUserInfo>,
    
    #[account(
        seeds = [b"whitelisted_token_container"],
        bump = whitelisted_token_container.whitelisted_tokens_bump
    )]
    pub whitelisted_token_container: Account<'info, WhitelistedTokenContainer>,
    
    pub system_program: Program<'info, System>,
}