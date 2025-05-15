use serde::{Deserialize, Serialize};
use crate::types::jsonrpc_message::JsonRpcMessageObject;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(TextContent),
    Image(ImageContent),
    Audio,
    Embedded
}

#[derive(Serialize, Deserialize)]
pub struct TextContent {
    #[serde(rename = "type")]
    pub content_type: String, // "text"
    pub text: String
}

#[derive(Serialize, Deserialize)]
pub struct ImageContent {
    #[serde(rename = "type")]
    pub content_type: String, // "image"
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String
}

#[derive(Serialize, Deserialize)]
pub struct AudioContent {
    #[serde(rename = "type")]
    pub content_type: String, // "audio"
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedContent {
    #[serde(rename = "type")]
    pub content_type: String, // "resource"
    #[serde(default)]
    pub resource: JsonRpcMessageObject
}