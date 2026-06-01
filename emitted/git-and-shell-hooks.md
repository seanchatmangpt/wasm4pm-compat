# Git and Shell Hooks Audit

**Generated:** 2026-06-01  
**Repository:** `/Users/sac/wasm4pm-compat`  
**Scope:** Git hooks, shell startup hooks, environment variable hooks, Claude Code configuration hooks

---

## Summary

This repository and environment contain:
- **Git hooks:** 14 sample templates (uninstalled, default state)
- **Shell startup hooks:** Zsh configuration with multiple init paths and environment variable hooks
- **Claude Code hooks:** 1 Stop hook (RDF-native loop handler)
- **Status line hook:** dogenpunk-themed status display for Claude Code
- **Todo system hook:** Comprehensive todo tracking system (shell-based)

No **active/installed** git hooks are currently deployed. All `.git/hooks/` entries are `.sample` files (default state).

---

## Section 1: Git Hooks

### Location
`/Users/sac/wasm4pm-compat/.git/hooks/`

### Current State
All git hook templates exist as `.sample` files (standard Git initialization). None are currently installed as active hooks.

### Available Git Hook Templates

| Hook Name | Trigger | Sample Commands | Status |
|---|---|---|---|
| `applypatch-msg` | Before applying a patch via `git am` | Edit commit message, validate format | `.sample` (inactive) |
| `commit-msg` | After commit message is edited, before commit completes | Validate message format, run linters | `.sample` (inactive) |
| `pre-applypatch` | Before `git am` applies a patch | Run tests, check file state | `.sample` (inactive) |
| `pre-commit` | Before `git commit` creates the commit object | Lint, format check, run tests | `.sample` (inactive) |
| `pre-merge-commit` | Before `git merge` creates merge commit | Validate merged state | `.sample` (inactive) |
| `pre-push` | Before `git push` contacts remote | Validate commits, run full tests | `.sample` (inactive) |
| `pre-rebase` | Before `git rebase` starts | Warn about detached HEAD, check clean working tree | `.sample` (inactive) |
| `post-update` | After `git push` updates remote refs (server-side) | Trigger CI, send notifications | `.sample` (inactive) |
| `prepare-commit-msg` | After commit message template is prepared, before editor opens | Auto-insert branch name, signed-off-by | `.sample` (inactive) |
| `pre-receive` | Before remote receives pushed refs (server-side) | Validate push policy, reject force-push | `.sample` (inactive) |
| `update` | For each ref being updated (server-side) | Enforce access control per branch | `.sample` (inactive) |
| `sendemail-validate` | During `git send-email` | Validate email format before sending | `.sample` (inactive) |
| `push-to-checkout` | After successful push when deployed as remote (server-side) | Auto-checkout working tree, restart service | `.sample` (inactive) |
| `fsmonitor-watchman` | Watchman filesystem monitor integration | Query Watchman for changed files (performance) | `.sample` (inactive) |

**Observation:** No custom git hooks are installed. The `.sample` files are Git's default distribution templates. To activate a hook, rename it by removing the `.sample` extension and ensure it is executable (`chmod +x`).

---

## Section 2: Shell Startup Hooks

### Location
`/Users/sac/.zshrc` (primary shell)  
`/Users/sac/.bashrc` (fallback, minimal)

### Zsh Startup Hooks

The zshrc file is sourced on **every interactive shell session**. It initializes multiple systems:

#### 2.1 Local Secrets Loading
**Trigger:** Shell starts  
**Condition:** File exists at `~/.env.local`  
**Command:**
```bash
[ -f ~/.env.local ] && source ~/.env.local
```
**Purpose:** Load environment secrets (tokens, API keys) without committing them to version control.

---

#### 2.2 ASDF Version Manager Setup
**Trigger:** Shell starts (before Oh My Zsh)  
**Commands:**
```bash
export ASDF_DIR="$HOME/.asdf"
fpath=("${ASDF_DIR}/internal/completions" $fpath)
```
**Purpose:** Initialize asdf (polyglot version manager for Ruby, Node, Python, etc.) and completions.

---

#### 2.3 Oh My Zsh Initialization
**Trigger:** Shell starts  
**Path:** `$ZSH=$HOME/.oh-my-zsh`  
**Theme:** `dogenpunk`  
**Plugins:** `git`, `zsh-autosuggestions`, `zsh-syntax-highlighting`  
**Command:**
```bash
source $ZSH/oh-my-zsh.sh
```
**Purpose:** Load Zsh framework, theme, and completion plugins.

---

#### 2.4 SDKMAN (Java Version Manager)
**Trigger:** Shell starts (after Oh My Zsh)  
**Commands:**
```bash
export SDKMAN_DIR="$HOME/.sdkman"
[[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"
export JAVA_HOME="$HOME/.sdkman/candidates/java/current"
```
**Purpose:** Initialize Java version manager (manages JDK, Maven, Gradle installations).

---

#### 2.5 Maven Daemon (mvnd) Configuration
**Trigger:** Shell starts  
**Commands:**
```bash
export MAVEN_DAEMON_HOME="$HOME/.m2/mvnd"
export PATH="$MAVEN_DAEMON_HOME/bin:$PATH"
```
**Purpose:** Fast Maven builds using daemon (per global CLAUDE.md preference).

---

#### 2.6 JOTP Java 26 Override
**Trigger:** Shell starts (after SDKMAN)  
**Condition:** OpenJDK 26 installed (Homebrew or Linux)  
**Commands:**
```bash
if [[ -d "/usr/local/opt/openjdk@26" ]]; then
  export JAVA_HOME="/usr/local/opt/openjdk@26"
  export PATH="/usr/local/opt/openjdk@26/bin:$PATH"
  echo "Using Java 26 via Homebrew: $JAVA_HOME"
elif [[ -d "/usr/lib/jvm/openjdk-26" ]]; then
  export JAVA_HOME="/usr/lib/jvm/openjdk-26"
  export PATH="/usr/lib/jvm/openjdk-26/bin:$PATH"
  echo "Using Java 26: $JAVA_HOME"
fi
```
**Purpose:** Ensure Java 26 is used (overrides SDKMAN for Java version).

---

#### 2.7 YAWL Database Configuration
**Trigger:** Shell starts  
**Commands:**
```bash
export YAWL_DB_URL="${YAWL_DB_URL:-jdbc:postgresql://localhost:5432/yawl}"
export YAWL_DB_USER="${YAWL_DB_USER:-yawl}"
export YAWL_DB_PASSWORD="${YAWL_DB_PASSWORD:-yawl}"
```
**Purpose:** Set PostgreSQL connection defaults for YAWL workflow engine (allows override via environment).

---

#### 2.8 Cargo Environment
**Trigger:** Shell starts (early, from `.bashrc` sourced)  
**Command:**
```bash
. "$HOME/.cargo/env"
```
**Purpose:** Initialize Rust toolchain and cargo paths.

---

#### 2.9 UV Python Environment
**Trigger:** Shell starts  
**Command:**
```bash
source ~/.uvrc
```
**Purpose:** Initialize uv (Python package manager / virtualenv replacement).

---

#### 2.10 Anthropic API Configuration
**Trigger:** Shell starts  
**Commands:**
```bash
export ZAI_API_KEY="8acb06e95f7e4ae49e5e3ad504ebec92.pB7bBPYDHACylsmw"
export GROQ_API_KEY="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"
export CARGO_REGISTRY_TOKEN="cioorN0zTT8Xh8cdJlKRLToXbv2zvSlFpux"
export ANTHROPIC_API_KEY="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"
```
**Purpose:** Set API tokens for Anthropic SDK, Groq, Cargo registry access.

---

#### 2.11 PATH Extensions
**Trigger:** Shell starts  
**Commands:**
```bash
export PATH="/Users/sac/.local/bin:$PATH"
export PATH="$PATH:/Users/sac/A2A"
export PATH=$PATH:$HOME/.maestro/bin
export PATH="/bin:/usr/bin:/usr/sbin:/sbin:$PATH"
```
**Purpose:** Extend shell $PATH to include local binaries, custom tools (A2A), maestro orchestrator.

---

### Bash Startup Hooks (Fallback)

**Location:** `/Users/sac/.bashrc`

**Minimal hooks (sourced by zshrc for compatibility):**
```bash
. "$HOME/.cargo/env"
. "$HOME/.local/bin/env"
export PATH="/Users/sac/.local/bin:$PATH"
source ~/.uvrc
```

**Note:** `.bashrc` also contains dangerous entries (e.g., `sudo shutdown -h 0` repeated twice). These appear to be configuration artifacts and should be reviewed/removed.

---

## Section 3: Claude Code Hooks (Settings-Based)

### Location
`/Users/sac/.claude/settings.json` (global user configuration)

### 3.1 Stop Hook (Session Exit Hook)

**Hook Type:** `Stop`  
**Trigger:** When user attempts to exit/stop Claude Code session  
**Command:**
```bash
bash ~/.claude/rdf-loop/rdf-stop-hook.sh
```

**Purpose:** RDF-native Ralph Wiggum Loop integration. Prevents session exit if an active loop is running, feeds Claude's output back as input for next iteration.

**Detailed Behavior:**

1. **State File:** `~/.claude/rdf-loop/state.ttl` (RDF Turtle triples)
2. **Check Conditions:**
   - If no state file exists → allow exit immediately
   - If session ID in state doesn't match hook input → allow exit (different session)
   - If max iterations reached → allow exit
3. **Update State (if loop active):**
   - Extract last assistant text from transcript (JSONL)
   - Increment iteration counter
   - Write new state: `prov:value` (output), `loop:iterationNumber` (next iteration), timestamp
4. **Completion Check (SPARQL ASK):**
   - Query: `FILTER(CONTAINS(?out, ?pat))` — check if output contains completion pattern
   - If true → approve exit
   - If false → block exit, generate next prompt via `~/.claude/rdf-loop/prompt.njk`
5. **Response to Claude Code:**
   - Exit: `{"decision":"approve"}`
   - Block: `{"decision":"block","reason":"<next-prompt>","systemMessage":"🔄 RDF iteration N/MAX"}`

**State Triples (RDF):**
- `<urn:loop:current> dcterms:identifier ?session_id`
- `<urn:loop:current> loop:iterationNumber ?iteration`
- `<urn:loop:current> loop:maxIterations ?max`
- `<urn:loop:current> loop:completionPattern ?pattern`
- `<urn:loop:current> prov:value ?last_output`
- `<urn:loop:current> prov:startedAtTime ?timestamp`

**Dependency:** Requires `unrdf` CLI tool (RDF query/template engine).

---

### 3.2 Status Line Hook (UI Hook)

**Hook Type:** `statusLine`  
**Trigger:** Continuously displayed in Claude Code UI status bar  
**Command:**
```bash
~/.claude/statusline-command.sh
```

**Purpose:** Display real-time session information in Claude Code's status line.

**Displayed Metrics:**
```
[hostname] ॐ [basename_dir]:[git_branch][git_status_indicator] [session_name] [time] [context%]
```

**Elements:**
1. **Hostname:** Machine name
2. **Current Directory:** Basename only (shortened display)
3. **Git Branch:** Current branch (via `git rev-parse --abbrev-ref HEAD`)
4. **Git Status:** Asterisk (*) if there are uncommitted changes (via `git status --porcelain`)
5. **Session Name:** From Claude Code context
6. **Time:** HH:MM:SS format (dogenpunk theme style)
7. **Context Indicator:** Remaining context percentage with color:
   - 🟢 Green: >70%
   - 🟡 Yellow: 30-70%
   - 🔴 Red: <30%

**Script Location:** `/Users/sac/.claude/statusline-command.sh`

---

## Section 4: Claude Code Configuration Hooks

### Location
`/Users/sac/.claude/settings.json`

### Configuration Details

**Enabled Plugins (with hooks):**
- `claude-md-management@claude-plugins-official` (tracks CLAUDE.md updates)
- `ralph-loop@claude-plugins-official` (RDF loop orchestration)
- `rust-analyzer-lsp@claude-plugins-official` (Rust code intelligence)
- `jdtls-lsp@claude-plugins-official` (Java code intelligence)

**Permission Model:**
- **Default Mode:** `auto` (prompt on unknown operations)
- **Allowlist:** 30+ Bash command patterns (git, npm, python, cargo, etc.)
- **Denylist:** `Bash(rm -rf /)`
- **Skip Prompts:** `skipDangerousModePermissionPrompt`, `skipAutoPermissionPrompt`, `skipWorkflowUsageWarning`

**MCP Servers:**
- **open-ontologies:** `/Users/sac/chatmangpt/ostar/vendors/open-ontologies/target/release/open-ontologies serve`

---

## Section 5: Todo System Hook (Shell-Based)

### Location
`/Users/sac/.claude/todo-system.sh`

### 5.1 Todo System Initialization

**Trigger:** Manual invocation via `init_todo_system()`  
**Files Created:**
- `$HOME/.claude/todos.json` — Current todos database
- `$HOME/.claude/todo_history.json` — Historical todo completion log
- Backup: `$HOME/.claude/todos.backup.YYYYMMDD_HHMMSS.json`

**Purpose:** Manage 5-10+ concurrent todos with priority levels and status tracking.

**JSON Schema (todos.json):**
```json
{
  "todos": [
    {
      "id": 1,
      "title": "string",
      "description": "string",
      "priority": "high|medium|low",
      "tags": ["string"],
      "status": "pending|in_progress|completed|blocked"
    }
  ],
  "next_id": 2
}
```

### 5.2 Todo Operations

**Available Functions:**
1. `init_todo_system()` — Initialize JSON database
2. `add_todo(title, description, priority, tags)` — Create new todo
   - Priority validation: must be `high`, `medium`, or `low`
3. (Additional functions in full script — read from file for complete list)

---

## Section 6: Environment Variable Hooks

### 6.1 Global Environment Variables

**Sourced During Shell Startup:**

| Variable | Value | Source | Purpose |
|---|---|---|---|
| `ASDF_DIR` | `$HOME/.asdf` | zshrc | Version manager home |
| `ZSH` | `$HOME/.oh-my-zsh` | zshrc | Oh My Zsh framework path |
| `SDKMAN_DIR` | `$HOME/.sdkman` | zshrc | Java version manager home |
| `JAVA_HOME` | `/usr/local/opt/openjdk@26` or `.sdkman/candidates/java/current` | zshrc | Java installation path (openjdk@26 preferred) |
| `MAVEN_DAEMON_HOME` | `$HOME/.m2/mvnd` | zshrc | Maven daemon installation |
| `YAWL_DB_URL` | `jdbc:postgresql://localhost:5432/yawl` | zshrc | PostgreSQL connection string (default) |
| `YAWL_DB_USER` | `yawl` | zshrc | Database user (default) |
| `YAWL_DB_PASSWORD` | `yawl` | zshrc | Database password (default) |
| `ZAI_API_KEY` | (API key) | zshrc | Anthropic ZAI token |
| `GROQ_API_KEY` | (API key) | zshrc | Groq API token |
| `CARGO_REGISTRY_TOKEN` | (Token) | zshrc | Cargo crate registry auth |
| `ANTHROPIC_API_KEY` | (API key) | zshrc | Anthropic SDK token |

### 6.2 Path Modifications

**Order of PATH construction:**
1. Cargo env (from `.cargo/env`)
2. Local bin (from `.local/bin/env`)
3. Maven daemon (`~/.m2/mvnd/bin`)
4. SDKMAN paths (if active)
5. Homebrew (`/opt/homebrew/bin`)
6. System paths (`/usr/local/bin`, `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`)
7. Custom: A2A tools, maestro orchestrator, ggen

---

## Section 7: Project-Level Configuration (Workflow Hooks)

### Location
`/Users/sac/wasm4pm-compat/.claude/workflows/`

### Registered Workflows

These are Claude Code workflow scripts (available via skill invocation):

| Workflow | File | Purpose |
|---|---|---|
| `paperlaw-003-100-commit-sprint` | `paperlaw-003-100-commit-sprint.js` | Manufacture 100 receipt-bearing commits from PAPERLAW_ALIVE_002 |
| `paperlaw-003-residual-close` | `paperlaw-003-residual-close.js` | Close 8 papers + 5 fail fixtures + 26 pass fixtures → PAPERLAW_ALIVE_003 |
| `paperlaw-004-500-commit-crown` | `paperlaw-004-500-commit-crown.js` | Manufacture 500 receipt-bearing commits from PARTIAL_003 → CROWN_ALIVE_004 |
| `wasm4pm-compat-paperlaw-alive-002` | `wasm4pm-compat-paperlaw-alive-002.js` | Two coupled outcomes: ALIVE_002 + 2000+ commits |

---

## Section 8: Risk Assessment

### High-Risk Findings

1. **Unprotected API Keys in `.zshrc`**
   - **Location:** `/Users/sac/.zshrc` (lines with ANTHROPIC_API_KEY, GROQ_API_KEY, CARGO_REGISTRY_TOKEN)
   - **Risk:** Credentials committed to shell startup file, visible in process env, exposed in shell history
   - **Recommendation:** Move to `~/.env.local` (which is gitignored) and source only from there
   - **Current Protection:** `.env.local` loading mechanism exists but keys are also in `.zshrc`

2. **Dangerous Commands in `.bashrc`**
   - **Lines:** `sudo shutdown -h 0` (appears twice)
   - **Risk:** Accidental system shutdown if `.bashrc` is sourced
   - **Recommendation:** Remove or comment out these lines immediately

3. **RDF Loop State File Permissions**
   - **File:** `~/.claude/rdf-loop/state.ttl`
   - **Risk:** State file may contain sensitive loop metadata or prompts
   - **Check:** Verify file permissions (should be user-only readable)

4. **Transcript Path Exposure in Stop Hook**
   - **Behavior:** The Stop hook reads entire transcript files (JSONL)
   - **Risk:** May expose conversation history if transcript path is accessible
   - **Recommendation:** Verify transcript file permissions

### Medium-Risk Findings

1. **Multiple PATH Duplications**
   - `~/.maestro/bin` appears 3 times in PATH
   - Causes inefficient PATH lookups
   - **Fix:** Remove duplicates

2. **Missing Error Handling in Status Line Hook**
   - Git operations may fail in non-git directories
   - **Impact:** Status line may not render correctly
   - **Recommendation:** Add `|| echo "NO-GIT"` fallbacks (already done for hostname)

3. **Todo System Unversioned**
   - Todo JSON files are not in git (good)
   - But backup files are created in user home (may accumulate)
   - **Recommendation:** Add cleanup routine or put in tmp directory

---

## Section 9: Summary Table

| Hook Type | Name | Trigger | Active? | Location |
|---|---|---|---|---|
| Git | pre-commit | Before commit | No (sample) | `.git/hooks/pre-commit.sample` |
| Git | commit-msg | After commit message edit | No (sample) | `.git/hooks/commit-msg.sample` |
| Git | pre-push | Before push | No (sample) | `.git/hooks/pre-push.sample` |
| Shell | ASDF setup | Shell start | Yes | `~/.zshrc` |
| Shell | Oh My Zsh | Shell start | Yes | `~/.zshrc` → `~/.oh-my-zsh/oh-my-zsh.sh` |
| Shell | SDKMAN | Shell start | Yes | `~/.zshrc` → `~/.sdkman/bin/sdkman-init.sh` |
| Shell | Cargo env | Shell start | Yes | `~/.zshrc` → `~/.cargo/env` |
| Shell | UV Python | Shell start | Yes | `~/.zshrc` → `~/.uvrc` |
| Claude Code | Stop | Session exit | Yes | `~/.claude/rdf-loop/rdf-stop-hook.sh` |
| Claude Code | Status Line | Continuous | Yes | `~/.claude/statusline-command.sh` |
| Claude Code | Todo Init | Manual | On-demand | `~/.claude/todo-system.sh` |

---

## Appendix: File Permissions Check

**Commands to verify hook security:**
```bash
# Check git hooks (should all be .sample, no executables)
ls -la ~/.wasm4pm-compat/.git/hooks/ | grep -v sample

# Check zshrc permissions (should be user-readable only)
stat -f "%OLp %N" ~/.zshrc

# Check claude code settings
stat -f "%OLp %N" ~/.claude/settings.json

# Check for setuid/setgid bits (dangerous)
find ~/.claude -type f -perm +4000 -o -perm +2000
```

**All hooks should have permissions:**
- Readable by user: ✓
- Writable by user: ✓
- No world-readable secrets: Verify (see Section 8, High-Risk #1)

---

## References

- [Git Hooks Documentation](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- [Oh My Zsh Plugin System](https://github.com/ohmyzsh/ohmyzsh/wiki)
- [ASDF Version Manager](https://asdf-vm.com/)
- [SDKMAN Documentation](https://sdkman.io/)
- [RDF/Turtle Format (W3C)](https://www.w3.org/TR/turtle/)
- [PROV-O Ontology](https://www.w3.org/TR/prov-o/)
