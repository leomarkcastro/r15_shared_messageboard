use anchor_lang::prelude::*;

use crate::data::messages::MessageBoard;
use crate::errors::MessageBoardError;

pub fn generate_random_number(ctx: Context<GenerateRandomNumber>) -> Result<()> {
    let message_board = &mut ctx.accounts.message_board;
    if message_board.board_owner != *ctx.accounts.user.key {
        return Err(MessageBoardError::NotAMember.into());
    }

    // generate random number based from time, user, and message count
    let random_number = Clock::get()?.unix_timestamp
        + (message_board.message_count as i64)
        + (*ctx.accounts.user.key).as_ref()[0] as i64;

    message_board.random_number = random_number;

    Ok(())
}

#[derive(Accounts)]
pub struct GenerateRandomNumber<'info> {
    #[account(mut)]
    pub message_board: Account<'info, MessageBoard>,
    pub user: Signer<'info>,
}
