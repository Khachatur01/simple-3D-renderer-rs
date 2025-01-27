use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PlaneDirection {
    YZ,
    XZ,
    XY,
}
