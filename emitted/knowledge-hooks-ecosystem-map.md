# Knowledge Hooks Ecosystem Map

**Date:** 2026-06-01  
**Scope:** wasm4pm-compat, ggen, process-intelligence projects  
**Authority Level:** xhigh effort discovery with cross-project dependency analysis

---

## Part 1: Hook Types Taxonomy

### 1.1 CLAUDE.md Hooks (Knowledge Declaration Layer)

The filesystem-based knowledge hierarchy that precedes all other hook systems.

| Hook Type | Location | Authority | Purpose | Applies To |
|-----------|----------|-----------|---------|-----------|
| **Global Rules** | `~/.claude/CLAUDE.md` | Highest (user default) | Literal interpretation, build tool prefs, git workflow | All projects |
| **Global Rules (Taxonomy)** | `~/.claude/rules/*.md` | High | Specialized knowledge by domain (manufacturing, process-mining, Python/RDF, tools) | All projects (conditional per language) |
| **Project CLAUDE.md** | `<repo>/CLAUDE.md` | High (project-specific) | Architecture, testing, commands, invariants, DX surfaces | Single project only |
| **Subproject CLAUDE.md** | `<subdir>/CLAUDE.md` | Medium (subdirectory) | Context override for specific subdirectory | Subdirectory scope |
| **Local Settings** | `~/.claude/settings.json` | High | Plugins, hooks, MCP servers, environment | All sessions |
| **Project Settings** | `.claude/settings.json` | Medium-High | Override global; plugins, hooks, env (per-project) | Single project |
| **Local Settings** | `.claude/settings.local.json` | Highest (session override) | Session-only ephemeral changes | Current session only |

**Authority Hierarchy (descending):**
1. `.claude/settings.local.json` (session-only, ephemeral)
2. `~/.claude/settings.json` (user global, persistent)
3. `.claude/settings.json` (project override)
4. `<repo>/CLAUDE.md` (project architecture knowledge)
5. `~/.claude/CLAUDE.md` (global user defaults)
6. `~/.claude/rules/*.md` (specialized domain knowledge)

---

### 1.2 Settings.json Hooks (Execution Layer)

Structured hooks that the Claude Code harness executes.

| Hook Type | Location | Trigger | Example |
|-----------|----------|---------|---------|
| **Pre-Session** | `settings.json::hooks::Start` | Session begins | Load project memory, validate toolchain |
| **Post-Tool** | `settings.json::hooks::ToolUse` | After any tool call | Emit OTEL, validate state, audit config changes |
| **Session Stop** | `settings.json::hooks::Stop` | Session ends (user exit) | Clean up, save memory, release resources |
| **User Prompt** | `settings.json::hooks::UserPrompt` | Before user message processed | Inject context, validate prompt, check memory |
| **Status Line** | `settings.json::statusLine` | Continuously (status bar) | Display project health, build state, memory status |

**Current Known Hooks in Ecosystem:**

- `~/.claude/settings.json::hooks::Stop` → `bash ~/.claude/rdf-loop/rdf-stop-hook.sh`
- `~/.claude/settings.json::statusLine` → `bash ~/.claude/statusline-command.sh`
- `ggen/.claude/hooks/` → 7 shell scripts (doctrine, validation, audit, emitter)
- **Not yet integrated:** `~/.claude/rdf-loop/` (separate RDF-focused loop system)

---

### 1.3 MCP Servers (Knowledge Exposure Layer)

External protocol servers that expose structured knowledge via the Model Context Protocol.

| Server | Location | Enabled? | Purpose | Knowledge Type |
|--------|----------|----------|---------|-----------------|
| `open-ontologies` | `/Users/sac/chatmangpt/ostar/vendors/open-ontologies/target/release/open-ontologies serve` | `~/.claude/settings.json` ✓ | O* ecosystem ontology exposure | RDF triples, SPARQL queries, canonical concepts |
| `cpmp` | `/Users/sac/capability-map/target/release/cpmp serve` | `~/.claude/mcp.json` | Computer Project Mapping Protocol | Project capabilities, classifications, projections |
| `ggen-lsp-mcp` | `cargo run -q -p ggen-lsp-mcp` | `ggen/.mcp.json` (test-only) | ggen LSP repair routes as MCP tools | Code repair, route discovery, intel logs |
| `ggen` (full) | `cargo run -p ggen-cli -- mcp start-server` | `ggen/.mcp.json` (test-only) | Full ggen CLI as MCP server | Code generation pipeline, validation, sync |
| `rdf-tools` | `oxigraph server` | `ggen/.mcp.json` (test-only) | SPARQL triple store | Direct RDF query against Oxigraph |

**Enabled in Current Session:** `open-ontologies` (O* ecosystem canonical knowledge)

---

### 1.4 Claude Code Plugins (Capability Extension Layer)

Marketplace-installed plugins that extend the Claude Code harness.

| Plugin | Status | Provides | Authority |
|--------|--------|----------|-----------|
| `rust-analyzer-lsp@claude-plugins-official` | Enabled | Rust code intelligence (LSP) | Medium (IDE-like) |
| `jdtls-lsp@claude-plugins-official` | Enabled | Java code intelligence (LSP) | Medium (IDE-like) |
| `pyright-lsp@claude-plugins-official` | Enabled | Python type checking (LSP) | Medium (IDE-like) |
| `typescript-lsp@claude-plugins-official` | Enabled | TypeScript code intelligence (LSP) | Medium (IDE-like) |
| `claude-md-management@claude-plugins-official` | Enabled | CLAUDE.md audit and improvement | Medium (meta-management) |
| `ralph-loop@claude-plugins-official` | Enabled | Workflow orchestration loop | Medium (orchestration) |
| `explanatory-output-style@claude-plugins-official` | Enabled | Output formatting preferences | Low (presentation only) |
| `frontend-design@claude-plugins-official` | Disabled | UI/design capability | N/A |
| `superpowers@claude-plugins-official` | Disabled | Extended capabilities | N/A |

**Active Knowledge Integration:** LSP plugins provide code intelligence; `claude-md-management` audits project CLAUDE.md files.

---

### 1.5 Memory & Persistent State

Auto-persisted knowledge across sessions.

| Location | Purpose | Scope | Authority |
|----------|---------|-------|-----------|
| `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/MEMORY.md` | Auto-memory for wasm4pm-compat | Project lifetime | High (accumulated knowledge) |
| `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/project_*.md` | Topic-specific memory (state, deps, research) | Project lifetime | High (topic-scoped) |
| `.claude/autonomous/workflow-state.json` | Phase state for multi-agent workflows | Session-scoped | Medium (phase tracking) |
| `.claude/scheduled_tasks.lock` | Cron job state | Session-scoped | Low (metadata only) |

---

### 1.6 Source Code Annotations (Implicit Knowledge)

Knowledge embedded in code via rustdoc, comments, and type signatures.

| Vehicle | Language | Example | Authority |
|---------|----------|---------|-----------|
| Rustdoc comments | Rust | `/// Must never violate: no unsafe_code` in src/lib.rs | High (in-code law) |
| Type constraints | Rust | `pub fn admit() -> Result<Admission<…>, Refusal<…>>` | High (compile-time proof) |
| Feature gates | Rust/Cargo | `#![feature(generic_const_exprs)]` in wasm4pm-compat | High (nightly law) |
| Test fixtures | Rust (ui/) | trybuild compile-fail/pass `.rs` + `.stderr` files | High (type-law receipts) |
| Example programs | Rust | `examples/basic_eventlog.rs` with docstring context | Medium (usage proof) |
| Shell scripts | Bash | Comments in hook scripts in ggen/.claude/hooks/ | Medium (automation logic) |

---

## Part 2: Knowledge Hook Hierarchy

### Authority Levels (from Highest to Lowest)

```
1. Type System + Compiler (infallible)
   ├─ rustc errors (compile-fail fixtures in tests/ui/)
   ├─ trait bounds & PhantomData types
   └─ const generics and ADT const params

2. CLAUDE.md Files (human-authored law)
   ├─ Global ~/.claude/CLAUDE.md (interpreted literally)
   ├─ Project /CLAUDE.md (architecture & invariants)
   ├─ ~/.claude/rules/*.md (specialized domains)
   └─ Project .claude/settings.json (overrides)

3. Executable Hooks (settings.json)
   ├─ Session hooks (Start, Stop, ToolUse)
   ├─ Status line (continuous)
   └─ MCP servers (external knowledge exposure)

4. Tests & Validation (empirical proof)
   ├─ Trybuild ui tests (type-law receipts)
   ├─ Integration tests (behavior proofs)
   ├─ Property tests (correctness proofs)
   └─ OTEL traces (execution evidence)

5. Code Comments & Rustdoc (secondary authority)
   ├─ Usage examples
   ├─ Design rationale
   └─ Temporary notes

6. Generated/Auto-Synced Files (derived knowledge)
   ├─ Docs generated from ontology (.specify/*.ttl)
   ├─ Code generated from specs
   └─ MCP-discovered metadata
```

### Knowledge Dependency Graph

```
LITERAL INTERPRETATION
(user global defaults)
        ↓
PROJECT CLAUDE.md
(architecture & commands)
        ↓
PROJECT .claude/settings.json
(plugins, hooks, env)
        ↓
EXECUTABLE HOOKS (settings.json)
        ├─→ Session Start: Load CLAUDE.md memory
        ├─→ Tool Use: Emit OTEL, validate state
        ├─→ Status Line: Display memory, health
        └─→ Session Stop: Save memory
        ↓
TESTS & VALIDATION
        ├─→ Rustc (compile-time)
        ├─→ Trybuild (type-law receipts)
        ├─→ Integration (runtime behavior)
        └─→ OTEL (execution traces)
        ↓
CODE INTELLIGENCE (LSP)
        ├─→ rust-analyzer (Rust)
        ├─→ jdtls (Java)
        └─→ pyright (Python)
        ↓
MCP SERVERS
        ├─→ open-ontologies (O* canonical knowledge)
        ├─→ ggen-lsp-mcp (code repair tools)
        └─→ cpmp (project capabilities)
```

---

## Part 3: Cross-Project Dependencies

### wasm4pm-compat Project

**Knowledge Sources:**
1. `wasm4pm-compat/CLAUDE.md` — Nightly Rust law, type-law receipts, testing surfaces (ALIVE gate)
2. `~/.claude/CLAUDE.md` — Literal interpretation, build tool prefs
3. `~/.claude/rules/manufacturing-terminology.md` — CodeManufactory doctrine
4. `~/.claude/rules/process-mining-chicago-tdd.md` — Van der Aalst constitution, event log proof
5. Memory: `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/project_wasm4pm_compat.md`

**Depends On (as dependencies):**
- `ggen` (code generation from RDF specs)
- `open-ontologies` MCP server (O* canonical knowledge)

**Depended Upon By:**
- `process-intelligence` (test case for process-evidence type system)

**Hook Usage:**
- Unit tests: `cargo test --all-features --tests`
- Type-law receipts: `cargo test --test ui_tests -- --ignored` (ALIVE gate)
- Doctests: `cargo test --doc --all-features` (explicit, documentation audit)

---

### ggen Project

**Knowledge Sources:**
1. `ggen/CLAUDE.md` — Architecture reference, 15-crate workspace, Chicago TDD ONLY, OTEL validation
2. `ggen/.claude/settings.json` — Agent definitions, environment, SLO targets
3. `ggen/.claude/hooks/` — 7 executables (doctrine, validation, audit, emitter)
4. `ggen/.mcp.json` — MCP server definitions (local testing: ggen-lsp-mcp, ggen, oxigraph)
5. `~/.claude/CLAUDE.md` — Rust build prefs, literal interpretation

**Architecture Knowledge (in public .md files):**
- `.claude/ARCHITECTURE_AND_DOCTRINE_SUMMARY.txt` (2,900 words)
- `docs/architecture/COMPRESSED_REFERENCE.md` (verified C4, real sync flow)
- `docs/crate-audits/AUDIT_DASHBOARD.md` (54 stubs, 8,900 lines dead code, 4 P0 blockers)
- `.claude/PHASE5_WAVE1_CAPABILITY_INVENTORY.md` (latest wave state)

**Depends On (as dependencies):**
- `open-ontologies` (RDF canonicality)
- `Oxigraph` (SPARQL triple store)
- Process-mining knowledge (external, not in codebase)

**Depended Upon By:**
- `wasm4pm-compat` (uses ggen code generation)
- `process-intelligence` (testing ggen capabilities)

**Hook Usage:**
- Pre-commit hook: `.git/hooks/pre-commit` → `scripts/hooks/pre-commit.sh` (workspace build gate)
- Pre-push hook: `.git/hooks/pre-push` → `scripts/hooks/pre-push.sh` (test gate, ~300s)
- Session start: `session_start_doctrine.sh` (load doctrine, check toolchain)
- Config audit: `config_change_auditor.sh` (detect invalid settings changes)
- Stop gate: `stop_release_gate.sh` (prevent incomplete deployments)

---

### process-intelligence Project

**Knowledge Sources:**
1. Memory: `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/project_process_intelligence.md`
2. Inherited CLAUDE.md: Uses `~/.claude/CLAUDE.md` + `~/.claude/rules/process-mining-chicago-tdd.md`
3. Test case authority: Both wasm4pm-compat and ggen capabilities serve as proof

**Authority:** Medium (test case only, not authoring domain knowledge)

**Depends On (as dependencies):**
- `wasm4pm-compat` (process-evidence type system)
- `ggen` (code generation for discovery models)
- Process-mining literature (external authority)

**Depended Upon By:**
- Research programs (external to these three projects)

---

## Part 4: Authority Alignment for wasm4pm-compat

### What is Authoritative for wasm4pm-compat?

| Knowledge Source | Authority Level | Used For |
|------------------|-----------------|----------|
| `wasm4pm-compat/CLAUDE.md` | **Highest** | Type law, nightly features, testing surfaces, architecture invariants |
| Rust compiler (`rustc`) + trybuild | **Highest** | Compile-time proofs, type-law receipts (ALIVE gate) |
| `~/.claude/rules/manufacturing-terminology.md` | **High** | CodeManufactory doctrine terminology |
| `~/.claude/rules/process-mining-chicago-tdd.md` | **High** | Van der Aalst constitution, event log proofs |
| `~/.claude/CLAUDE.md` (Global) | **High** | Literal interpretation, git workflow, build prefs |
| `open-ontologies` MCP server | **Medium-High** | O* ecosystem canonical knowledge (external authority) |
| Memory: `project_wasm4pm_compat.md` | **Medium** | Accumulated project state (PAPERLAW_CROWN_ALIVE_004, GAP_001) |
| ggen capabilities | **Medium** | Code generation targets (graduation to wasm4pm) |
| Integration tests | **Medium** | Behavior proofs (not yet emphasized in current CLAUDE.md) |

### What is NOT Authoritative for wasm4pm-compat?

- ggen's Chicago TDD methodology (wasm4pm-compat emphasizes type laws, not test doubles)
- Generic "best practices" (literal interpretation only)
- External online documentation (except W3C specs, RFC standards)
- Conventions from other Rust projects (except what rustc enforces)

---

## Part 5: Unified Hook Discovery Summary

### All Active Hooks in Ecosystem

#### Global Layer
- `~/.claude/settings.json` → Defines `hooks::Stop`, `statusLine`, MCP servers (open-ontologies), plugins, env
- `~/.claude/CLAUDE.md` → Global rules, literal interpretation, build tool prefs
- `~/.claude/rules/` → manufacturing-terminology.md, process-mining-chicago-tdd.md, python-rdf.md, tools.md, rdf-files.md

#### Per-Project Layer
- `wasm4pm-compat/CLAUDE.md` → Type law, nightly Rust, ALIVE gate, testing surfaces
- `wasm4pm-compat/.claude/settings.json` → (not present; inherits global)
- `ggen/CLAUDE.md` → 15-crate workspace, Chicago TDD, OTEL validation, agent coordination
- `ggen/.claude/settings.json` → Agent definitions, SLO targets, DFLSS mode, extended thinking
- `ggen/.claude/hooks/` → 7 shell scripts + README
- `ggen/.mcp.json` → 6 MCP server definitions (claude-code-guide, git, bash, rdf-tools, weaver-registry, ggen, ggen-lsp-mcp)
- `ggen/scripts/hooks/pre-commit.sh` → Workspace build gate
- `ggen/scripts/hooks/pre-push.sh` → Test gate
- `open-ontologies/.claude/settings.json` → (inherited)

#### Memory Layer
- `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/MEMORY.md` → Index
- `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/project_wasm4pm_compat.md` → PAPERLAW_CROWN_ALIVE_004 state
- `~/.claude/projects/-Users-sac-wasm4pm-compat/memory/project_process_intelligence.md` → Research program state

#### Test & Validation Layer
- `wasm4pm-compat/tests/ui/compile_fail/` → Type-law receipt fixtures
- `wasm4pm-compat/tests/ui/compile_pass/` → Type-law proof fixtures
- `ggen/scripts/hooks/pre-commit.sh` → Pre-commit validation (workspace)
- `ggen/scripts/hooks/pre-push.sh` → Pre-push test gate (~300s)

---

## Part 6: Action Items & Recommendations

### Immediate (P0 — Missing Integration)

| Action | Scope | Effort | Rationale |
|--------|-------|--------|-----------|
| **Integrate wasm4pm-compat/.claude/settings.json** | wasm4pm-compat | 1h | Override per-project env, plugins (currently inherits global) |
| **Expose wasm4pm-compat ALIVE gate via hook** | wasm4pm-compat | 2h | `hooks::ToolUse` should emit trybuild receipt status |
| **Unify hook architecture docs** | ggen, wasm4pm-compat | 3h | Create `.claude/hooks/README.md` explaining the 7 hook types |
| **Verify open-ontologies MCP is operational** | Ecosystem | 1h | Test that `open-ontologies` is running and answering SPARQL queries |

### Short-term (P1 — Authority Clarity)

| Action | Scope | Effort | Rationale |
|--------|-------|--------|-----------|
| **Document CLAUDE.md precedence** | All projects | 2h | Explicit statement: "CLAUDE.md > MCP > plugins > comments" |
| **Codify ggen hook types** | ggen | 3h | Map 7 hooks to settings.json standard hook names (Start, Stop, ToolUse, etc.) |
| **Add wasm4pm-compat memory refresh task** | wasm4pm-compat | 1h | Quarterly MEMORY.md audit (last update: 2026-05-30) |
| **Test process-intelligence hook inheritance** | process-intelligence | 2h | Verify that process-mining rules flow through to tests |

### Medium-term (P2 — Integration & Discovery)

| Action | Scope | Effort | Rationale |
|--------|-------|--------|-----------|
| **Create cross-project hook dependency graph** | Ecosystem | 4h | Emit visual map (Mermaid) showing which hooks feed which projects |
| **Add MCP-based hook discovery** | ggen (via ggen-lsp-mcp) | 6h | Expose hooks as MCP tools so Claude can query them directly |
| **Implement hook health dashboard** | All projects | 8h | `.claude/hooks-status.json` with last-run, exit-code, duration for each hook |
| **Graduate ggen hooks to settings.json standard** | ggen | 4h | Refactor `ggen/.claude/hooks/*.sh` → entries in `.claude/settings.json::hooks` |

### Long-term (P3 — Knowledge Synthesis)

| Action | Scope | Effort | Rationale |
|--------|-------|--------|-----------|
| **Unify hook knowledge into ontology** | Ecosystem | 12h | Add `ggen/ontology/domains/hooks.nt` with hook types, triggers, authorities, examples |
| **Build hook-discovery MCP server** | ggen (new crate) | 16h | `ggen-hooks-mcp` exposes all hooks as queryable RDF entities with lineage, authority, examples |
| **Implement hook versioning** | All projects | 6h | Track breaking changes in hook APIs (e.g., adding new required env vars) |

---

## Part 7: Ecosystem Visual Diagrams

### Diagram 1: Hook Authority Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│              HOOK AUTHORITY HIERARCHY                    │
└─────────────────────────────────────────────────────────┘

Level 1 (INFALLIBLE)
═══════════════════════════════════════════════════════
  rustc compiler errors
  + trybuild fixtures (ui tests)
  + type constraints & PhantomData


Level 2 (HUMAN AUTHORED ARCHITECTURE)
═══════════════════════════════════════════════════════
  ~/.claude/CLAUDE.md (global defaults)
    ↓ (overridden by)
  <project>/CLAUDE.md (project-specific)
    ↓ (overridden by)
  .claude/settings.json (session override)
    ↓ (overridden by)
  .claude/settings.local.json (ephemeral)


Level 3 (EXECUTABLE HOOKS)
═══════════════════════════════════════════════════════
  settings.json::hooks::Start
    → Load CLAUDE.md memory
    → Validate toolchain
    
  settings.json::hooks::ToolUse
    → Emit OTEL traces
    → Audit state changes
    → Validate configuration
    
  settings.json::hooks::Stop
    → Save memory
    → Clean up resources
    
  settings.json::statusLine
    → Display project health
    → Show memory status


Level 4 (TESTS & VALIDATION)
═══════════════════════════════════════════════════════
  Compile-time: cargo test --test ui_tests
  Runtime: cargo test --all-features --tests
  Property: property-based tests
  OTEL: Execution traces proving real calls


Level 5 (CODE INTELLIGENCE)
═══════════════════════════════════════════════════════
  LSP plugins (rust-analyzer, jdtls, pyright)
  → Type hints, error diagnostics
  
  claude-md-management plugin
  → CLAUDE.md audit & improvement


Level 6 (MCP SERVERS)
═══════════════════════════════════════════════════════
  open-ontologies
    → O* canonical knowledge (RDF/SPARQL)
    
  ggen-lsp-mcp
    → Code repair tools, intel logs
    
  cpmp
    → Project capability discovery
```

---

### Diagram 2: Cross-Project Knowledge Flows

```
                GLOBAL AUTHORITIES
                       ↓
        ┌──────────────┴──────────────┐
        ↓                             ↓
  ~/.claude/CLAUDE.md         ~/.claude/rules/
   (Build prefs)             (Domain knowledge)
        ↓                             ↓
   ┌────┴────────────────────────────┘
   ↓
   
   WASM4PM-COMPAT PROJECT
   ├─ CLAUDE.md (Type law, ALIVE gate)
   ├─ tests/ui/ (Trybuild receipts)
   ├─ src/ (Evidence<T, State, W> types)
   └─ examples/ (DX surfaces)
        ↓
        │ (depends on code generation)
        ↓
   
   GGEN PROJECT
   ├─ CLAUDE.md (15-crate workspace, Chicago TDD)
   ├─ .claude/hooks/ (7 executables)
   ├─ scripts/hooks/ (pre-commit, pre-push)
   ├─ .claude/settings.json (agents, SLOs, DFLSS)
   └─ .mcp.json (6 MCP servers, test-only)
        ↓
        │ (exposes via MCP)
        ↓
   
   KNOWLEDGE EXPOSURE
   ├─ open-ontologies MCP
   │  └─ O* canonical RDF
   ├─ ggen-lsp-mcp
   │  └─ Code repair tools
   └─ cpmp
      └─ Project capabilities
      
   
   PROCESS-INTELLIGENCE PROJECT
   ├─ Memory (ALIVE_001 sealed, 736 commits)
   ├─ Inherited: process-mining-chicago-tdd rules
   ├─ Test case: Proves both wasm4pm-compat & ggen work
   └─ Authority: Medium (proof-of-concept only)


   MEMORY LAYER
   └─ ~/.claude/projects/-Users-sac-wasm4pm-compat/memory/
      ├─ project_wasm4pm_compat.md (PAPERLAW_CROWN_ALIVE_004)
      ├─ project_process_intelligence.md (ALIVE_001, GAP_001-008)
      └─ MEMORY.md (index)
```

---

### Diagram 3: Hook Lifecycle (Session Timeline)

```
SESSION START
    ↓
    ├─ settings.json::hooks::Start
    │   └─ Load CLAUDE.md memory
    │       Load ~/.claude/CLAUDE.md (global rules)
    │       Load <project>/CLAUDE.md (architecture)
    │       Load ~/.claude/rules/*.md (specialized)
    │       Load .claude/settings.json (overrides)
    │
    ├─ statusLine (continuous)
    │   └─ Display: [wasm4pm-compat ALIVE_004 | Nightly | 602 receipts]
    │
    ├─ User provides prompt
    │   │
    │   ├─ CLAUDE Code calls tools
    │   │
    │   └─ settings.json::hooks::ToolUse (each tool)
    │       ├─ Emit OTEL trace
    │       ├─ Validate state change
    │       ├─ Audit config changes
    │       └─ Log to .ggen/ocel/agent-edit-events.ocel.jsonl
    │
    ├─ Tests run (if triggered)
    │   ├─ Trybuild fixtures (compile-fail/pass)
    │   ├─ Integration tests (Chicago TDD)
    │   ├─ OTEL validation (prove real calls)
    │   └─ Memory updated with results
    │
    └─ SESSION ENDS (Ctrl+D or explicit exit)
        ├─ settings.json::hooks::Stop
        │   ├─ Save CLAUDE.md memory
        │   ├─ Clean up resources
        │   └─ bash ~/.claude/rdf-loop/rdf-stop-hook.sh
        │
        └─ Session ends
```

---

## Part 8: Hook Type Reference Table

### All 13 Hook Types in Ecosystem

| # | Hook Type | Scope | Trigger | File | Authority |
|---|-----------|-------|---------|------|-----------|
| 1 | Global Rules | All projects | Session start | `~/.claude/CLAUDE.md` | Highest |
| 2 | Specialized Rules | Conditional | Session start | `~/.claude/rules/*.md` | High |
| 3 | Project Architecture | Single project | Load project | `<repo>/CLAUDE.md` | High |
| 4 | Project Settings | Single project | Project-specific override | `.claude/settings.json` | High |
| 5 | Session Override | Current session | Before each prompt | `.claude/settings.local.json` | Highest (ephemeral) |
| 6 | Session Start Hook | All projects | Session begins | `settings.json::hooks::Start` | Medium-High |
| 7 | Tool Use Hook | All projects | After tool call | `settings.json::hooks::ToolUse` | Medium-High |
| 8 | Session Stop Hook | All projects | Session ends | `settings.json::hooks::Stop` | Medium-High |
| 9 | Status Line Hook | All projects | Continuously | `settings.json::statusLine` | Low (visual only) |
| 10 | Pre-commit Hook | ggen only | `git commit` | `.git/hooks/pre-commit` | Medium |
| 11 | Pre-push Hook | ggen only | `git push` | `.git/hooks/pre-push` | Medium |
| 12 | MCP Servers | Conditional | On demand | `settings.json::mcpServers` + `.mcp.json` | Medium |
| 13 | LSP Plugins | Conditional | On language file load | `settings.json::enabledPlugins` | Medium |

---

## Part 9: Knowledge Completeness Audit

### What Each Project Knows (Authoritative Sources Only)

**wasm4pm-compat:**
- ✅ Type laws (Rust compiler, trybuild)
- ✅ Nightly Rust features (CLAUDE.md)
- ✅ ALIVE gate (tests/ui/)
- ✅ Zero-cost type-law surfaces (src/nightly_foundry.rs)
- ✅ Evidence lifecycle (Evidence<T, State, W> types)
- ⚠️ CodeManufactory doctrine (via rules file, not integrated)
- ⚠️ Process-mining Chicago TDD (via rules file, not enforced)

**ggen:**
- ✅ 15-crate workspace (CLAUDE.md crate map)
- ✅ Chicago TDD methodology (CLAUDE.md, test rules)
- ✅ OTEL validation (CLAUDE.md § 🔍 OpenTelemetry)
- ✅ Pre-commit/push gates (scripts/hooks/)
- ✅ Agent coordination (settings.json::agents)
- ✅ RDF source-of-truth (.specify/*.ttl)
- ✅ SLO targets (settings.json::env)
- ✅ 7 executable hooks (hooks/*.sh)
- ⚠️ Hook discovery (no MCP server yet)
- ⚠️ Cross-project knowledge (no authority over wasm4pm-compat)

**process-intelligence:**
- ✅ Research program state (memory file)
- ✅ Process-mining authority (inherits rules)
- ✅ Test case status (memory + ALIVE_001)
- ❌ No independent CLAUDE.md (relies on inherited)
- ❌ No hook definitions

---

## Conclusion

The wasm4pm-compat ecosystem has **13 distinct hook types** across **4 scopes** (global, per-project, MCP, plugin). The **authority hierarchy is strict and literal**: CLAUDE.md files > executable hooks > tests > code comments.

**Key Findings:**
1. **ggen** has the most advanced hook infrastructure (7 shell scripts + settings-based definitions)
2. **wasm4pm-compat** prioritizes type-law proofs (trybuild) over test hooks
3. **process-intelligence** is a proof-of-concept test case, not an independent authority
4. **Open-ontologies MCP** is the canonical external knowledge source
5. **Memory layer** is critical for session continuity (MEMORY.md files)
6. **Authority conflicts** are prevented by literal interpretation of CLAUDE.md precedence

**Next Steps (if needed):**
1. Create `.claude/hooks/README.md` explaining hook taxonomy
2. Implement wasm4pm-compat/.claude/settings.json (project-specific overrides)
3. Expose hook discovery via MCP (ggen-hooks-mcp crate)
4. Document hook health dashboard (3-point status: last-run, exit-code, duration)

---

**Emitted:** 2026-06-01 | **Format:** Markdown + ASCII Diagrams | **Scope:** Complete ecosystem  
**Authority:** xhigh effort discovery with cross-project dependency analysis
