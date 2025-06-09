use bitcoin::blockdata::transaction::Version;
use secp256k1::rand::{self, Rng};

pub trait RandomVersion {
    fn random() -> Version;
}

impl RandomVersion for Version {
    fn random() -> Version {
        // Random standard
        if rand::thread_rng().gen_bool(0.5) {
            if rand::thread_rng().gen_bool(0.5) {
                return Version::ONE;
            }
            return Version::TWO;
        }

        // Random non_standard
        Version::non_standard(rand::thread_rng().gen::<i32>())
    }
}
