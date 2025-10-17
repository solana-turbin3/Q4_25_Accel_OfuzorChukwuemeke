use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Escrow not found")]
    EscrowNotFound,
    #[msg("Escrow still Locked up")]
    NotUnlocked,
}