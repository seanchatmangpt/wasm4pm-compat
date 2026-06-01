# MCP Server Configuration Report

**Generated:** 2026-06-01  
**Scope:** Global (`~/.claude/`) and project-level configurations  
**Config Files Scanned:**
- `~/.claude/mcp.json` (global)
- `~/.claude/settings.json` (global)
- `/Users/sac/wasm4pm-compat/.claude/settings.json` (project — not configured)

---

## Summary

| Configuration | Location | Status |
|---|---|---|
| **Global MCP servers** | `~/.claude/mcp.json` | 1 server enabled |
| **Global settings** | `~/.claude/settings.json` | 1 server enabled, multiple plugins |
| **Project-level overrides** | `.claude/settings.json` | Not configured |

---

## Global Configuration: `~/.claude/mcp.json`

### Enabled Servers

#### cpmp (Capability Map)
- **Type:** stdio
- **Command:** `/Users/sac/capability-map/target/release/cpmp`
- **Args:** `["serve"]`
- **Environment:** Empty (inherits parent)
- **Status:** Enabled
- **Tools Exposed:** Not documented (cpmp is a capability-mapping binary; typical tools: capability discovery, mapping composition, validation)

---

## Global Configuration: `~/.claude/settings.json`

### MCP Servers via `mcpServers`

#### open-ontologies
- **Type:** stdio (implicit)
- **Command:** `/Users/sac/chatmangpt/ostar/vendors/open-ontologies/target/release/open-ontologies`
- **Args:** `["serve"]`
- **Environment:** Inherited from parent
- **Status:** Enabled via `enabledMcpjsonServers: ["open-ontologies"]`
- **Tools Exposed:** Not documented (likely RDF/ontology tools for the O* project)

### Enabled Plugins

| Plugin | Package | Enabled | Purpose |
|---|---|---|---|
| `rust-analyzer-lsp` | `claude-plugins-official` | ✓ | Rust code intelligence via LSP |
| `jdtls-lsp` | `claude-plugins-official` | ✓ | Java code intelligence via LSP |
| `claude-md-management` | `claude-plugins-official` | ✓ | CLAUDE.md audit & management |
| `ralph-loop` | `claude-plugins-official` | ✓ | Recurring task execution |
| `pyright-lsp` | `claude-plugins-official` | ✓ | Python type checking via LSP |
| `typescript-lsp` | `claude-plugins-official` | ✓ | TypeScript/JavaScript LSP |
| `explanatory-output-style` | `claude-plugins-official` | ✓ | Output formatting |
| `frontend-design` | `claude-plugins-official` | ✗ | (disabled) |
| `superpowers` | `claude-plugins-official` | ✗ | (disabled) |

### Custom Configuration

#### PATH Environment
Augmented with:
- Maven Daemon (`mvnd`) — `/Users/sac/.m2/mvnd/bin`
- SDKMAN candidates — `mvnd`, `maven`, `java`
- Erlang OTP 28.3.1 — `/Users/sac/.erlmcp/otp-28.3.1/bin`
- Go, Cargo, local tools

#### Permissions Model
- **Default mode:** `auto` (permission-gated by regex allowlist)
- **Allowlist (select patterns):**
  - Git operations: `git status`, `git diff:*`, `git log:*`, `git add:*`, `git commit:*`, `git push`, `git tag:*`, `git branch:*`, `git checkout:*`, `git stash:*`
  - npm: `npm run lint`, `npm run test:*`
  - Python tools: `uv run pytest:*`, `uv run ruff check:*`, `uv run mypy:*`, `uv run python:*`
  - Build tools: `node:*`, `python3:*`, `python -m pytest:*`
  - Utilities: `jq:*`, `which:*`, `pwd`, `ls:*`, `tree:*`, `find:*`, `sort:*`, `xargs:*`
  - Package management: `uv pip install:*`, `pip install pyoxigraph`
- **Denylist:** `rm -rf /`

#### Hooks

**Stop hook:** Executes `~/.claude/rdf-loop/rdf-stop-hook.sh` on session termination

#### Status Line
- **Type:** command
- **Command:** `~/.claude/statusline-command.sh`

#### Model
- **Default:** `haiku` (Claude 3.5 Haiku)

#### Other Flags
- `skipDangerousModePermissionPrompt`: `true`
- `skipWorkflowUsageWarning`: `true`
- `skipAutoPermissionPrompt`: `true`
- `effortLevel`: `xhigh` (maximum effort for code reviews/analysis)

---

## Project-Level Configuration

### `.claude/settings.json` (wasm4pm-compat)
**Status:** Not configured  
Projects inherit all global settings from `~/.claude/settings.json`. To override:
1. Create `.claude/settings.json` in project root
2. Include only fields to override (merges with global config)
3. Example: custom `mcpServers`, project-specific `permissions.allow`, plugins override

---

## Key Observations

### MCP Server Deployment Model
- **Global:** Two servers (`cpmp`, `open-ontologies`) registered as local stdio processes
- **Both require pre-built binaries** at documented paths; servers fail silently if binaries absent
- **No network servers configured** (only stdio over local processes)

### Tool Exposure Gap
- MCP server configurations do not enumerate exposed tools
- Tools become available at runtime when servers start successfully
- See system-level tool documentation or server `--help` for tool lists

### Plugin Architecture
- **8 official plugins** available; **6 enabled**
- **LSP plugins** (rust-analyzer, jdtls, pyright, typescript) enable code intelligence
- **No custom plugins** registered

### Permission Model
- Granular Bash command allowlist (regex patterns)
- Designed to block dangerous commands (`rm -rf /`) while allowing common CI/dev tools
- Git, npm, Python, and build tools whitelisted; operator approval required for unknown commands

---

## Connectivity & Troubleshooting

### Verifying Server Status
```bash
# Check if servers are running (both require pre-built binaries)
ps aux | grep -E 'cpmp|open-ontologies'

# Verify binary paths exist
test -x /Users/sac/capability-map/target/release/cpmp && echo "cpmp: OK"
test -x /Users/sac/chatmangpt/ostar/vendors/open-ontologies/target/release/open-ontologies && echo "open-ontologies: OK"
```

### If Servers Fail to Start
1. Check stderr output in Claude Code logs
2. Verify binaries are built (`cargo build --release` in respective projects)
3. Confirm binary execute permissions (`chmod +x /path/to/binary`)
4. Remove server from `mcpServers` or `enabledMcpjsonServers` to disable gracefully

---

## Schema Reference

### MCP Server Definition (stdio variant)
```json
{
  "type": "stdio",
  "command": "/path/to/binary",
  "args": ["optional", "arguments"],
  "env": {}
}
```

### Settings Override Hierarchy
1. **Default Claude Code settings** (internal)
2. `~/.claude/settings.json` (global user)
3. `.claude/settings.json` (project-specific, overrides global)

