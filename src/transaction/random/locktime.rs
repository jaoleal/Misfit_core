use bitcoin::absolute::LockTime;
use secp256k1::rand::{self, Rng};

pub trait RandomLockTime {
    fn random() -> LockTime;
}

impl RandomLockTime for LockTime {
    fn random() -> LockTime {
        let choice = rand::thread_rng().gen_range(0..5);

        match choice {
            0 => LockTime::from_consensus(rand::thread_rng().gen::<u32>()),
            1 => LockTime::from_height(rand::thread_rng().gen::<u32>()).unwrap_or(LockTime::ZERO),
            2 => {
                let hex_value = format!("{:X}", rand::thread_rng().gen::<u32>());
                LockTime::from_hex(&hex_value).unwrap_or(LockTime::ZERO)
            }
            3 => LockTime::from_time(rand::thread_rng().gen::<u32>()).unwrap_or(LockTime::ZERO),
            _ => LockTime::ZERO,
        }
    }
}
