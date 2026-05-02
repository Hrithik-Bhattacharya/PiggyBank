use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, program::invoke_signed, system_instruction};

declare_id!("311bKTpsjmh5BJCLvcE4TftcggpWRSR9Hym38k6hoNfp");

#[program]
pub mod piggy_bank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let piggy_bank = &mut ctx.accounts.piggy_bank;
        piggy_bank.owner = *ctx.accounts.user.key;
        piggy_bank.bump = ctx.bumps.piggy_bank;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.piggy_bank.key(),
            amount,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.piggy_bank.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let piggy_bank_info = ctx.accounts.piggy_bank.to_account_info();
        let owner_info = ctx.accounts.owner.to_account_info();
        if piggy_bank_info.lamports() < amount {
            return Err(ProgramError::InsufficientFunds.into());
        }

        **piggy_bank_info.try_borrow_mut_lamports()? = piggy_bank_info
            .lamports()
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        **owner_info.try_borrow_mut_lamports()? = owner_info
            .lamports()
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = PiggyBank::LEN,
        seeds = [b"piggy-bank", user.key().as_ref()],
        bump
    )]
    pub piggy_bank: Account<'info, PiggyBank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"piggy-bank", user.key().as_ref()],
        bump = piggy_bank.bump
    )]
    pub piggy_bank: Account<'info, PiggyBank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"piggy-bank", owner.key().as_ref()],
        bump = piggy_bank.bump,
        has_one = owner, 
    )]
    pub piggy_bank: Account<'info, PiggyBank>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub bump: u8,
}

impl PiggyBank {
    pub const LEN: usize = 8 + 32 + 1;
}