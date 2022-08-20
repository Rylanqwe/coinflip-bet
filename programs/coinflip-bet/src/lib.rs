use anchor_lang::prelude::*;
use vipers::prelude::*;

mod account_validators;

declare_id!("4SmSWTXY3MXgXW35rRfvfgEpZLASPeAbFcF87kyqjhNu");

#[error_code]
pub enum CustomError {
    #[msg("Insufficient Funds")]
    InsufficientFundsForTransaction,
}

#[program]
mod coinflip_bet {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    #[access_control(ctx.accounts.validate())]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("{}", ctx.accounts.wager.key());
        let ix = system_instruction::transfer(&ctx.accounts.user.key(), &ctx.accounts.wager.key(), 100);
        invoke_signed(
            &ix,
            &[ctx.accounts.user.to_account_info()],
            &[&[ctx.accounts.user.key.to_bytes().as_ref()]]
        )?;
        // let amount = 1000;
        // if **ctx.accounts.user.try_borrow_lamports()? < amount {
        //     return Err(anchor_lang::error!(CustomError::InsufficientFundsForTransaction));
        // }
        // **ctx.accounts.user.try_borrow_mut_lamports()? -= amount;
        // **ctx.accounts.wager.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn update(ctx: Context<Update>) -> Result<()> {
        let my_account = &mut ctx.accounts.flip_result;
        let timestamp = Clock::get()?.unix_timestamp;
        let roll = ((timestamp + 7789) * 997) % 100;
        let won = roll < 49;
        my_account.roll = roll;
        my_account.won = won;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 1,
        seeds = [b"wager", user.key().as_ref()],
        bump
    )]
    pub wager: Account<'info, Wager>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub flip_result: Account<'info, FlipResult>,
}

#[account]
pub struct FlipResult {
    pub roll: i64,
    pub won: bool,
}

#[account]
pub struct Wager {
    pub bump: u8,
}