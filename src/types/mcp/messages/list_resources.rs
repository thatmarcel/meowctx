use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;
use crate::types::mcp::paginated_request_message_paramas::PaginatedRequestMessageParams;

#[allow(dead_code)]
pub type ListResourcesRequestMessageParams = PaginatedRequestMessageParams;

#[derive(Serialize, Deserialize)]
pub struct ListResourcesResponseMessageResult {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub resources: Vec<JsonRpcMessageObject>
}

pub const LIST_RESOURCES_METHOD: &str = "resources/list";