use crate::mcp_server_tool_property::{McpServerToolAnyOfPropertyType, McpServerToolArrayPropertyInnerType, McpServerToolPropertyInfo, McpServerToolPropertyType};
use crate::types::openapi::{OpenApiComponentsSchemaArrayPropertyItemType, OpenApiComponentsSchemaArrayPropertyItems, OpenApiComponentsSchemaProperty};
use crate::utils::string_capitalizable::Capitalizable;

pub fn convert_mcp_to_openapi_schema_property(mcp_property: &McpServerToolPropertyInfo) -> OpenApiComponentsSchemaProperty {
    match &mcp_property.property_type {
        McpServerToolPropertyType::Null => {
            OpenApiComponentsSchemaProperty::Null
        },
        McpServerToolPropertyType::String => {
            OpenApiComponentsSchemaProperty::String {
                title: mcp_property.identifier.ascii_capitalized(),
                description: mcp_property.description.clone()
            }
        },
        McpServerToolPropertyType::Array(inner_type) => {
            OpenApiComponentsSchemaProperty::Array {
                title: mcp_property.identifier.ascii_capitalized(),
                description: mcp_property.description.clone(),
                items: OpenApiComponentsSchemaArrayPropertyItems::SingleType(
                    match inner_type {
                        McpServerToolArrayPropertyInnerType::Null => {
                            OpenApiComponentsSchemaArrayPropertyItemType::Null
                        },
                        McpServerToolArrayPropertyInnerType::String => {
                            OpenApiComponentsSchemaArrayPropertyItemType::String
                        },
                        McpServerToolArrayPropertyInnerType::Number => {
                            OpenApiComponentsSchemaArrayPropertyItemType::Number
                        }
                    }
                )
            }
        },
        McpServerToolPropertyType::Number => {
            OpenApiComponentsSchemaProperty::Number {
                title: mcp_property.identifier.ascii_capitalized(),
                description: mcp_property.description.clone()
            }
        }
        McpServerToolPropertyType::AnyOf(inner_types) => {
            OpenApiComponentsSchemaProperty::Array {
                title: mcp_property.identifier.ascii_capitalized(),
                description: mcp_property.description.clone(),
                items: OpenApiComponentsSchemaArrayPropertyItems::AnyOf(
                    inner_types.iter().map(|inner_type| {
                        match inner_type {
                            McpServerToolAnyOfPropertyType::Null => {
                                OpenApiComponentsSchemaArrayPropertyItemType::Null
                            },
                            McpServerToolAnyOfPropertyType::String => {
                                OpenApiComponentsSchemaArrayPropertyItemType::String
                            },
                            McpServerToolAnyOfPropertyType::Number => {
                                OpenApiComponentsSchemaArrayPropertyItemType::Number
                            },
                            McpServerToolAnyOfPropertyType::Array(inner_inner_type) => {
                                OpenApiComponentsSchemaArrayPropertyItemType::Array {
                                    items: Some(Box::new(match inner_inner_type {
                                        McpServerToolArrayPropertyInnerType::Null => {
                                            OpenApiComponentsSchemaArrayPropertyItemType::Null
                                        },
                                        McpServerToolArrayPropertyInnerType::String => {
                                            OpenApiComponentsSchemaArrayPropertyItemType::String
                                        },
                                        McpServerToolArrayPropertyInnerType::Number => {
                                            OpenApiComponentsSchemaArrayPropertyItemType::Number
                                        }
                                    }))
                                }
                            }
                        }
                    }).collect()
                )
            }
        }
    }
}