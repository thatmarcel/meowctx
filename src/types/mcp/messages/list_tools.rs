use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;
use crate::types::mcp::paginated_request_message_paramas::PaginatedRequestMessageParams;
use crate::types::mcp::tool_info::ToolInfo;

#[allow(dead_code)]
pub type ListToolsRequestMessageParams = PaginatedRequestMessageParams;

#[derive(Serialize, Deserialize)]
pub struct ListToolsResponseMessageResult {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub _meta: JsonRpcMessageObject,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub tools: Vec<ToolInfo>
}

pub const LIST_TOOLS_METHOD: &str = "tools/list";