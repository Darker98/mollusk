
use serde::{Deserialize, Serialize};
use solana_sysvar::{Sysvar, SysvarSerialize};

#[repr(C)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Lock {
    // global state of the chain
    pub locked: bool,

    // when the lock state was last changed
    pub last_changed: i64,

    // how many times the lock state has been changed
    pub lock_count: u64,
}

impl Lock {
    pub fn new(locked: bool, last_changed: i64) -> Self {
        Self { locked, last_changed, lock_count: 0 }
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
    
impl Sysvar for Lock {}
impl SysvarSerialize for Lock {}
