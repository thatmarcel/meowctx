use crate::mcp_server::McpServer;
use crate::mcp_server_tool::McpServerTool;

pub struct McpServerBuilder {
    display_name: String,
    display_version: String,
    tools: Vec<McpServerTool>
}

impl McpServerBuilder {
    pub fn with_name_and_version(name: String, version: String) -> Self {
        Self {
            display_name: name,
            display_version: version,
            tools: vec![]
        }
    }
    
    pub fn build(self) -> McpServer {
        McpServer {
            display_name: self.display_name,
            display_version: self.display_version,
            tools: self.tools
        }
    }
    
    pub fn add_tool(mut self, tool: McpServerTool) -> Self {
        self.tools.push(tool);
        self
    }
}