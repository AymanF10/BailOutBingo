use anchor_lang::prelude::*;
use anchor_spl::token_interface::TransferChecked;
use anchor_spl::token_interface::transfer_checked;
use crate::states::*;

pub fn stake(my_structure: Context<Staker>, token_mint: Pubkey, staking_amount: u64,) -> Result<()> {
    let staker_information = &mut my_structure.accounts.staker_info;

    let accounts_involved_in_transfer = TransferChecked {
        from: my_structure.accounts.stakers_ata.to_account_info(),
        mint: my_structure.accounts.staking_token.to_account_info(),
        to: my_structure.accounts.pool_ata.to_account_info(),
        authority: my_structure.accounts.staker.to_account_info(),
    };

    let program_doing_transfer = my_structure.accounts.token_program.to_account_info();

    let cpi = CpiContext::new(program_doing_transfer, accounts_involved_in_transfer);
    

    transfer_checked(cpi, staking_amount, my_structure.accounts.staking_token.decimals)?;
    staker_information.user_pubkey = my_structure.accounts.staker.key();
    staker_information.staker_amount = staking_amount;
    staker_information.staker_timestamp = Clock::get()?.unix_timestamp;
    staker_information.token_mint = token_mint;

    Ok(())
}
   

 
