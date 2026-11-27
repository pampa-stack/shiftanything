//! The configuration an MCP client needs to start the `craft-ai-mcp` gateway, in each format
//! clients use, for Settings › AI agents › Copy. Every format names the same one server.

use serde_json::{Value, json};

/// The name the gateway's entry has in a client's configuration. Clients show it as the
/// integration's name, so it is the suite's name.
pub const SERVER_KEY: &str = "ShiftAnything";

/// A configuration format MCP clients read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpConfigFormat {
    /// `{"mcpServers": {...}}`: Claude Desktop, Cursor, LM Studio, Windsurf and most others.
    Json,
    /// VS Code's `mcp.json`: `{"servers": {...}}` with `"type": "stdio"`.
    VsCode,
    /// Codex's `config.toml`: a `[mcp_servers.<name>]` table.
    CodexToml,
}

impl McpConfigFormat {
    /// Every format, in the order Settings offers them.
    pub const ALL: [McpConfigFormat; 3] = [McpConfigFormat::Json, McpConfigFormat::VsCode, McpConfigFormat::CodexToml];
}

/// A TOML basic string: JSON's string escapes (`\\`, `\"`, `\uXXXX`) are valid TOML too.
fn toml_string(s: &str) -> String {
    Value::String(s.to_string()).to_string()
}

/// The configuration that starts `gateway --stdio` as [`SERVER_KEY`], in `format`.
pub fn mcp_config(format: McpConfigFormat, gateway: &str) -> String {
    match format {
        McpConfigFormat::Json => {
            let v = json!({"mcpServers": {SERVER_KEY: {"command": gateway, "args": ["--stdio"]}}});
            serde_json::to_string_pretty(&v).unwrap_or_default()
        }
        McpConfigFormat::VsCode => {
            let v = json!({"servers": {SERVER_KEY: {"type": "stdio", "command": gateway, "args": ["--stdio"]}}});
            serde_json::to_string_pretty(&v).unwrap_or_default()
        }
        McpConfigFormat::CodexToml => {
            format!("[mcp_servers.{SERVER_KEY}]\ncommand = {}\nargs = [\"--stdio\"]\n", toml_string(gateway))
        }
    }
}

/// Format the scoped catalog server's exact command and roots for each supported client.
pub fn catalog_config(format: McpConfigFormat, configuration: &str) -> Result<String, String> {
    let config: Value = serde_json::from_str(configuration).map_err(|error| error.to_string())?;
    let entry = &config["mcpServers"]["shiftanything"];
    let command = entry["command"].as_str().ok_or("missing catalog server command")?;
    let args = entry["args"].as_array().ok_or("missing catalog server arguments")?;
    if args.iter().any(|arg| !arg.is_string()) {
        return Err("invalid catalog server arguments".into());
    }
    let output = match format {
        McpConfigFormat::Json => json!({"mcpServers": {"shiftanything": entry}}),
        McpConfigFormat::VsCode => json!({"servers": {"shiftanything": {"type": "stdio", "command": command, "args": args}}}),
        McpConfigFormat::CodexToml => {
            return Ok(format!(
                "[mcp_servers.shiftanything]\ncommand = {}\nargs = [{}]\n",
                toml_string(command),
                args.iter().map(Value::to_string).collect::<Vec<_>>().join(", ")
            ));
        }
    };
    serde_json::to_string_pretty(&output).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GW: &str = r"C:\Users\a b\AppData\Local\ArtCraft\mcp\bin\craft-ai-mcp.exe";

    #[test]
    fn json_formats_name_the_gateway_with_stdio() {
        let v: Value = serde_json::from_str(&mcp_config(McpConfigFormat::Json, GW)).unwrap();
        assert_eq!(v["mcpServers"][SERVER_KEY]["command"], GW);
        assert_eq!(v["mcpServers"][SERVER_KEY]["args"], json!(["--stdio"]));
        let v: Value = serde_json::from_str(&mcp_config(McpConfigFormat::VsCode, GW)).unwrap();
        assert_eq!(v["servers"][SERVER_KEY]["type"], "stdio");
        assert_eq!(v["servers"][SERVER_KEY]["command"], GW);
        assert_eq!(v["servers"][SERVER_KEY]["args"], json!(["--stdio"]));
    }

    #[test]
    fn codex_toml_escapes_the_path() {
        let t = mcp_config(McpConfigFormat::CodexToml, GW);
        assert!(t.starts_with("[mcp_servers.ShiftAnything]\n"), "{t}");
        assert!(t.contains(r#"command = "C:\\Users\\a b\\AppData\\Local\\ArtCraft\\mcp\\bin\\craft-ai-mcp.exe""#), "{t}");
        assert!(t.contains("args = [\"--stdio\"]"), "{t}");
        let quoted = mcp_config(McpConfigFormat::CodexToml, "/opt/a\"b/craft-ai-mcp");
        assert!(quoted.contains(r#"command = "/opt/a\"b/craft-ai-mcp""#), "{quoted}");
    }
    #[test]
    fn catalog_formats_preserve_scoped_arguments_with_spaces_and_quotes() {
        let args = json!([
            "run",
            "C:\\Test profile\\a\"b.py",
            "--automation-read-root",
            "/tmp/profile with spaces",
            "--automation-write-root",
            "/tmp/profile with spaces"
        ]);
        let input = json!({"mcpServers": {"shiftanything": {"command": "uv", "args": args}}}).to_string();
        let json: Value = serde_json::from_str(&catalog_config(McpConfigFormat::Json, &input).unwrap()).unwrap();
        let vscode: Value = serde_json::from_str(&catalog_config(McpConfigFormat::VsCode, &input).unwrap()).unwrap();
        let toml: toml::Value = toml::from_str(&catalog_config(McpConfigFormat::CodexToml, &input).unwrap()).unwrap();
        assert_eq!(json["mcpServers"]["shiftanything"]["args"], args);
        assert_eq!(vscode["servers"]["shiftanything"]["args"], args);
        let toml_args: Vec<&str> =
            toml["mcp_servers"]["shiftanything"]["args"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
        let expected: Vec<&str> = args.as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
        assert_eq!(toml_args, expected);
        assert_eq!(vscode["servers"]["shiftanything"]["type"], "stdio");
    }
}
