use core::mem::size_of;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
 
#[repr(C)]
pub struct Config {
    state: u8,
    seed: [u8; 8],
    authority: Pubkey,
    mint_x: Pubkey,
    mint_y: Pubkey,
    fee: [u8; 2],
    config_bump: [u8; 1],
}
 
#[repr(u8)]
pub enum AmmState {
    Uninitialized = 0u8,
    Initialized = 1u8,
    Disabled = 2u8,
    WithdrawOnly = 3u8,
}
 
impl Config {
    pub const LEN: usize = size_of::<Config>();
 
    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Config::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
         if bytes.len() != Config::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { & *core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    pub fn set_state(&mut self, state: AmmState) {
        self.state = state as u8;
    }

    pub fn set_seed(&mut self, seed: [u8; 8]) {
        self.seed = seed;
    }
 
    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }
 
    pub fn set_mint_x(&mut self, mint_x: Pubkey) {
        self.mint_x = mint_x;
    }
 
    pub fn set_mint_y(&mut self, mint_y: Pubkey) {
        self.mint_y = mint_y;
    }
 
    pub fn set_fee(&mut self, fee: [u8; 2]) {
        self.fee = fee;
    }
 
    pub fn set_config_bump(&mut self, config_bump: [u8; 1]) {
        self.config_bump = config_bump;
    }
}
