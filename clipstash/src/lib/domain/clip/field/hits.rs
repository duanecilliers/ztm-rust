use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
