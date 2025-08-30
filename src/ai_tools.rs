use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::env;

/// Represents an AI coding CLI tool
pub struct AiTool {
    /// Name of the AI tool
    pub name: String,
    /// Command to execute
    pub command: String,
    /// Default CLI arguments
    pub default_args: Vec<String>,
    /// Environment variable to override the command
    pub env_var: String,
}

/// Available AI coding CLI tools in order of preference
pub enum AiToolType {
    OpenCode,
    QwenCode,
    Claude,
}

impl AiToolType {
    /// Get the corresponding AiTool configuration
    pub fn get_config(&self) -> AiTool {
        match self {
            AiToolType::OpenCode => AiTool {
                name: "OpenCode".to_string(),
                command: "opencode".to_string(),
                default_args: vec![], // OpenCode doesn't need special flags by default
                env_var: "XLAUDE_OPENCODE_CMD".to_string(),
            },
            AiToolType::QwenCode => AiTool {
                name: "Qwen Code".to_string(),
                command: "qwen".to_string(),
                default_args: vec![], // Qwen doesn't need special flags by default
                env_var: "XLAUDE_QWEN_CMD".to_string(),
            },
            AiToolType::Claude => AiTool {
                name: "Claude".to_string(),
                command: "claude".to_string(),
                default_args: vec!["--dangerously-skip-permissions".to_string()],
                env_var: "XLAUDE_CLAUDE_CMD".to_string(),
            },
        }
    }
}

/// Check if a command is available in the system PATH
fn is_command_available(command: &str) -> bool {
    let cmd_check = if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    } else {
        Command::new("which")
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    };
    
    match cmd_check {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

/// Attempts to find the first available AI tool
/// 
/// Returns the first available tool based on priority:
/// 1. OpenCode
/// 2. Qwen Code
/// 3. Claude
///
/// If none are found, returns None
pub fn find_available_tool() -> Option<AiTool> {
    // Priority order of tools to try
    let tools = [
        AiToolType::OpenCode,
        AiToolType::QwenCode, 
        AiToolType::Claude,
    ];
    
    for tool_type in tools.iter() {
        let tool = tool_type.get_config();
        
        // Check if the tool is explicitly set via environment variable
        if let Ok(custom_cmd) = env::var(&tool.env_var) {
            if !custom_cmd.is_empty() {
                return Some(AiTool {
                    command: custom_cmd,
                    ..tool
                });
            }
        }
        
        // Check if the default command is available
        if is_command_available(&tool.command) {
            return Some(tool);
        }
    }
    
    None
}

/// Launch an AI tool with the specified command and arguments
pub fn launch_ai_tool(tool: &AiTool, stdin_mode: StdinMode) -> Result<()> {
    let mut cmd = Command::new(&tool.command);
    
    // Add default arguments if using the standard command
    if tool.command == tool_type_to_default_cmd(&tool.name) {
        for arg in &tool.default_args {
            cmd.arg(arg);
        }
    }
    
    // Inherit all environment variables
    cmd.envs(env::vars());
    
    // Handle stdin based on mode
    match stdin_mode {
        StdinMode::Inherit => {
            // Do nothing, inherit stdin
        },
        StdinMode::Null => {
            cmd.stdin(Stdio::null());
        },
    }
    
    let status = cmd.status()
        .with_context(|| format!("Failed to launch {}", tool.name))?;
    
    if !status.success() {
        anyhow::bail!("{} exited with error", tool.name);
    }
    
    Ok(())
}

/// Map a tool type name to its default command
fn tool_type_to_default_cmd(name: &str) -> &str {
    match name {
        "OpenCode" => "opencode",
        "Qwen Code" => "qwen",
        "Claude" => "claude",
        _ => "",
    }
    // Future Options: Work vs Play
    // Work: Cursor + Claude, VSCode + OpenAI, openCode + Bedrock, Vertex AI
    // Play: OpenCode, Qwen, Claude, Kimi K2, Gemini, GHT 4.5, DeepSeek, Kilo, Cline, Trae
}

/// Stdin handling mode for AI tool execution
pub enum StdinMode {
    /// Inherit stdin from parent process
    Inherit,
    /// Don't pass stdin to the AI tool
    Null,
}
