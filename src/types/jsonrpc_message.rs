use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const JSONRPC_VERSION: &str = "2.0";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonRpcMessage {
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub id: Option<JsonRpcMessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub result: JsonRpcMessageObject,
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub params: JsonRpcMessageObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcMessageError>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcMessageId {
    Integer(i64),
    Text(String)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonRpcMessageError {
    pub code: i64,
    pub message: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum JsonRpcMessageObject {
    Dictionary(HashMap<String, JsonRpcMessageObject>),
    Array(Vec<JsonRpcMessageObject>),
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null
}

impl JsonRpcMessageObject {
    pub fn is_null(&self) -> bool {
        self == &JsonRpcMessageObject::Null
    }
}

impl Default for JsonRpcMessageObject {
    fn default() -> Self {
        JsonRpcMessageObject::Null
    }
}