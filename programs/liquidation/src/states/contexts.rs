use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, Mint, TokenInterface};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::accounts::*;

#[derive(Accounts)]
#[instruction(admin_pubkey: Pubkey)]
pub struct AdministratorInit<'info> {
    #[account(mut)]
    pub deployer: Signer<'info>,

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

#[derive(Accounts)]
pub struct WhitelistedTokenContainerInit<'info>{
    #[account(mut)]
    pub caller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"admin", admin_account.admin_pubkey.as_ref()],
        bump = admin_account.admin_bump
    )]
    pub admin_account: Account<'info, Administrator>,

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

#[derive(Accounts)]
pub struct AddWhitelistedToken<'info> {
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

#[derive(Accounts)]
#[instruction(token_mint: Pubkey)]
pub struct Staker<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        init,
        payer = staker,
        space = 8 + StakerUserInfo::INIT_SPACE,
        seeds = [b"staker", staker.key().as_ref()],
        bump,
    )]
    pub staker_info: Account<'info, StakerUserInfo>,
    
    #[account(
        constraint = staking_token.key() == token_mint
    )]
    pub staking_token: InterfaceAccount<'info, Mint>,
    
    #[account(
        associated_token::mint = staking_token,
        associated_token::authority = staker,
    )]
    pub stakers_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"admin", admin_account.admin_pubkey.as_ref()],
        bump = admin_account.admin_bump
    )]
    pub admin_account: Account<'info, Administrator>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = staking_token,
        associated_token::authority = admin_account,
    )] 
    pub pool_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,       
}

#[derive(Accounts)]
#[instruction(token_mint: Pubkey)]
pub struct LoanRequests <'info> {
    #[account(mut)]
    pub caller: Signer <'info>,

    #[account(
        init,
        payer = caller,
        space = 8 + loanInfo::INIT_SPACE,
        seeds = [b"LoanInfo", caller.key().as_ref()],
        bump,
    )]
    pub loan_account: Account<'info, loanInfo>,
    
    #[account(
        seeds = [b"whitelisted_token_container"],
        bump = whitelisted_token_container.whitelisted_tokens_bump
    )]
    pub whitelisted_token_container: Account<'info, WhitelistedTokenContainer>,
    
    pub system_program: Program<'info, System>,
}
