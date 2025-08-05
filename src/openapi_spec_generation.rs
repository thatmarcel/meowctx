use crate::mcp_server::McpServer;
use crate::types::openapi::{OpenApiApplicationInfo, OpenApiComponents, OpenApiComponentsSchemaProperty, OpenApiPath, OpenApiPostPath, OpenApiPostPathRequestBody, OpenApiPostPathRequestBodyContent, OpenApiPostPathRequestBodyContentApplicationJson, OpenApiPostPathRequestBodyContentApplicationJsonSchema, OpenApiPostPathResponse, OpenApiPostPathResponseContent, OpenApiPostPathResponseContentApplicationJson, OpenApiPostPathResponseContentApplicationJsonSchema, OpenApiSpec};
use crate::utils::string_capitalizable::Capitalizable;
use std::collections::HashMap;
use crate::openapi_mcp_property_conversion::convert_mcp_to_openapi_schema_property;

pub fn generate_openapi_spec_for_mcp_server(mcp_server: &McpServer) -> OpenApiSpec {
    let mut openapi_paths: HashMap<String, OpenApiPath> = HashMap::new();

    let mut openapi_components_schemas: HashMap<String, OpenApiComponentsSchemaProperty> = HashMap::new();

    for tool in mcp_server.tools.iter() {
        let tool_path_name = format!("/{}", tool.name);

        let tool_summary = tool.name.split("_").map(|p| {
            p.ascii_capitalized()
        }).collect::<Vec<String>>().join(" ");

        let successful_tool_response_title = format!("Response Tool {}", tool_summary);

        let mut tool_responses_for_status_codes: HashMap<String, OpenApiPostPathResponse> = HashMap::new();
        tool_responses_for_status_codes.insert("200".to_string(), OpenApiPostPathResponse {
            description: "Successful Response".to_string(),
            content: OpenApiPostPathResponseContent {
                application_json: OpenApiPostPathResponseContentApplicationJson {
                    schema: OpenApiPostPathResponseContentApplicationJsonSchema::Title(
                        successful_tool_response_title
                    )
                }
            },
        });
        // TODO: Status code 422

        let component_name = format!("{}_form_model", tool.name);

        openapi_paths.insert(tool_path_name, OpenApiPath {
            post: OpenApiPostPath {
                summary: tool_summary.clone(),
                description: tool.description.clone().unwrap_or(tool_summary),
                operation_id: tool.name.clone(),
                request_body: OpenApiPostPathRequestBody {
                    content: OpenApiPostPathRequestBodyContent {
                        application_json: OpenApiPostPathRequestBodyContentApplicationJson {
                            schema: OpenApiPostPathRequestBodyContentApplicationJsonSchema {
                                reference_path: format!("#/components/schemas/{}", component_name)
                            }
                        }
                    },
                    is_required: !tool.properties.is_empty()
                },
                responses_for_status_codes: tool_responses_for_status_codes
            }
        });

        let mut tool_properties: HashMap<String, OpenApiComponentsSchemaProperty> = HashMap::with_capacity(tool.properties.len());
        let mut tool_required_properties: Vec<String> = Vec::with_capacity(tool.properties.len());

        for mcp_property in tool.properties.iter() {
            tool_properties.insert(
                mcp_property.identifier.clone(),
                convert_mcp_to_openapi_schema_property(mcp_property)
            );

            if mcp_property.is_required {
                tool_required_properties.push(mcp_property.identifier.clone());
            }
        }

        openapi_components_schemas.insert(component_name.clone(), OpenApiComponentsSchemaProperty::Object {
            title: component_name,
            description: None,
            properties: tool_properties,
            required: Some(tool_required_properties)
        });
    }

    let openapi_components: OpenApiComponents = OpenApiComponents {
        schemas: openapi_components_schemas
    };

    OpenApiSpec {
        spec_version: "3.1.0".to_string(),
        info: OpenApiApplicationInfo {
            title: mcp_server.display_name.clone(),
            description: format!("{} MCP Server", mcp_server.display_name),
            version: mcp_server.display_version.clone()
        },
        paths: openapi_paths,
        components: openapi_components
    }
}