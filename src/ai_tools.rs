use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::env;
use colored::Colorize;

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
    Zed,  // Zed IDE with Gemini CLI integration
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
            AiToolType::Zed => AiTool {
                name: "Zed IDE".to_string(),
                command: "zed".to_string(),
                default_args: vec![], // Zed will use Gemini CLI integration automatically
                env_var: "XLAUDE_ZED_CMD".to_string(),
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
/// 3. Zed IDE (with Gemini CLI)
/// 4. Claude
///
/// If none are found, returns None
pub fn find_available_tool() -> Option<AiTool> {
    println!("{} Searching for available AI tools...", "🔍".cyan());
    
    // Check if user prefers Gemini (Zed) first
    if env::var("XLAUDE_PREFER_GEMINI").is_ok() {
        println!("{} XLAUDE_PREFER_GEMINI is set, prioritizing Zed IDE", "⚡".yellow());
        let zed_tool = AiToolType::Zed.get_config();
        
        // Check if Zed is explicitly set via environment variable
        if let Ok(custom_cmd) = env::var(&zed_tool.env_var) {
            if !custom_cmd.is_empty() {
                println!("{} Using custom Zed command from {}: {}", "🔧".blue(), zed_tool.env_var, custom_cmd);
                return Some(AiTool {
                    command: custom_cmd,
                    ..zed_tool
                });
            }
        }
        
        // Check if Zed is available
        if is_command_available(&zed_tool.command) {
            println!("{} Found Zed IDE: {}", "✅".green(), zed_tool.command);
            return Some(zed_tool);
        } else {
            println!("{} Zed IDE not found: {}", "❌".red(), zed_tool.command);
        }
    } else {
        println!("{} XLAUDE_PREFER_GEMINI not set, using default priority order", "📋".blue());
    }
    
    // Default priority order of tools to try
    let tools = [
        AiToolType::OpenCode,
        AiToolType::QwenCode, 
        AiToolType::Zed,
        AiToolType::Claude,
    ];
    
    for tool_type in tools.iter() {
        let tool = tool_type.get_config();
        println!("{} Checking {}: {}", "🔎".cyan(), tool.name, tool.command);
        
        // Check if the tool is explicitly set via environment variable
        if let Ok(custom_cmd) = env::var(&tool.env_var) {
            if !custom_cmd.is_empty() {
                println!("{} Using custom command from {}: {}", "🔧".blue(), tool.env_var, custom_cmd);
                return Some(AiTool {
                    command: custom_cmd,
                    ..tool
                });
            }
        }
        
        // Check if the default command is available
        if is_command_available(&tool.command) {
            println!("{} Found {}: {}", "✅".green(), tool.name, tool.command);
            return Some(tool);
        } else {
            println!("{} {} not found: {}", "❌".red(), tool.name, tool.command);
        }
    }
    
    println!("{} No AI tools found", "⚠️".yellow());
    None
}


/// Launch an AI tool with the specified command and arguments, optionally with a specific path
pub fn launch_ai_tool_with_path(tool: &AiTool, stdin_mode: StdinMode, worktree_path: Option<std::path::PathBuf>) -> Result<()> {
    let mut cmd = Command::new(&tool.command);
    
    // Add default arguments if using the standard command
    if tool.command == tool_type_to_default_cmd(&tool.name) {
        for arg in &tool.default_args {
            cmd.arg(arg);
        }
    }
    
    // For Zed, pass the worktree path as an argument to ensure it opens the correct directory
    if tool.name == "Zed IDE" {
        let target_path = if let Some(path) = worktree_path {
            // Use the provided worktree path
            path
        } else {
            // Fall back to current directory
            std::env::current_dir().context("Failed to get current directory")?
        };
        
        let path_str = target_path.to_string_lossy();
        println!("{} Opening Zed at worktree path: {}", "📁".green(), path_str);
        
        // Pass the worktree path as an argument to zed command
        cmd.arg(&target_path);
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
    
    // Print the full command being executed
    let cmd_str = format!("{} {}", 
        tool.command, 
        cmd.get_args().map(|os| os.to_string_lossy()).collect::<Vec<_>>().join(" ")
    );
    println!("{} Executing: {}", "🚀".yellow(), cmd_str);
    
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
        "Zed IDE" => "zed",
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
