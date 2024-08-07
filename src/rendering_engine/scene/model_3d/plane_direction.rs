use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum PlaneDirection {
    YZ,
    XZ,
    XY,
}
