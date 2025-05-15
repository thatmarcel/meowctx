use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;

#[derive(Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub experimental: JsonRpcMessageObject,
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub sampling: JsonRpcMessageObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<CapabilitiesRoots>
}

#[derive(Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub experimental: JsonRpcMessageObject,
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub logging: JsonRpcMessageObject,
    #[serde(default, skip_serializing_if = "JsonRpcMessageObject::is_null")]
    pub completions: JsonRpcMessageObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<CapabilitiesPrompts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<CapabilitiesResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<CapabilitiesTools>
}

#[derive(Serialize, Deserialize)]
pub struct CapabilitiesRoots {
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct CapabilitiesPrompts {
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct CapabilitiesResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct CapabilitiesTools {
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>
}