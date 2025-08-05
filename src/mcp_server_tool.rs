use crate::mcp_server_tool_property::{McpServerToolAnyOfPropertyType, McpServerToolArrayPropertyInnerType, McpServerToolPropertyInfo, McpServerToolPropertyType, McpServerToolPropertyValue};
use crate::types::mcp::tool_info::{ToolInfo, ToolInputSchema, ToolInputSchemaProperty, ToolInputSchemaPropertyItems};
use std::collections::HashMap;
use crate::types::jsonrpc_message::JsonRpcMessageObject;

#[derive(Debug, Clone)]
pub struct McpServerTool {
    pub name: String,
    pub description: Option<String>,
    pub properties: Vec<McpServerToolPropertyInfo>,
    pub function: fn(HashMap<String, McpServerToolPropertyValue>) -> Option<serde_json::Value>
}

impl McpServerTool {
    pub(crate) fn to_tool_info(&self) -> ToolInfo {
        ToolInfo {
            name: self.name.clone(),
            description: self.description.clone(),
            input_schema: ToolInputSchema {
                input_type: "object".to_string(),
                properties: self.properties.iter().map(|p| {
                    (p.identifier.clone(), {
                        let mut property_type = None;
                        let mut any_of_property_types = None;
                        let mut items = None;
                        
                        match &p.property_type {
                            McpServerToolPropertyType::Null => {
                                property_type = Some("null".to_string());
                            },
                            McpServerToolPropertyType::String => {
                                property_type = Some("string".to_string());
                            },
                            McpServerToolPropertyType::Array(items_type) => {
                                property_type = Some("array".to_string());

                                items = Some(ToolInputSchemaPropertyItems {
                                    property_type: match items_type {
                                        McpServerToolArrayPropertyInnerType::Null => Some("null".to_string()),
                                        McpServerToolArrayPropertyInnerType::String => Some("string".to_string()),
                                        McpServerToolArrayPropertyInnerType::Number => Some("number".to_string()),
                                        // TODO: McpServerToolArrayPropertyInnerType::AnyOf(_) => None
                                    }
                                });
                            },
                            McpServerToolPropertyType::Number => {
                                property_type = Some("number".to_string());
                            },
                            McpServerToolPropertyType::AnyOf(types) => {
                                any_of_property_types = Some(
                                    types.iter()
                                        .filter_map(|t| {
                                            let mut items = None;
                                            
                                            let property_type = match t {
                                                McpServerToolAnyOfPropertyType::Null => Some("null".to_string()),
                                                McpServerToolAnyOfPropertyType::String => Some("string".to_string()),
                                                McpServerToolAnyOfPropertyType::Array(items_type) => {
                                                    items = Some(ToolInputSchemaPropertyItems {
                                                        property_type: match items_type {
                                                            McpServerToolArrayPropertyInnerType::Null => Some("null".to_string()),
                                                            McpServerToolArrayPropertyInnerType::String => Some("string".to_string()),
                                                            McpServerToolArrayPropertyInnerType::Number => Some("number".to_string()),
                                                            // TODO: McpServerToolArrayPropertyInnerType::AnyOf(_) => None
                                                        }
                                                    });
                                                    
                                                    Some("array".to_string())
                                                },
                                                McpServerToolAnyOfPropertyType::Number => Some("number".to_string())
                                            };
                                            
                                            Some(ToolInputSchemaProperty {
                                                property_type,
                                                any_of_property_types: None,
                                                description: None,
                                                items,
                                                title: None,
                                                enum_values: None,
                                                default_value: JsonRpcMessageObject::Null,
                                            })
                                        })
                                        .collect()
                                );
                            }
                        };

                        let default_value = match serde_json::to_string(&p.default_value) {
                            Ok(default_value_as_string) => serde_json::from_str(
                                default_value_as_string.as_str()
                            ).unwrap_or_else(|_| JsonRpcMessageObject::Null),
                            Err(_) => JsonRpcMessageObject::Null
                        };
                        
                        ToolInputSchemaProperty {
                            property_type,
                            any_of_property_types,
                            description: p.description.clone(),
                            items,
                            title: None,
                            enum_values: p.enum_values.clone(),
                            default_value
                        }
                    })
                }).collect(),
                required_property_names: Some(self.properties.iter().filter_map(|p| {
                    if p.is_required {
                        Some(p.identifier.clone())
                    } else {
                        None
                    }
                }).collect())
            },
            annotations: None
        }
    }
}