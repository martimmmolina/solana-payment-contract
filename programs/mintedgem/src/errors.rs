use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Mintedgem: The master account is already initialized")]
    MasterAccountAlreadyInitialized,

    #[msg("Mintedgem: Insufficient amount")]
    InsufficientAmount,

    #[msg("Mintedgem: Only owner can call this function!")]
    NotOwner,

    #[msg("Mintedgem: Vault SOL is already initialized")]
    VaultSolAlreadyInitialized,

    #[msg("Mintedgem: Vault DONE token is already initialized")]
    VaultDoneTokenAlreadyInitialized,

    #[msg("Mintedgem: Amount must be greater than 0")]
    InvalidAmount,

    #[msg("Mintedgem: Percent must be greater thean or equal 0 and less than or equal 100")]
    InvalidPercent,

    #[msg("Mintedgem: Invalid creator")]
    InvalidCreator,
}
