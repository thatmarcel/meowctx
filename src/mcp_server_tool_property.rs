use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct McpServerToolPropertyInfo {
    pub identifier: String,
    pub property_type: McpServerToolPropertyType,
    pub is_required: bool,
    pub description: Option<String>,
    pub enum_values: Option<Vec<String>>,
    pub default_value: Option<McpServerToolPropertyValue>
}

#[derive(Clone, Debug)]
pub enum McpServerToolPropertyType {
    Null,
    String,
    Array(McpServerToolArrayPropertyInnerType),
    Number,
    AnyOf(Vec<McpServerToolAnyOfPropertyType>)
}

#[derive(Clone, Debug)]
pub enum McpServerToolArrayPropertyInnerType {
    Null,
    String,
    Number,
    // TODO: AnyOf(Vec<McpServerToolPropertyType>)
}

#[derive(Clone, Debug)]
pub enum McpServerToolAnyOfPropertyType {
    Null,
    String,
    Array(McpServerToolArrayPropertyInnerType),
    Number
}

impl ToString for McpServerToolPropertyType {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpServerToolPropertyValue {
    String(String),
    Array(Vec<String>),
    Number
}