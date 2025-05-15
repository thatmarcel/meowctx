use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;
use crate::types::mcp::paginated_request_message_paramas::PaginatedRequestMessageParams;

#[allow(dead_code)]
pub type ListResourceTemplatesRequestMessageParams = PaginatedRequestMessageParams;

#[derive(Serialize, Deserialize)]
pub struct ListResourceTemplatesResponseMessageResult {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<JsonRpcMessageObject>
}

pub const LIST_RESOURCE_TEMPLATES_METHOD: &str = "resources/templates/list";