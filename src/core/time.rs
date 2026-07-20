use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[repr(transparent)]
pub struct Tick(pub u64);

impl core::ops::Add<u64> for Tick {
    type Output = Tick;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        Tick(self.0 + rhs)
    }
}

impl core::ops::Sub<Tick> for Tick {
    type Output = u64;
    #[inline]
    fn sub(self, rhs: Tick) -> Self::Output {
        self.0.saturating_sub(rhs.0)
    }
}
