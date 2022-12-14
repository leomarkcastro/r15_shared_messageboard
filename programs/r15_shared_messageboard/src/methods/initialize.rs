use anchor_lang::prelude::*;

use crate::data::MessageBoard;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let message_board = &mut ctx.accounts.message_board;
    message_board.message_list = vec![];
    message_board.message_count = 0;
    message_board.board_title = "Shared Message Board".to_string();
    message_board.board_description =
        "A message board for the community to share messages".to_string();
    message_board.board_owner = *ctx.accounts.user.key;
    message_board.allowed_users = vec![*ctx.accounts.user.key];
    message_board.random_number = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space= 9000)]
    pub message_board: Account<'info, MessageBoard>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
