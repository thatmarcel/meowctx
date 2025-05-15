use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;
use crate::types::mcp::paginated_request_message_paramas::PaginatedRequestMessageParams;

#[allow(dead_code)]
pub type ListPromptsRequestMessageParams = PaginatedRequestMessageParams;

#[derive(Serialize, Deserialize)]
pub struct ListPromptsResponseMessageResult {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub prompts: Vec<JsonRpcMessageObject>
}

pub const LIST_PROMPTS_METHOD: &str = "prompts/list";