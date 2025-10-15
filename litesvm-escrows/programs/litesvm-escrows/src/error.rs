use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Escrow not found")]
    EscrowNotFound,
    #[msg("Escrow still Locked up")]
    NotUnlocked,
}