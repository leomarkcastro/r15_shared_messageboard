use anchor_lang::prelude::*;
use anchor_lang::system_program;

// pub fn transfer_solana(ctx: Context<TransferSolana>) -> Result<()> {
//     Ok(())
// }

#[derive(Accounts)]
pub struct TransferSolana<'info> {
    #[account(mut)]
    recipient: SystemAccount<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn transfer_solana(ctx: Context<TransferSolana>, amount: u64) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
