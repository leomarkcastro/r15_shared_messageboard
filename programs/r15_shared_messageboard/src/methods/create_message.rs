use anchor_lang::prelude::*;

use crate::data::messages::{Message, MessageBoard};
use crate::errors::MessageBoardError;

pub fn create_message(ctx: Context<CreateMessage>, content: String) -> Result<()> {
    // check if user is allowed to post
    let message_board = &mut ctx.accounts.message_board;
    if !message_board.allowed_users.contains(&ctx.accounts.user.key) {
        return Err(MessageBoardError::NotAMember.into());
    }

    let message = Message {
        author: *ctx.accounts.user.key,
        content,
        likers: vec![],
        likes: 0,
        timestamp: Clock::get()?.unix_timestamp,
    };

    message_board.message_list.push(message);
    message_board.message_count += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateMessage<'info> {
    #[account(mut)]
    pub message_board: Account<'info, MessageBoard>,
    pub user: Signer<'info>,
}
