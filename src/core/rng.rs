use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeterministicRng {
    #[serde(skip)]
    rng: Option<ChaCha8Rng>,
    seed: u64,
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Some(ChaCha8Rng::seed_from_u64(seed)),
            seed,
        }
    }

    #[inline]
    pub fn gen_f32(&mut self) -> f32 {
        self.rng.as_mut().unwrap().gen()
    }
}
