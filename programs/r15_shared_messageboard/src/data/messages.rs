use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Message {
    pub author: Pubkey,
    pub content: String,
    pub likers: Vec<Pubkey>,
    pub likes: u64,
    pub timestamp: i64,
}

#[account]
pub struct MessageBoard {
    pub message_list: Vec<Message>,
    pub message_count: u64,
    pub board_title: String,
    pub board_description: String,
    pub board_owner: Pubkey,
    pub allowed_users: Vec<Pubkey>,
    pub random_number: i64,
}
