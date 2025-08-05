use crate::mcp_server::McpServer;
use crate::openapi_spec_generation::generate_openapi_spec_for_mcp_server;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::collections::HashMap;
use crate::mcp_server_tool_property::McpServerToolPropertyValue;

pub async fn serve_openapi(mcp_server: &McpServer) {
    let openapi_spec = generate_openapi_spec_for_mcp_server(&mcp_server);
    let openapi_spec_json = Json(openapi_spec.clone());

    let mut app = Router::new()
        .route("/openapi.json", get(async || {
            openapi_spec_json
        }));

    for (path_string, path_info) in openapi_spec.paths.clone() {
        let mcp_tool = mcp_server.tools
            .iter()
            .filter(|t| t.name == path_info.post.operation_id)
            .next()
            .unwrap()
            .clone();

        app = app.route(&path_string.clone(), post(async move |body: Json<serde_json::Value>| {
            let params = body.as_object().unwrap();

            let arguments: HashMap<String, McpServerToolPropertyValue> = params.iter().filter_map(|(argument_property_identifier, argument_property_value)| {
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

            let result = (mcp_tool.function)(arguments);

            Json(result.unwrap())
        }));
    }

    let port = std::env::var("PORT").unwrap_or("4000".to_string());
    let addr = format!("0.0.0.0:{}",port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on {}", &addr);
    println!("(=> http://127.0.0.1:{}/openapi.json)", port);

    axum::serve(listener, app).await.unwrap();
}