use anchor_lang::prelude::*;

use crate::{data::MessageBoard, errors::MessageBoardError};

pub fn add_allowed_user(ctx: Context<AddAllowedUser>) -> Result<()> {
    let message_board = &mut ctx.accounts.message_board;
    if message_board.board_owner != *ctx.accounts.user.key {
        return Err(MessageBoardError::NotAMember.into());
    }

    message_board.allowed_users.push(*ctx.accounts.new_user.key);

    Ok(())
}

#[derive(Accounts)]
pub struct AddAllowedUser<'info> {
    #[account(mut)]
    pub message_board: Account<'info, MessageBoard>,
    pub user: Signer<'info>,
    /// CHECK: Is this the right way to do this?
    pub new_user: AccountInfo<'info>,
}
