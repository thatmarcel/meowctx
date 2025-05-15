use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImplementationInfo {
    pub name: String,
    pub version: String
}