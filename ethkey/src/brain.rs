use super::{Error, Generator, KeyPair, Secret};
use keccak::Keccak256;

/// Simple brainwallet.
pub struct Brain(String);

impl Brain {
    pub fn new(s: String) -> Self {
        Brain(s)
    }
}

impl Generator for Brain {
    fn generate(self) -> Result<KeyPair, Error> {
        let seed = self.0;
        let mut secret = seed.bytes().collect::<Vec<u8>>().keccak256();

        let mut i = 0;
        loop {
            secret = secret.keccak256();

            match i > 16384 {
                false => i += 1,
                true => {
                    let result = KeyPair::from_secret(Secret::from(secret.clone()));
                    if result.is_ok() {
                        return result;
                    }
                }
            }
        }
    }
}
