use anchor_lang::prelude::*;

declare_id!("5Wnpeo6hA75WGacPT9THrf1GFYsUky5JnEfh2b91tgkc");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
        new_account.owner = ctx.accounts.signer.key();
        new_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.new_account.count = ctx
            .accounts
            .new_account
            .count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        ctx.accounts.new_account.count = ctx
            .accounts
            .new_account
            .count
            .checked_sub(1)
            .ok_or(ErrorCode::Underflow)?;
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        ctx.accounts.new_account.count = 0;
        Ok(())
    }
}

#[account]
pub struct AccountInstructions {
    pub owner: Pubkey,
    pub count: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8)]
    pub new_account: Account<'info, AccountInstructions>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = owner @ ErrorCode::Unauthorized)]
    pub new_account: Account<'info, AccountInstructions>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut, has_one = owner @ ErrorCode::Unauthorized)]
    pub new_account: Account<'info, AccountInstructions>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut, has_one = owner @ ErrorCode::Unauthorized)]
    pub new_account: Account<'info, AccountInstructions>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("Overflow: cannot add")]
    Overflow,
    #[msg("Underflow: cannot subtract")]
    Underflow,
}
