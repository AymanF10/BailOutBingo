use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use crate::states::*;

pub fn stake(my_structure: Context<Staker>, token_mint: Pubkey, staking_amount: u64) -> Result<()> {
    let staker_information = &mut my_structure.accounts.staker_info;

    let accounts_involved_in_transfer = Transfer {
        from: my_structure.accounts.stakers_ata.to_account_info(),
        to: my_structure.accounts.pool_ata.to_account_info(),
        authority: my_structure.accounts.staker.to_account_info(),
    };

    let program_doing_transfer = my_structure.accounts.token_program.to_account_info();

    let cpi = CpiContext::new(program_doing_transfer, accounts_involved_in_transfer);

    transfer(cpi, staking_amount)?;
    staker_information.user_pubkey = my_structure.accounts.staker.key();
    staker_information.staker_amount = staking_amount;
    staker_information.staker_timestamp = Clock::get()?.unix_timestamp;
    staker_information.token_mint = token_mint;

    Ok(())
}
   

 
