use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;
use crate::types::mcp::content::Content;

#[derive(Serialize, Deserialize)]
pub struct CallToolRequestMessageParams {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    pub name: String,
    pub arguments: HashMap<String, JsonRpcMessageObject>
}

#[derive(Serialize, Deserialize)]
pub struct CallToolResponseMessageResult {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    pub content: Vec<Content>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>
}

pub const CALL_TOOL_METHOD: &str = "tools/call";