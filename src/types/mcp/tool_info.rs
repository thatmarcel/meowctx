use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;

#[derive(Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>
}

#[derive(Serialize, Deserialize)]
pub struct ToolAnnotations {
    pub title: Option<String>,
    #[serde(rename = "readOnlyHint", skip_serializing_if = "Option::is_none")]
    pub read_only_hint: Option<bool>,
    #[serde(rename = "destructiveHint", skip_serializing_if = "Option::is_none")]
    pub destructive_hint: Option<bool>,
    #[serde(rename = "idempotentHint", skip_serializing_if = "Option::is_none")]
    pub idempotent_hint: Option<bool>,
    #[serde(rename = "openWorldHint", skip_serializing_if = "Option::is_none")]
    pub open_world_hint: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct ToolInputSchema {
    #[serde(rename = "type")]
    pub input_type: String, // "object"
    pub properties: HashMap<String, ToolInputSchemaProperty>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required_property_names: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct ToolInputSchemaProperty {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<String>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of_property_types: Option<Vec<ToolInputSchemaProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ToolInputSchemaPropertyItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(rename = "default", skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub default_value: JsonRpcMessageObject
}

#[derive(Serialize, Deserialize)]
pub struct ToolInputSchemaPropertyItems {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<String>
}