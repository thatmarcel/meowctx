use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;

#[derive(Serialize, Deserialize)]
pub struct ListRootsResponseMessageResult {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    pub roots: Vec<JsonRpcMessageObject>
}

pub const LIST_ROOTS_METHOD: &str = "roots/list";