use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct AdminStatus {
    pub status: bool,
}
