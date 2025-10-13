
use serde::{Deserialize, Serialize};
use solana_sysvar::{Sysvar, SysvarSerialize};

#[repr(C)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Lock {
    pub locked: bool,
}

impl Lock {
    pub fn new(locked: bool) -> Self {
        Self { locked }
    }
}

solana_pubkey::declare_id!("LockSysvar111111111111111111111111111111111");
    impl solana_sysvar_id::SysvarId for Lock {
        fn id() -> solana_pubkey::Pubkey {
            id()
        }

        fn check_id(pubkey: &solana_pubkey::Pubkey) -> bool {
            check_id(pubkey)
        }
    }

impl Lock {}
impl Sysvar for Lock {}
impl SysvarSerialize for Lock {}
