use anchor_lang::error_code;

#[error_code]
pub enum MessageBoardError {
    NotAMember,
    TransferFailed,
}
