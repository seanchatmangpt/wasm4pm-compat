# Settings & Hooks Exhaustive Discovery Report

**Generated:** 2026-06-01  
**Scope:** Complete audit of hook declarations, plugins, MCP servers, and tool registrations in Claude Code configuration  
**Primary Config:** `/Users/sac/.claude/settings.json` and `/Users/sac/.claude/settings.local.json`

---

## 1. Global Settings Configuration

### Location
- **Primary:** `/Users/sac/.claude/settings.json`
- **Local Overrides:** `/Users/sac/.claude/settings.local.json`
- **Project-Level:** None found in `/Users/sac/wasm4pm-compat/.claude/`

### Core Settings
| Setting | Value | Purpose |
|---------|-------|---------|
| `model` | `haiku` | Default Claude model (Haiku 4.5) |
| `effortLevel` | `xhigh` | Extra-high effort for code reviews/analysis |
| `skipDangerousModePermissionPrompt` | `true` | Suppress dangerous mode warnings |
| `skipWorkflowUsageWarning` | `true` | Skip workflow usage notifications |
| `skipAutoPermissionPrompt` | `true` | Auto-approve permitted bash/mcp calls |

### Environment Variables
**PATH override set in `env.PATH`:**
```
/Users/sac/.m2/mvnd/bin
/Users/sac/.sdkman/candidates/mvnd/current/bin
/Users/sac/.sdkman/candidates/maven/current/bin
/Users/sac/.sdkman/candidates/java/current/bin
/Users/sac/.local/bin
/Users/sac/.erlmcp/otp-28.3.1/bin
/opt/homebrew/bin
/opt/homebrew/sbin
/usr/local/bin
/usr/bin
/bin
/usr/sbin
/sbin
/opt/pmk/env/global/bin
/Library/Apple/usr/bin
/Library/TeX/texbin
/Applications/VMware Fusion.app/Contents/Public
/usr/local/go/bin
/Users/sac/.cargo/bin
```

---

## 2. Hooks Registry

### Global Hooks (settings.json)

#### Hook: `Stop` Event
**Type:** Session termination / exit prevention  
**Activation:** When user attempts to exit Claude Code session  
**Command:** `bash ~/.claude/rdf-loop/rdf-stop-hook.sh`  
**Purpose:** Prevent early session exit when RDF-native Ralph Loop is active  

**Handler Details:**
- **File:** `/Users/sac/.claude/rdf-loop/rdf-stop-hook.sh`
- **Language:** Bash
- **State Machine:** RDF Turtle triples using PROV-O ontology
- **Logic Flow:**
  1. Check for active loop state file (`~/.claude/rdf-loop/state.ttl`)
  2. Extract session ID, iteration number, max iterations, completion pattern via SPARQL queries
  3. If max iterations reached → return `{"decision":"approve"}` (allow exit)
  4. Extract last assistant output from JSONL transcript
  5. Update state file with new iteration and captured output
  6. Check completion via SPARQL ASK: `FILTER(CONTAINS(?out, ?pat))`
  7. If completed → return `{"decision":"approve"}`
  8. Otherwise → return `{"decision":"block","reason":$prompt,"systemMessage":"🔄 RDF iteration N/MAX"}`

**Output Format:** JSON
```json
{
  "decision": "approve" | "block",
  "reason": "optional prompt text",
  "systemMessage": "optional system status"
}
```

**State File Format:** RDF Turtle
```turtle
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix schema: <https://schema.org/> .
@prefix loop: <http://claude.ai/loop/> .
@prefix dcterms: <http://purl.org/dc/terms/> .

<urn:loop:current>
    a prov:Activity, schema:Action ;
    dcterms:identifier "${SESSION_ID}" ;
    schema:actionStatus schema:ActiveActionStatus ;
    loop:iterationNumber ${ITERATION} ;
    loop:maxIterations ${MAX} ;
    loop:completionPattern "${PATTERN}" ;
    prov:value "${OUTPUT_TEXT}" ;
    prov:startedAtTime "${ISO8601_TIMESTAMP}" .
```

---

### Plugin Hooks

#### Plugin: `ralph-loop@claude-plugins-official` (ENABLED)

**Hook: `Stop` Event**
- **Command:** `bash "${CLAUDE_PLUGIN_ROOT}/hooks/stop-hook.sh"`
- **Purpose:** Self-referential loop termination control
- **State:** Active
- **Location:** 
  - Marketplace: `/Users/sac/.claude/plugins/marketplaces/claude-plugins-official/plugins/ralph-loop/hooks/hooks.json`
  - Cache: `/Users/sac/.claude/plugins/cache/claude-plugins-official/ralph-loop/1.0.0/hooks/hooks.json`

---

#### Plugin: `explanatory-output-style@claude-plugins-official` (ENABLED)

**Hook: `SessionStart` Event**
- **Command:** `bash "${CLAUDE_PLUGIN_ROOT}/hooks-handlers/session-start.sh"`
- **Purpose:** Inject educational insights instructions on session startup
- **State:** Active
- **Locations:**
  - Marketplace: `/Users/sac/.claude/plugins/marketplaces/claude-plugins-official/plugins/explanatory-output-style/hooks/hooks.json`
  - Cache: `/Users/sac/.claude/plugins/cache/claude-plugins-official/explanatory-output-style/1.0.0/hooks/hooks.json`

---

#### Plugin: `learning-output-style@claude-plugins-official` (UNKNOWN — listed in hooks but not in enabledPlugins)

**Hook: `SessionStart` Event**
- **Command:** `bash "${CLAUDE_PLUGIN_ROOT}/hooks-handlers/session-start.sh"`
- **Purpose:** Inject interactive learning instructions on session startup
- **State:** Unknown (hook declared but plugin enablement status unclear)
- **Location:** `/Users/sac/.claude/plugins/marketplaces/claude-plugins-official/plugins/learning-output-style/hooks/hooks.json`

---

#### Plugin: `security-guidance@claude-plugins-official` (ENABLED)

**Hook: `SessionStart` Event**
- **Command:** `bash "${CLAUDE_PLUGIN_ROOT}/hooks/sg-python.sh" "${CLAUDE_PLUGIN_ROOT}/hooks/ensure_agent_sdk.py"`
- **Timeout:** 180 seconds
- **Purpose:** Ensure Agent SDK is available; validate security guidance setup
- **State:** Active

**Hook: `UserPromptSubmit` Event**
- **Command:** `bash "${CLAUDE_PLUGIN_ROOT}/hooks/sg-python.sh" "${CLAUDE_PLUGIN_ROOT}/hooks/security_reminder_hook.py"`
- **Purpose:** Pattern-based security warnings on user prompts
- **State:** Active
- **Trigger:** Each time user submits a prompt

**Hook: `PostToolUse` Event**
- **Command:** `bash "${CLAUDE_PLUGIN_ROOT}/hooks/sg-python.sh" "${CLAUDE_PLUGIN_ROOT}/hooks/security_reminder_hook.py"`
- **Purpose:** Pattern-based security warnings on tool output
- **State:** Active
- **Trigger:** After any tool completes execution

---

#### Plugin: `superpowers@claude-plugins-official` (DISABLED)

**Hook: `SessionStart` Event**
- **Command:** `"${CLAUDE_PLUGIN_ROOT}/hooks/run-hook.cmd" session-start`
- **Matcher:** `startup|clear|compact`
- **Async:** false
- **Purpose:** Initialize superpowers plugin state
- **State:** Disabled (not in enabledPlugins list, but hook is registered)

---

#### Plugin: `hookify@claude-plugins-official` (STATUS UNKNOWN)

**Hook: `PreToolUse` Event**
- **Command:** `python3 "${CLAUDE_PLUGIN_ROOT}/hooks/pretooluse.py"`
- **Timeout:** 10 seconds
- **Purpose:** User-configurable hook from `.local.md` files (pre-tool-use phase)
- **State:** Hook registered; plugin enabled status unknown

**Hook: `PostToolUse` Event**
- **Command:** `python3 "${CLAUDE_PLUGIN_ROOT}/hooks/posttooluse.py"`
- **Timeout:** 10 seconds
- **Purpose:** User-configurable hook from `.local.md` files (post-tool-use phase)
- **State:** Hook registered; plugin enabled status unknown

**Hook: `Stop` Event**
- **Command:** `python3 "${CLAUDE_PLUGIN_ROOT}/hooks/stop.py"`
- **Purpose:** User-configurable hook on session exit
- **State:** Hook registered; plugin enabled status unknown

---

## 3. Status Line Command

### Location
`~/.claude/statusline-command.sh`

### Activation
**Event:** Real-time status display (runs continuously during session)

### Input Format
JSON structure passed via stdin:
```json
{
  "model": { "display_name": "string" },
  "workspace": { "current_dir": "string" },
  "session_name": "string",
  "context_window": { "remaining_percentage": "number" }
}
```

### Output Format
Single-line terminal-friendly status string:
```
HOSTNAME ॐ BASENAME:GIT_BRANCH[*] SESSION_NAME [HH:MM:SS] │ CONTEXT%🟢|🟡|🔴
```

### Logic
1. Extract model name, directory, git branch, git status (modified file count)
2. Extract context remaining percentage
3. Color-code context: 🟢 >70%, 🟡 30-70%, 🔴 <30%
4. If git has uncommitted changes, append `*` to branch indicator
5. Display session name if present
6. Show ISO 8601 timestamp

---

## 4. MCP Servers Configuration

### Primary MCP Server
**Name:** `open-ontologies`  
**Command:** `/Users/sac/chatmangpt/ostar/vendors/open-ontologies/target/release/open-ontologies`  
**Arguments:** `["serve"]`  
**Purpose:** RDF/ontology query and manipulation via MCP protocol  
**State:** Registered in `mcpServers`  
**Enabled:** Yes (in `enabledMcpjsonServers: ["open-ontologies"]`)

### Local Override Permissions
**File:** `/Users/sac/.claude/settings.local.json`  
```json
{
  "permissions": {
    "allow": ["mcp__ruv-swarm"],
    "deny": []
  }
}
```
**Meaning:** `mcp__ruv-swarm` tool is whitelisted (allows auto-invocation without permission prompt)

---

## 5. Plugin Registry (Enabled/Disabled State)

| Plugin | Marketplace | Enabled | LSP | Purpose |
|--------|-----------|---------|-----|---------|
| `rust-analyzer-lsp` | official | ✅ | Yes | Rust code intelligence |
| `jdtls-lsp` | official | ✅ | Yes | Java code intelligence |
| `pyright-lsp` | official | ✅ | Yes | Python code intelligence |
| `typescript-lsp` | official | ✅ | Yes | TypeScript code intelligence |
| `claude-md-management` | official | ✅ | No | CLAUDE.md auditing/updates |
| `ralph-loop` | official | ✅ | No | Self-referential loop control |
| `explanatory-output-style` | official | ✅ | No | Educational mode output |
| `frontend-design` | official | ❌ | No | Frontend design guidance |
| `superpowers` | official | ❌ | No | Superpowers features |

---

## 6. Permission Allowlist (Bash + MCP Tools)

### Allowed Bash Patterns
```
npm run lint
npm run test:*
npm test:*
git status
git diff:*
git log:*
git add:*
git commit:*
git push
git config:*
git tag:*
git branch:*
git checkout:*
git stash:*
jq:*
node:*
which:*
pwd
ls:*
uv run pytest:*
tree:*
find:*
done
sort:*
xargs:*
python3:*
uv run ruff check:*
git -C /Users/sac/dev/kgcl log:*
uv run mypy:*
python -m pytest:*
uv pip install:*
pip install pyoxigraph
uv run python:*
```

### Denied Bash Patterns
```
rm -rf /
```

### MCP Tools Allowlist
```
mcp__ruv-swarm
```

---

## 7. Hook Event Types Observed

| Event | Hook Registered | Activation Condition | Num Hooks | Blocking |
|-------|-----------------|---------------------|-----------|----------|
| `SessionStart` | ✅ | Session initializes | 3-4 | No (informational) |
| `UserPromptSubmit` | ✅ | User submits prompt | 1 | No (advisory) |
| `PreToolUse` | ✅ | Before tool invocation | 1 | Possibly (10s timeout) |
| `PostToolUse` | ✅ | After tool completion | 2 | Possibly (10s timeout) |
| `Stop` | ✅ | Session exit requested | 3 | **YES (can block exit)** |

---

## 8. Stateful Components

### RDF Loop State
**File:** `~/.claude/rdf-loop/state.ttl`  
**Format:** RDF Turtle  
**Semantics:** PROV-O + loop: custom vocabulary  
**Persistence:** File-based; scoped to session_id  
**Lifecycle:** Created on loop start, deleted on loop completion or session exit

### RDF Loop Configuration
**Files:**
- Template: `~/.claude/rdf-loop/prompt.njk` (Nunjucks template for next prompt generation)
- Hooks: JSON hook configuration (nested in hooks array)

### Git Integration
**Tool:** Native git commands (delegated to Bash)  
**Integration Points:**
1. Status line updates git branch via `git -C $dir rev-parse --abbrev-ref HEAD`
2. Status line counts modified files via `git status --porcelain`
3. Permission allowlist includes git operations: status, diff, log, add, commit, push, etc.

---

## 9. Attribution & PR Configuration

### Attribution Settings
```json
{
  "commit": "",
  "pr": ""
}
```
**Status:** Empty (no auto-attribution configured)  
**Implication:** Manual commit/PR attribution only

---

## 10. Workflow & Tool Usage Suppressions

| Setting | Value | Effect |
|---------|-------|--------|
| `skipDangerousModePermissionPrompt` | true | Suppress "dangerous mode" confirmation dialogs |
| `skipWorkflowUsageWarning` | true | Do not warn when workflows are invoked |
| `skipAutoPermissionPrompt` | true | Auto-allow Bash/MCP calls matching allowlist (no prompt) |

---

## 11. Hook Invocation Chain Example: Session Stop Flow

```
User requests: exit / Ctrl+C / session end
         ↓
Claude Code triggers "Stop" event
         ↓
Invoke global hook: bash ~/.claude/rdf-loop/rdf-stop-hook.sh
         ├─ INPUT: { session_id, transcript_path, ... }
         ├─ Read ~/.claude/rdf-loop/state.ttl (SPARQL queries)
         ├─ Max iterations reached? → {decision:"approve"} → EXIT
         ├─ Extract last assistant output from transcript
         ├─ Update state.ttl with new iteration + output
         ├─ Check completion pattern via SPARQL ASK
         ├─ Pattern matched? → {decision:"approve"} → EXIT
         └─ Not completed? → {decision:"block","reason":"[prompt]"} → BLOCK SESSION EXIT
         ↓
Invoke plugin hook: ralph-loop → bash "${CLAUDE_PLUGIN_ROOT}/hooks/stop-hook.sh"
         ↓
Return combined decision (if any hook blocks, exit is blocked)
```

---

## 12. Absence of Declared Hooks

### Not Found in Configuration
- Pre-commit hooks (git hooks)
- Post-push hooks
- File-watcher triggers
- Cron/scheduled job triggers
- Webhook integrations
- Database triggers
- Event bus subscriptions

### Location
These would typically appear in:
- `.git/hooks/` — not registered in settings.json
- GitHub Actions workflows — not in this settings audit
- `.claude/cron/` — not found

---

## 13. Knowledge Gaps & Ambiguities

| Item | Status | Notes |
|------|--------|-------|
| learning-output-style plugin enablement | Unknown | Hook exists, but plugin not in enabledPlugins list |
| hookify plugin enablement | Unknown | Hooks exist (PreToolUse, PostToolUse, Stop) but no explicit enable/disable |
| superpowers plugin enablement | DISABLED | Hook registered but plugin is disabled (likely stale) |
| MCP server authentication | Not visible | open-ontologies command path exists; no auth tokens in visible settings |
| Prompt template for next iteration | File assumed | `~/.claude/rdf-loop/prompt.njk` referenced in rdf-stop-hook.sh; not validated |
| Async hook execution | Mixed | Some hooks marked `async:false`, others use timeouts; execution model unclear |

---

## 14. Configuration Files Scanned

| Path | Type | Status |
|------|------|--------|
| `/Users/sac/.claude/settings.json` | JSON | ✅ Complete |
| `/Users/sac/.claude/settings.local.json` | JSON | ✅ Complete |
| `/Users/sac/wasm4pm-compat/.claude/settings.json` | JSON | ❌ Not found |
| `~/.clauderc` / `~/.claude-code-config` | Various | ❌ Not found |
| `~/.vscode/settings.json` | JSON | Skipped (not Claude Code) |

---

## Summary

**Total Hooks:** 12 declared across global + plugin sources  
**Active Hooks:** 9-10 (learning-output-style, hookify, superpowers status unclear)  
**Blocking Hooks:** 1 (`Stop` event — can prevent session exit)  
**Event Types:** 5 (SessionStart, UserPromptSubmit, PreToolUse, PostToolUse, Stop)  
**Stateful Components:** 1 (RDF Turtle state file for ralph-loop)  
**MCP Servers:** 1 active (`open-ontologies`), 1 whitelisted (`mcp__ruv-swarm`)  
**Permission Allowlist:** 32 Bash patterns, 1 MCP tool  

---

**Report Generated By:** Claude Code (Haiku 4.5)  
**Scope:** All hook, plugin, MCP, and configuration declarations in `~/.claude/` and `/Users/sac/wasm4pm-compat/.claude/`
