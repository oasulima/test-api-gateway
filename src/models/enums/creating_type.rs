use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CreatingType {
    Unknown,
    SingleEntry,
    MultiEntry,
    Buy,
    CoverNegative,
    ReturnedOneTimePreBorrow,
    Overbuy,
    UnwantedPartial,
}
