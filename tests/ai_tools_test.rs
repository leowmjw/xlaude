use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command as StdCommand;
use tempfile::TempDir;

// Set up a mock git repository with a worktree
fn setup_git_repo_with_worktree() -> (TempDir, TempDir) {
    // Create main repo
    let repo_dir = TempDir::new().unwrap();
    
    // Initialize git repo
    StdCommand::new("git")
        .args(["init"])
        .current_dir(repo_dir.path())
        .output()
        .unwrap();
    
    // Create an initial commit
    fs::write(repo_dir.path().join("README.md"), "# Test Repository").unwrap();
    
    StdCommand::new("git")
        .args(["config", "--local", "user.email", "test@example.com"])
        .current_dir(repo_dir.path())
        .output()
        .unwrap();
    
    StdCommand::new("git")
        .args(["config", "--local", "user.name", "Test User"])
        .current_dir(repo_dir.path())
        .output()
        .unwrap();
        
    StdCommand::new("git")
        .args(["add", "README.md"])
        .current_dir(repo_dir.path())
        .output()
        .unwrap();
        
    StdCommand::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_dir.path())
        .output()
        .unwrap();
    
    // Create a worktree
    let worktree_dir = TempDir::new().unwrap();
    let worktree_name = "test-feature";
    
    StdCommand::new("git")
        .args(["worktree", "add", "-b", worktree_name, worktree_dir.path().to_str().unwrap()])
        .current_dir(repo_dir.path())
        .output()
        .unwrap();
    
    // Create the xlaude state directory if it doesn't exist
    let home_dir = env::var("HOME").unwrap();
    let state_dir = Path::new(&home_dir).join(".local/share/xlaude");
    fs::create_dir_all(&state_dir).unwrap_or_default();
    
    // Register the worktree with xlaude
    Command::cargo_bin("xlaude")
        .unwrap()
        .arg("add")
        .arg(worktree_name)
        .current_dir(worktree_dir.path())
        .env("XLAUDE_YES", "1")
        .output()
        .unwrap();
    
    (repo_dir, worktree_dir)
}

// Mock an AI tool for testing by creating a dummy executable
fn create_mock_ai_tool(temp_dir: &TempDir, name: &str) -> String {
    let tool_path = temp_dir.path().join(name);
    
    #[cfg(not(target_os = "windows"))]
    {
        use std::fs::{self, File};
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;

        let content = r#"#!/bin/sh
echo "Mock AI tool: $0 $*"
echo "MOCK_AI_TOOL_EXECUTED=$0" >&2
exit 0
"#;

        let mut file = File::create(&tool_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        drop(file);

        let mut perms = fs::metadata(&tool_path).unwrap().permissions();
        perms.set_mode(0o755);  // Set executable permission
        fs::set_permissions(&tool_path, perms).unwrap();
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::fs::File;
        use std::io::Write;
        
        let bat_path = temp_dir.path().join(format!("{}.bat", name));
        let content = format!(
            "@echo off\r\necho Mock AI tool: %0 %*\r\necho MOCK_AI_TOOL_EXECUTED=%0 1>&2\r\nexit /b 0\r\n"
        );
        
        let mut file = File::create(&bat_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
    
    temp_dir.path().to_string_lossy().to_string()
}

#[test]
fn test_opencode_priority() {
    // Set up git repo with worktree
    let (_repo_dir, worktree_dir) = setup_git_repo_with_worktree();
    
    // Create a temporary directory for mock tools
    let temp_dir = TempDir::new().unwrap();
    let mock_path = create_mock_ai_tool(&temp_dir, "opencode");
    
    // Add temp directory to PATH
    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}{}{}", mock_path, if cfg!(windows) { ";" } else { ":" }, original_path);
    
    // Run xlaude open with PATH set to include our mock tool
    let assert = Command::cargo_bin("xlaude")
        .unwrap()
        .arg("open")
        .arg("test-feature") // Specify the worktree name
        .current_dir(worktree_dir.path())
        .env("PATH", new_path)
        .env("XLAUDE_MOCK_TEST", "1") // Signal that we're in test mode
        .env("XLAUDE_MOCK_OUTPUT", "1") // Capture mock tool execution
        .env("XLAUDE_YES", "1") // Auto-confirm prompts
        .env("XLAUDE_NON_INTERACTIVE", "1") // Disable interactive mode
        .assert();
    
    // Check that the command mentions using OpenCode
    assert
        .stderr(predicate::str::contains("MOCK_AI_TOOL_EXECUTED"))
        .stderr(predicate::str::contains("opencode"));
}

#[test]
fn test_ai_tool_fallback() {
    // Set up git repo with worktree
    let (_repo_dir, worktree_dir) = setup_git_repo_with_worktree();
    
    // Create a temporary directory for mock tools
    let temp_dir = TempDir::new().unwrap();
    
    // Only create qwen, not opencode
    let mock_path = create_mock_ai_tool(&temp_dir, "qwen");
    
    // Add temp directory to PATH
    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}{}{}", mock_path, if cfg!(windows) { ";" } else { ":" }, original_path);
    
    // Run xlaude open with PATH set to include our mock tool
    let assert = Command::cargo_bin("xlaude")
        .unwrap()
        .arg("open")
        .arg("test-feature") // Specify the worktree name
        .current_dir(worktree_dir.path())
        .env("PATH", new_path)
        .env("XLAUDE_MOCK_TEST", "1") // Signal that we're in test mode
        .env("XLAUDE_MOCK_OUTPUT", "1") // Capture mock tool execution
        .env("XLAUDE_YES", "1") // Auto-confirm prompts
        .env("XLAUDE_NON_INTERACTIVE", "1") // Disable interactive mode
        .assert();
    
    // Check that the command mentions using Qwen (fallback)
    assert
        .stderr(predicate::str::contains("MOCK_AI_TOOL_EXECUTED"))
        .stderr(predicate::str::contains("qwen"));
}

#[test]
fn test_env_override() {
    // Set up git repo with worktree
    let (_repo_dir, worktree_dir) = setup_git_repo_with_worktree();
    
    // Create a temporary directory for mock tools
    let temp_dir = TempDir::new().unwrap();
    let mock_path = create_mock_ai_tool(&temp_dir, "custom_ai_tool");
    
    // Add temp directory to PATH
    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}{}{}", mock_path, if cfg!(windows) { ";" } else { ":" }, original_path);
    
    // Set up custom command path
    let custom_tool_path = format!("{}/custom_ai_tool", mock_path);
    
    // Run xlaude open with PATH and override env var
    let assert = Command::cargo_bin("xlaude")
        .unwrap()
        .arg("open")
        .arg("test-feature") // Specify the worktree name
        .current_dir(worktree_dir.path())
        .env("PATH", new_path)
        .env("XLAUDE_OPENCODE_CMD", &custom_tool_path) // Override the opencode command
        .env("XLAUDE_MOCK_TEST", "1") // Signal that we're in test mode
        .env("XLAUDE_MOCK_OUTPUT", "1") // Capture mock tool execution
        .env("XLAUDE_YES", "1") // Auto-confirm prompts
        .env("XLAUDE_NON_INTERACTIVE", "1") // Disable interactive mode
        .assert();
    
    // Check that the command used our custom tool
    assert
        .stderr(predicate::str::contains("MOCK_AI_TOOL_EXECUTED"))
        .stderr(predicate::str::contains("custom_ai_tool"));
}
