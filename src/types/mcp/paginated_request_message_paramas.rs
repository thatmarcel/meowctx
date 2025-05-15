use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;

#[derive(Serialize, Deserialize)]
pub struct PaginatedRequestMessageParams {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>
}