use anchor_lang::prelude::*;
use crate::states::accounts::*;

//initializing the administrators account
#[derive(Accounts)]
#[instruction(admin_pubkey: Pubkey)] // enables anchor to retrieve the instruction argument even before the instruction is called 
pub struct AdministratorInit<'info> {
    
    //signer
    #[account(mut)] //mut can be modified(change in the lamports amount, as fees is needed to be paid for an account formation)
    pub deployer: Signer<'info>,

    //creating the administrator account
    #[account(
        init, // the init constrain creates an account, assignimg space for an account
        payer = deployer,
        space = 8 + Administrator::INIT_SPACE, // 8 is the discriminator size for the account.
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
    

    //bringing the admin account here to reference it.
    #[account(
        mut,
        seeds = [b"admin", admin_account.admin_pubkey.as_ref()], // already created an admin account which stores the admin pubkey. so here we are retrieving the admin pub key which we already have stored in the the admin account, during initialization.
        bump = admin_account.admin_bump // bumb is also stored in the admin account.
    )]
    pub admin_account: Account<'info, Administrator>,

    //whitelisted token container
    #[account(
        init,
        payer = caller,
        space = 8 + WhitelistedTokenContainer::INIT_SPACE,
        seeds = [b"whitelisted_token_container"], //just a container to store the tokens ..dosent need to be unique, so pubkey etc..is not needed, a string is enough.
        bump,
    )]
    pub whitelisted_token_container: Account<'info, WhitelistedTokenContainer>,
    pub system_program: Program<'info, System>,
}


//enables us to define some functions or other structs to be used in a particular struct. 
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
#[instruction(token_mint: Pubkey)]
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