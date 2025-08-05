use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    #[serde(rename = "openapi")]
    pub spec_version: String,
    pub info: OpenApiApplicationInfo,
    pub paths: HashMap<String, OpenApiPath>,
    pub components: OpenApiComponents
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiComponents {
    pub schemas: HashMap<String, OpenApiComponentsSchemaProperty>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OpenApiComponentsSchemaProperty {
    #[serde(rename = "object")]
    Object {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        properties: HashMap<String, OpenApiComponentsSchemaProperty>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<Vec<String>>
    },
    #[serde(rename = "array")]
    Array {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        items: OpenApiComponentsSchemaArrayPropertyItems
    },
    #[serde(rename = "string")]
    String {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>
    },
    #[serde(rename = "number")]
    Number {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>
    },
    #[serde(rename = "integer")]
    Integer {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>
    },
    #[serde(rename = "boolean")]
    Boolean {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        title: String
    },
    #[serde(rename = "null")]
    Null
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpenApiComponentsSchemaArrayPropertyItems {
    #[serde(rename = "$ref")]
    ReferencePath(String),
    #[serde(rename = "anyOf")]
    AnyOf(Vec<OpenApiComponentsSchemaArrayPropertyItemType>),
    #[serde(untagged)]
    SingleType(OpenApiComponentsSchemaArrayPropertyItemType)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OpenApiComponentsSchemaArrayPropertyItemType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array {
        #[serde(skip_serializing_if = "Option::is_none")]
        items: Option<Box<OpenApiComponentsSchemaArrayPropertyItemType>>
    },
    #[serde(rename = "null")]
    Null
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiApplicationInfo {
    pub title: String,
    pub description: String,
    pub version: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPath {
    pub post: OpenApiPostPath
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPath {
    pub summary: String,
    pub description: String,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(rename = "requestBody")]
    pub request_body: OpenApiPostPathRequestBody,
    #[serde(rename = "responses")]
    pub responses_for_status_codes: HashMap<String, OpenApiPostPathResponse>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathResponse {
    pub description: String,
    pub content: OpenApiPostPathResponseContent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathRequestBody {
    pub content: OpenApiPostPathRequestBodyContent,
    #[serde(rename = "required")]
    pub is_required: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathRequestBodyContent {
    #[serde(rename = "application/json")]
    pub application_json: OpenApiPostPathRequestBodyContentApplicationJson
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathRequestBodyContentApplicationJson {
    pub schema: OpenApiPostPathRequestBodyContentApplicationJsonSchema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathRequestBodyContentApplicationJsonSchema {
    #[serde(rename = "$ref")]
    pub reference_path: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathResponseContent {
    #[serde(rename = "application/json")]
    pub application_json: OpenApiPostPathResponseContentApplicationJson
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiPostPathResponseContentApplicationJson {
    pub schema: OpenApiPostPathResponseContentApplicationJsonSchema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpenApiPostPathResponseContentApplicationJsonSchema {
    #[serde(rename = "$ref")]
    ReferencePath(String),
    #[serde(rename = "title")]
    Title(String)
}