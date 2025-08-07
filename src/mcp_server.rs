use crate::mcp_server_builder::McpServerBuilder;
use crate::mcp_server_tool::McpServerTool;
use crate::mcp_server_tool_property::McpServerToolPropertyValue;
use crate::types::jsonrpc_message::{JsonRpcMessage, JsonRpcMessageObject, JSONRPC_VERSION};
use crate::types::mcp::capabilities::{CapabilitiesTools, ServerCapabilities};
use crate::types::mcp::content::{Content, TextContent};
use crate::types::mcp::implementation_info::ImplementationInfo;
use crate::types::mcp::messages::call_tool::{CallToolRequestMessageParams, CallToolResponseMessageResult, CALL_TOOL_METHOD};
use crate::types::mcp::messages::initialize::{InitializeRequestMessageParams, InitializeResponseMessageResult, INITIALIZE_METHOD};
use crate::types::mcp::messages::list_prompts::{ListPromptsResponseMessageResult, LIST_PROMPTS_METHOD};
use crate::types::mcp::messages::list_resource_templates::{ListResourceTemplatesResponseMessageResult, LIST_RESOURCE_TEMPLATES_METHOD};
use crate::types::mcp::messages::list_resources::{ListResourcesResponseMessageResult, LIST_RESOURCES_METHOD};
use crate::types::mcp::messages::list_roots::{ListRootsResponseMessageResult, LIST_ROOTS_METHOD};
use crate::types::mcp::messages::list_tools::{ListToolsResponseMessageResult, LIST_TOOLS_METHOD};
use crate::types::mcp::messages::ping::PING_METHOD;
use anyhow::anyhow;
use std::collections::HashMap;
use std::io::Write;

pub struct McpServer {
    pub(crate) display_name: String,
    pub(crate) display_version: String,
    pub(crate) tools: Vec<McpServerTool>
}

impl McpServer {
    pub fn with_name_and_version(name: &str, version: &str) -> McpServerBuilder {
        McpServerBuilder::with_name_and_version(name.to_string(), version.to_string())
    }

    pub fn start(&self) {
        loop {
            let mut line_content = String::new();
            _ = std::io::stdin().read_line(&mut line_content)
                .inspect_err(|e| eprintln!("{}", e));

            let incoming_message: JsonRpcMessage = match serde_json::from_str(line_content.as_str()) {
                Ok(im) => im,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            match self.handle_message(&incoming_message) {
                Ok(Some(outgoing_message)) => {
                    std::io::stdout().write_all(
                        format!("{}\n", serde_json::to_string(&outgoing_message).unwrap()
                    ).as_bytes()).unwrap();
                },
                Ok(None) => {},
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        }
    }

    #[cfg(feature = "openapi-server")]
    pub async fn serve_openapi(&self, bearer_auth_token: Option<String>) {
        crate::openapi_server::serve_openapi(&self, bearer_auth_token).await;
    }

    fn handle_message(&self, message: &JsonRpcMessage) -> Result<Option<JsonRpcMessage>, anyhow::Error> {
        if message.id.is_none() {
            return Ok(None);
        }

        match &message.method {
            Some(method) => {
                self.handle_message_with_method(message, method.as_str())
            },
            None => Err(anyhow!("Message without method"))
        }
    }

    fn handle_message_with_method(&self, request_message: &JsonRpcMessage, method: &str) -> Result<Option<JsonRpcMessage>, anyhow::Error> {
        match method {
            CALL_TOOL_METHOD => {
                let params: CallToolRequestMessageParams = serde_json::from_str(
                    serde_json::to_string(&request_message.params)?.as_str()
                )?;

                let tool = self.tools.iter()
                    .filter(|t| t.name == params.name)
                    .next()
                    .ok_or(anyhow!("Unknown tool"))?;

                let arguments: HashMap<String, McpServerToolPropertyValue> = params.arguments.iter().filter_map(|(argument_property_identifier, argument_property_value)| {
                    let value: McpServerToolPropertyValue = match serde_json::to_string(&argument_property_value) {
                        Ok(argument_property_value_as_string) => match serde_json::from_str(
                            argument_property_value_as_string.as_str()
                        ) {
                            Ok(v) => v,
                            Err(_) => return None
                        },
                        Err(_) => return None
                    };

                    Some((argument_property_identifier.to_string(), value))
                }).collect();

                let function_result = (tool.function)(arguments);

                let result = CallToolResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    content: match function_result.clone() {
                        Some(fr) => vec![
                            Content::Text(TextContent {
                                content_type: "text".to_string(),
                                text: serde_json::to_string(&fr)?,
                            })
                        ],
                        None => vec![]
                    },
                    is_error: Some(function_result.is_none())
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            INITIALIZE_METHOD => {
                let params: InitializeRequestMessageParams = serde_json::from_str(
                    serde_json::to_string(&request_message.params)?.as_str()
                )?;

                let result = InitializeResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    protocol_version: params.protocol_version, // "2025-03-26".to_string(),
                    capabilities: ServerCapabilities {
                        experimental: JsonRpcMessageObject::Null,
                        logging: JsonRpcMessageObject::Null,
                        completions: JsonRpcMessageObject::Null,
                        prompts: None,
                        resources: None,
                        tools: Some(CapabilitiesTools {
                            list_changed: Some(false)
                        })
                    },
                    server_info: ImplementationInfo {
                        name: self.display_name.clone(),
                        version: self.display_version.clone()
                    },
                    instructions: None
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            LIST_PROMPTS_METHOD => {
                /* let _params: ListPromptsRequestMessageParams = serde_json::from_str(
                    serde_json::to_string(&request_message.params)?.as_str()
                )?; */

                let result: ListPromptsResponseMessageResult = ListPromptsResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    next_cursor: None,
                    prompts: vec![]
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            LIST_RESOURCE_TEMPLATES_METHOD => {
                /* let _params: ListResourceTemplatesRequestMessageParams = serde_json::from_str(
                    serde_json::to_string(&request_message.params)?.as_str()
                )?; */

                let result = ListResourceTemplatesResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    next_cursor: None,
                    resource_templates: vec![]
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            LIST_RESOURCES_METHOD => {
                /* let _params: ListResourcesRequestMessageParams = serde_json::from_str(
                    serde_json::to_string(&request_message.params)?.as_str()
                )?; */

                let result = ListResourcesResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    next_cursor: None,
                    resources: vec![]
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            LIST_ROOTS_METHOD => {
                let result = ListRootsResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    roots: vec![]
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            LIST_TOOLS_METHOD => {
                /* let _params: ListToolsRequestMessageParams = serde_json::from_str(
                    serde_json::to_string(&request_message.params)?.as_str()
                )?; */

                let result = ListToolsResponseMessageResult {
                    _meta: JsonRpcMessageObject::Null,
                    next_cursor: None,
                    tools: self.tools.iter()
                        .map(|t| t.to_tool_info())
                        .collect()
                };

                let result_object: JsonRpcMessageObject = serde_json::from_str(
                    serde_json::to_string(&result)?.as_str()
                )?;

                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: result_object,
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            },
            PING_METHOD => {
                let response_message = JsonRpcMessage {
                    version: JSONRPC_VERSION.to_string(),
                    id: request_message.id.clone(),
                    method: None,
                    result: JsonRpcMessageObject::Dictionary(HashMap::new()),
                    params: JsonRpcMessageObject::Null,
                    error: None
                };

                Ok(Some(response_message))
            }
            _ => Err(anyhow!("Unknown method"))
        }
    }
}