use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

pub trait AccountCheck {
    fn check(&self, account: &Pubkey) -> Result<(), ProgramError>;
}

pub struct SignerAccount;

impl AccountCheck for SignerAccount {
    pub fn check(&self, account: &Pubkey) -> Result<(), ProgramError> {
        if !account.isSigner() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}