# CCometixLine

[English](README.md) | [中文](README.zh.md)

A high-performance Claude Code statusline tool written in Rust, combining beautiful Nerd Font rendering with real-time session statistics — context usage, active tools, running agents, todo progress, and environment info. Always visible below your input.

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)

## Screenshots

![CCometixLine](assets/img1.png)

```
 Opus 4.6 | 󰉋 project |  ████░░░░░░ 42.5% · 85k | 󱈔 Read: main.rs 2act 8ok | 󰈙 Explore (15s) | 󰘵 Fix tests 3/5 | 󰒓 2 CLAUDE.md · 4 rules · 3 MCPs
```

## Features

### Statusline Segments

| Segment | Icon | Description |
|---------|------|-------------|
| **Model** |  | Claude model name with auto version extraction |
| **Directory** | 󰉋 | Current workspace directory |
| **Git** | 󰊢 | Branch, status (clean/dirty/conflicts), ahead/behind |
| **Context Window** |  | Colored progress bar + percentage + token count |
| **Usage** | 󰪞 | API rate limit (5h/7d) with dynamic circle icon |
| **Cost** |  | Session cost in USD |
| **Session** | 󱦻 | Duration + lines added/removed |
| **Tools** | 󰊕 | Active tools + completed tool counts from transcript |
| **Agents** | 󰚩 | Running/completed agent type, description, elapsed time |
| **Todos** | 󰄬 | Current task + progress (completed/total) |
| **Environment** | 󰒓 | CLAUDE.md, rules, MCPs, hooks counts |
| **Output Style** | 󱋵 | Current output style name |
| **Update** |  | Version check |

### Context Progress Bar

Visual colored bar that changes with usage level:
- **Green** `████░░░░░░` — below 70%
- **Yellow** `██████░░░░` — 70%-85%
- **Red** `█████████░` — above 85%

### Real-time Session Monitoring

Parses the Claude Code transcript JSONL file to provide live tracking:

- **Active Tools** — Shows currently running tools (e.g., `Read: main.rs`) and completed tool counts (`8ok`)
- **Running Agents** — Displays agent type, description, and elapsed time (e.g., `Explore: Find API patterns`)
- **Todo Progress** — Shows current in-progress task and completion ratio (e.g., `Fix tests 3/5`)
- **Environment** — Counts loaded CLAUDE.md files, rules, MCP servers, and hooks

### Interactive TUI Configuration

- **TUI config interface** with real-time preview (`ccline -c`)
- **Theme system** — 9 built-in presets (Cometix, Gruvbox, Nord, Powerline variants, etc.)
- **Segment customization** — Enable/disable, reorder, customize icons and colors
- **Custom themes** — Save and load from `~/.claude/ccline/themes/*.toml`

### Claude Code Enhancement

- **Context warning disabler** — Remove "Context low" messages
- **Verbose mode enabler** — Enhanced output detail
- **Robust patcher** — Tree-sitter AST-based, survives version updates
- **Automatic backups** — Safe modification with easy recovery

## Installation

### One-line Install (Recommended)

Requires [Rust toolchain](https://rustup.rs/) installed.

```bash
curl -fsSL https://raw.githubusercontent.com/lucasmen9527/CCometixLine/master/install.sh | bash
```

This will clone, build, and install `ccline` to `~/.claude/ccline/`.

### Build from Source

```bash
git clone https://github.com/lucasmen9527/CCometixLine.git
cd CCometixLine
cargo build --release
mkdir -p ~/.claude/ccline
cp target/release/ccometixline ~/.claude/ccline/ccline
```

### Update

Re-run the install script or rebuild from source:

```bash
curl -fsSL https://raw.githubusercontent.com/lucasmen9527/CCometixLine/master/install.sh | bash
```

### Claude Code Configuration

Add to your Claude Code `settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/ccline/ccline",
    "padding": 0
  }
}
```

> **Note:** `~` works on all platforms (macOS/Linux/Windows with Claude Code v2.1.47+).

## Usage

### Theme Override

```bash
ccline --theme cometix
ccline --theme gruvbox
ccline --theme nord
ccline --theme powerline-dark
ccline --theme powerline-tokyo-night
```

### Claude Code Enhancement

```bash
ccline --patch /path/to/claude-code/cli.js
```

## Configuration

Configuration file: `~/.claude/ccline/config.toml`

All segments support:
- Enable/disable toggle
- Custom Nerd Font / emoji icons
- 16-color, 256-color, and RGB color
- Bold text style
- Per-segment options

### Segment Configuration Example

```toml
[[segments]]
id = "tools"
enabled = true

[segments.icon]
plain = "🔧"
nerd_font = "󰊕"

[segments.colors.icon]
c256 = 75

[segments.colors.text]
c256 = 75

[segments.styles]
text_bold = false
```

### Model Configuration (`models.toml`)

Location: `~/.claude/ccline/models.toml`

Claude models are auto-recognized. Use this for third-party models:

```toml
[[models]]
pattern = "glm-4.5"
display_name = "GLM-4.5"
context_limit = 128000

[[context_modifiers]]
pattern = "[1m]"
display_suffix = " 1M"
context_limit = 1000000
```

## Requirements

- **Terminal**: Nerd Font support ([nerdfonts.com](https://www.nerdfonts.com/))
- **Git**: 1.5+ (2.22+ recommended)
- **Claude Code**: For statusline integration

## Development

```bash
cargo build          # Dev build
cargo test           # Run tests
cargo build --release # Optimized release
```

## Roadmap

- [x] TOML configuration file support
- [x] TUI configuration interface
- [x] Custom themes (9 built-in presets)
- [x] Interactive main menu
- [x] Claude Code enhancement tools
- [x] Context progress bar with color coding
- [x] Active tools tracking from transcript
- [x] Running agents monitoring
- [x] Todo progress display
- [x] Environment config counts

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgements

This project is a fork of [CCometixLine](https://github.com/Haleclipse/CCometixLine) by [Haleclipse](https://github.com/Haleclipse), which provides the excellent foundation — high-performance Rust statusline engine, beautiful theme system, TUI configurator, and Claude Code patcher.

The real-time session monitoring features (tools tracking, agents monitoring, todo progress, environment counts, and context progress bar) are inspired by [claude-hud](https://github.com/jarrodwatts/claude-hud) by [Jarrod Watts](https://github.com/jarrodwatts), which pioneered the idea of displaying live transcript statistics in the Claude Code statusline.

Thanks to both projects for their outstanding work!

## License

This project is licensed under the [MIT License](LICENSE).
