// Import Anchor framework
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use solana_program::entrypoint::ProgramResult;

// Define custom error types
#[derive(Error, Debug)]
pub enum StakingError {
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Token transfer failed")]
    TokenTransferFailed,
    #[error("Referral code not found")]
    ReferralCodeNotFound,
}

// Define custom error codes
#[program]
mod staking {
    use super::*;

    #[error]
    pub enum ErrorCode {
        #[msg("Invalid amount")]
        InvalidAmount,
        #[msg("Insufficient balance")]
        InsufficientBalance,
        #[msg("Token transfer failed")]
        TokenTransferFailed,
        #[msg("Referral code not found")]
        ReferralCodeNotFound,
    }

    // Define the BARK staking program
    pub struct Staking;

    #[derive(Accounts)]
    pub struct Stake<'info> {
        #[account(signer)]
        pub authority: AccountInfo<'info>,
        #[account(mut)]
        pub staker_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub staking_account: Account<'info, TokenAccount>,
        #[account(mut)]
        pub treasury: Account<'info, TokenAccount>,
        #[account(mut)]
        pub token_program: AccountInfo<'info>,
    }

    impl<'info> Staking {
        // Function to stake BARK tokens
        #[access_control(check_stake_authority(&ctx))]
        pub fn stake(&self, ctx: Context<Stake>, amount: u64, referral_code: Option<String>) -> ProgramResult {
            // Validate amount
            if amount == 0 {
                return Err(StakingError::InvalidAmount.into());
            }
            // Check user balance
            let staker_token_balance = ctx.accounts.staker_token_account.amount;
            if staker_token_balance < amount {
                return Err(StakingError::InsufficientBalance.into());
            }
            // Stake BARK tokens
            let transfer_amount = (amount as f64 * 0.95) as u64;
            // Apply referral rebate if a valid referral code is provided
            let rebate_amount = self.apply_referral_rebate(transfer_amount, &referral_code, &ctx.accounts.staking_account)?;
            // Transfer BARK tokens from staker's account to staking account only if there's a non-zero transfer amount
            if transfer_amount > 0 {
                self.transfer_tokens(transfer_amount, &ctx.accounts)?;
                // Update staking account state.
                ctx.accounts.staking_account.amount += transfer_amount;
            }
            // Update rewards with the rebate amount
            ctx.accounts.staking_account.total_rewards += rebate_amount;
            Ok(())
        }

        // Function to apply referral rebate
        fn apply_referral_rebate(&self, amount: u64, referral_code: &Option<String>, staking_account: &TokenAccount) -> Result<u64, ProgramError> {
            if let Some(code) = referral_code {
                if let Some(stored_code) = &staking_account.referral_code {
                    if code == stored_code {
                        return Ok((amount as f64 * 0.15) as u64);
                    } else {
                        return Err(StakingError::ReferralCodeNotFound.into());
                    }
                }
            }
            Ok(0)
        }

        // Function to transfer BARK tokens
        fn transfer_tokens(&self, amount: u64, accounts: &Stake) -> ProgramResult {
            token::transfer(
                accounts.token_program.clone(),
                accounts.staker_token_account.to_account_info().clone(),
                accounts.staking_account.to_account_info().clone(),
                accounts.authority.clone(),
                &[],
                amount,
            ).map_err(|_| StakingError::TokenTransferFailed.into())
        }
    }

    // Access control function to check if the authority is the signer
    fn check_stake_authority(ctx: &Context<Stake>) -> Result<(), ErrorCode> {
        if ctx.accounts.authority.key != ctx.accounts.staker_token_account.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        Ok(())
    }
}
