# xlaude - AI Coding Agent Management

xlaude is a CLI tool for managing AI coding agent instances with git worktree for parallel development workflows. It supports multiple AI coding tools in the following priority order:

1. **OpenCode** - A powerful terminal AI coding tool
2. **Qwen Code** - Qwen's code assistant CLI 
3. **Claude** - AI assistant developed by Anthropic

## Core Features

### xlaude create [name]
Create new worktree and branch:
- Must be run on main/master/develop branch
- If no name provided, randomly selects a word from BIP39 wordlist
- Creates new branch `<n>`
- Creates worktree at `../<repo-name>-<n>` directory
- **Does not automatically launch AI tool**

### xlaude open [name]
Open existing worktree and launch AI coding tool:
- With argument: Opens specified worktree
- Without argument:
  - If current directory is a worktree (not main/master/develop): Opens current worktree
  - If current worktree not managed: Asks to add and open
  - Otherwise: Displays interactive selection
- Changes to worktree directory
- Attempts to launch AI tools in priority order:
  1. OpenCode
  2. Qwen Code
  3. Claude (adds `--dangerously-skip-permissions` parameter if used)
- Inherits all environment variables

### xlaude delete [name]
Delete worktree and clean up:
- With argument: Delete specified worktree
- Without argument: Delete current worktree
- Checks for uncommitted changes and unpushed commits
- Checks if branch is fully merged, prompts for force delete if not
- Performs double confirmation when needed
- Automatically deletes worktree and local branch (if safe)

### xlaude add [name]
Add current worktree to xlaude management:
- Must be run within a git worktree
- If no name provided, uses current branch name
- Checks if already managed to avoid duplicates
- Useful for manually created worktrees or projects cloned from elsewhere

### xlaude list
List all active worktrees, showing:
- Name
- Repository name
- Path
- Creation time
- AI tool sessions (if any)
  - Shows up to 3 most recent sessions
  - Each session displays: last update time and last user message
  - Shows remaining count if more than 3

### xlaude clean
Clean up invalid worktrees:
- Checks if all managed worktrees still exist in git
- Automatically removes worktrees deleted manually
- Useful after using `git worktree remove`
- Keeps xlaude state in sync with git state

### xlaude rename <old_name> <new_name>
Rename worktree state:
- Renames worktree name in xlaude management
- Only updates xlaude state, not actual git worktree or directory
- Checks for name conflicts
- Preserves all AI tool sessions and metadata

### xlaude dir [name]
Get worktree directory path:
- With argument: Returns absolute path of specified worktree
- Without argument: Shows interactive selection
- Outputs plain path without decorators for shell command use
- Useful for integration with other tools (cd, editors, zoxide, etc)

## Technical Implementation

- Built with Rust
- Direct git command invocation
- AI tool priority and fallback mechanism
  - Tries tools in OpenCode > Qwen Code > Claude order
  - Customizable through environment variables
- State persistence location:
  - macOS: `~/Library/Application Support/com.xuanwo.xlaude/state.json`
  - Linux: `~/.config/xlaude/state.json`
  - Windows: `%APPDATA%\xuanwo\xlaude\config\state.json`
  - Worktree key format: `<repo-name>/<worktree-name>` (v0.3+)
  - Automatic migration from older formats
- Built with clap CLI framework
- BIP39 wordlist for random name generation
- Colorful output and interactive confirmations
- Integration tests covering all core functionality

## Usage Examples

```bash
# Create new working branch in opendal project
cd opendal
xlaude create feature-x  # Creates ../opendal-feature-x directory

# Create with random name
xlaude create  # Might create ../opendal-dolphin directory

# Open and launch AI coding tool
xlaude open feature-x  # Opens specified worktree
xlaude open  # Opens current worktree if in one, otherwise interactive selection

# Add existing worktree to management
cd ../opendal-bugfix
xlaude add  # Uses current branch name
xlaude add hotfix  # Or specify custom name

# List all active instances
xlaude list

# Delete current worktree
xlaude delete

# Delete specified worktree
xlaude delete feature-x

# Clean invalid worktrees
xlaude clean

# Rename worktree
xlaude rename feature-x feature-improved

# Typical workflow
xlaude create my-feature  # Create worktree
xlaude open my-feature   # Open and start working
# ... work complete ...
xlaude delete my-feature # Clean up worktree

# Launch directly in current worktree
cd ../opendal-feature
xlaude open  # Auto-detects and opens current worktree

# Get worktree path (for directory switching)
cd $(xlaude dir feature-x)  # Switch to specified worktree
xlaude dir  # Interactive selection and output path

# Use with shell function
# Add to .bashrc/.zshrc:
# xcd() { cd $(xlaude dir "$@"); }
xcd feature-x  # Quickly switch to worktree

# Integration with other tools
code $(xlaude dir feature-x)  # Open in VSCode
vim $(xlaude dir feature-x)/src/main.rs  # Edit file
```

## Environment Variables

- `XLAUDE_YES`: Set to "1" to auto-confirm all prompts
- `XLAUDE_NON_INTERACTIVE`: Set to "1" to disable interactive prompts
- `XLAUDE_OPENCODE_CMD`: Override the OpenCode command (default: "opencode")
- `XLAUDE_QWEN_CMD`: Override the Qwen Code command (default: "qwen")
- `XLAUDE_CLAUDE_CMD`: Override the Claude command (default: "claude")

## AI Tool Installation

### OpenCode

```bash
# Using installation script
curl -fsSL https://raw.githubusercontent.com/opencode-ai/opencode/refs/heads/main/install | bash

# Using Homebrew (macOS and Linux)
brew install opencode-ai/tap/opencode
```

More information: [OpenCode Documentation](https://opencode.ai/docs/cli/)

### Qwen Code

```bash
# Install with npm
npm install -g @qwen-code/qwen-code@latest

# Check version
qwen --version
```

More information: [Qwen Code Repository](https://github.com/QwenLM/qwen-code)

### Claude

Please refer to Anthropic's official installation guide for Claude CLI.
