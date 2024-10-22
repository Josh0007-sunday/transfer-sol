use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod sol_transfer {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Program initialized!");
        Ok(())
    }

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                },
            ),
            amount,
        )?;

        msg!("Transfer successful! {} lamports transferred", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is safe because we're just transferring SOL
    pub to: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}