use anchor_lang::prelude::error_code;

#[error_code]
pub enum MsgError {
    Unauthorized,
    // InvalidSender,
    // InvalidDecimals,
    // SlippageExceeded,
    // InvalidTokenDest,
    RateLimitExceeded,
    InvalidFee,
    // InvalidMintAuthority,
    // Paused,
}
