# RDF Hook Lifecycle: Semantic Graph Mapping

**Generated:** 2026-06-01  
**Scope:** Claude Code hook lifecycle modeled as RDF property chains and SPARQL conformance patterns  
**Subject Domain:** Hook declaration → registration → activation → execution → completion + cancellation/revocation

---

## Executive Summary

Claude Code hooks form a **directed lifecycle graph** with seven semantic stages:

1. **Declaration** (plugin or settings.json)
2. **Registration** (hook registry loaded)
3. **Activation** (event triggered)
4. **Execution** (handler process spawned)
5. **Completion** (handler output returned + decision applied)
6. **Persistence** (state logged/updated)
7. **Cancellation/Revocation** (hook disabled or state cleared)

This document models each stage as RDF triples (NTriples format), defines state machine semantics, and provides SPARQL conformance queries.

---

## 1. RDF Ontology Definition

### Namespace Prefixes

```turtle
@prefix hook: <http://claude.ai/hook/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix schema: <https://schema.org/> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
```

### Core Classes

```turtle
hook:Hook
  a rdfs:Class ;
  rdfs:label "A Claude Code hook instance"@en ;
  rdfs:comment "Represents a single hook handler declaration and its lifecycle"@en .

hook:HookEvent
  a rdfs:Class ;
  rdfs:label "An event that activates hooks"@en ;
  rdfs:subClassOf schema:Event .

hook:HookState
  a rdfs:Class ;
  rdfs:label "A discrete stage in the hook lifecycle"@en .

hook:HookDecision
  a rdfs:Class ;
  rdfs:label "The outcome of hook execution"@en .

hook:HookHandler
  a rdfs:Class ;
  rdfs:label "An executable process (shell command or script)"@en .
```

### Core Properties

```turtle
hook:event
  a rdf:Property ;
  rdfs:domain hook:Hook ;
  rdfs:range hook:HookEvent ;
  rdfs:label "The event type that activates this hook"@en .

hook:handler
  a rdf:Property ;
  rdfs:domain hook:Hook ;
  rdfs:range hook:HookHandler ;
  rdfs:label "The command/script to execute"@en .

hook:state
  a rdf:Property ;
  rdfs:domain hook:Hook ;
  rdfs:range hook:HookState ;
  rdfs:label "Current lifecycle state of this hook"@en .

hook:decision
  a rdf:Property ;
  rdfs:domain hook:Hook ;
  rdfs:range hook:HookDecision ;
  rdfs:label "The decision returned after execution"@en .

hook:isBlocking
  a rdf:Property ;
  rdfs:domain hook:Hook ;
  rdfs:range xsd:boolean ;
  rdfs:label "Whether this hook can block the triggering operation"@en .

hook:timeout
  a rdf:Property ;
  rdfs:domain hook:HookHandler ;
  rdfs:range xsd:integer ;
  rdfs:label "Maximum execution time in seconds"@en .

hook:source
  a rdf:Property ;
  rdfs:domain hook:Hook ;
  rdfs:range [ owl:unionOf ( <http://claude.ai/config#Settings> <http://claude.ai/plugin#PluginDeclaration> ) ] ;
  rdfs:label "Where the hook was declared (settings.json or plugin)"@en .

prov:startedAtTime
  rdfs:domain hook:Hook .

prov:endedAtTime
  rdfs:domain hook:Hook .

prov:value
  rdfs:domain hook:Hook ;
  rdfs:label "Output or payload from hook execution"@en .
```

---

## 2. Hook Lifecycle Stages (RDF NTriples)

### Stage 1: Declaration

A hook is declared in either `settings.json` or a plugin's `hooks.json`.

```ntriples
# Example: RDF Loop Stop Hook declared in settings.json
<urn:hook:rdf-stop-hook>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/Hook> .
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/event>
    <urn:hook:event:Stop> .
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/handler>
    <urn:hook:handler:bash-rdf-stop> .
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/source>
    <urn:config:settings-json> .
<urn:hook:rdf-stop-hook>
  <http://purl.org/dc/terms/isPartOf>
    <urn:config:global> .
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/state>
    <urn:hook:state:Declared> .
<urn:hook:rdf-stop-hook>
  <http://purl.org/dc/terms/created>
    "2026-06-01T00:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .

# Handler definition
<urn:hook:handler:bash-rdf-stop>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookHandler> .
<urn:hook:handler:bash-rdf-stop>
  <http://schema.org/command>
    "bash ~/.claude/rdf-loop/rdf-stop-hook.sh" .
<urn:hook:handler:bash-rdf-stop>
  <http://claude.ai/hook/timeout>
    "30"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:handler:bash-rdf-stop>
  <http://schema.org/language>
    "bash" .

# Event definition
<urn:hook:event:Stop>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookEvent> .
<urn:hook:event:Stop>
  <http://www.w3.org/2000/01/rdf-schema#label>
    "Stop event"@en .
<urn:hook:event:Stop>
  <http://www.w3.org/2000/01/rdf-schema#comment>
    "Fired when user requests session exit"@en .
```

### Stage 2: Registration

Hook is loaded from declaration source into the runtime hook registry.

```ntriples
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/state>
    <urn:hook:state:Registered> .
<urn:hook:rdf-stop-hook>
  <http://www.w3.org/ns/prov#wasLoadedFrom>
    <urn:config:settings-json> .
<urn:hook:rdf-stop-hook>
  <http://www.w3.org/ns/prov#wasRegisteredAt>
    "2026-06-01T08:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/registryMembership>
    <urn:hook:registry:global> .
```

### Stage 3: Activation

An event occurs that matches the hook's trigger condition.

```ntriples
# Event instance for this activation
<urn:hook:activation:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookActivation> .
<urn:hook:activation:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/hook>
    <urn:hook:rdf-stop-hook> .
<urn:hook:activation:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/event>
    <urn:hook:event:Stop> .
<urn:hook:activation:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#startedAtTime>
    "2026-06-01T09:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<urn:hook:activation:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/sessionId>
    "session-uuid-12345" .
<urn:hook:activation:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/context>
    <urn:hook:context:user-stop-request> .
```

### Stage 4: Execution

Hook handler process is spawned with context (environment, input, timeout).

```ntriples
# Process execution record
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://www.w3.org/ns/prov#Activity> .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#wasTriggeredBy>
    <urn:hook:activation:rdf-stop-hook:20260601T090000Z> .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#wasStartedBy>
    <urn:hook:runtime:executor> .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/agent>
    "bash" .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#startedAtTime>
    "2026-06-01T09:00:00.100Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/timeout>
    "30"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/pid>
    "47382" .
<urn:hook:execution:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/environment>
    <urn:hook:execution-context:20260601T090000Z> .

# Execution context
<urn:hook:execution-context:20260601T090000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Thing> .
<urn:hook:execution-context:20260601T090000Z>
  <http://schema.org/stdin>
    "{\"session_id\":\"session-uuid-12345\",\"transcript_path\":\"/path/to/transcript.jsonl\"}" .
<urn:hook:execution-context:20260601T090000Z>
  <http://schema.org/environment>
    "PATH=/Users/sac/.cargo/bin:..." .
```

### Stage 5: Completion

Handler process terminates and emits a decision.

```ntriples
# Completion record
<urn:hook:completion:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookCompletion> .
<urn:hook:completion:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#wasEndedBy>
    <urn:hook:execution:rdf-stop-hook:20260601T090000Z> .
<urn:hook:completion:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#endedAtTime>
    "2026-06-01T09:00:01.500Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<urn:hook:completion:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/exitCode>
    "0"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:completion:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/output>
    "{\"decision\":\"block\",\"reason\":\"Loop still active\",\"systemMessage\":\"🔄 RDF iteration 3/5\"}" .
<urn:hook:completion:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/decision>
    <urn:hook:decision:block> .

# Decision details
<urn:hook:decision:block>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookDecision> .
<urn:hook:decision:block>
  <http://claude.ai/hook/status>
    "block" .
<urn:hook:decision:block>
  <http://claude.ai/hook/isBlocking>
    "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:decision:block>
  <http://schema.org/reason>
    "Loop still active" .
<urn:hook:decision:block>
  <http://claude.ai/hook/nextPrompt>
    "Continue the RDF-native loop iteration..." .
```

### Stage 6: Persistence

Hook state and outcome are logged/persisted.

```ntriples
# State persistence record
<urn:hook:persistence:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/StatePersistence> .
<urn:hook:persistence:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/hook>
    <urn:hook:rdf-stop-hook> .
<urn:hook:persistence:rdf-stop-hook:20260601T090000Z>
  <http://claude.ai/hook/stateFile>
    "~/.claude/rdf-loop/state.ttl" .
<urn:hook:persistence:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/potentialAction>
    "UPDATE state with iteration count, output, completion check" .
<urn:hook:persistence:rdf-stop-hook:20260601T090000Z>
  <http://www.w3.org/ns/prov#wasAttributedTo>
    <urn:hook:handler:bash-rdf-stop> .
<urn:hook:persistence:rdf-stop-hook:20260601T090000Z>
  <http://schema.org/dateModified>
    "2026-06-01T09:00:01.600Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .

# RDF state file log entry
<urn:hook:statefile:rdf-loop-state:iteration-3>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <urn:hook:event:Stop> .
<urn:hook:statefile:rdf-loop-state:iteration-3>
  <http://claude.ai/loop/iterationNumber>
    "3"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:statefile:rdf-loop-state:iteration-3>
  <http://claude.ai/loop/maxIterations>
    "5"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:statefile:rdf-loop-state:iteration-3>
  <http://www.w3.org/ns/prov#value>
    "Last assistant output text..." .
<urn:hook:statefile:rdf-loop-state:iteration-3>
  <http://www.w3.org/ns/prov#startedAtTime>
    "2026-06-01T09:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
```

### Stage 7: Cancellation / Revocation

Hook is disabled, removed from registry, or state is cleared.

```ntriples
# Revocation record
<urn:hook:revocation:rdf-stop-hook:20260601T100000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookRevocation> .
<urn:hook:revocation:rdf-stop-hook:20260601T100000Z>
  <http://claude.ai/hook/hook>
    <urn:hook:rdf-stop-hook> .
<urn:hook:revocation:rdf-stop-hook:20260601T100000Z>
  <http://schema.org/action>
    "remove_from_registry" .
<urn:hook:revocation:rdf-stop-hook:20260601T100000Z>
  <http://www.w3.org/ns/prov#wasInformedBy>
    <urn:hook:event:stop-request-approved> .
<urn:hook:revocation:rdf-stop-hook:20260601T100000Z>
  <http://schema.org/reason>
    "Loop completed; state file cleared" .
<urn:hook:revocation:rdf-stop-hook:20260601T100000Z>
  <http://www.w3.org/ns/prov#atTime>
    "2026-06-01T10:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .

# State cleanup
<urn:hook:statefile:rdf-loop-state:cleared>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Action> .
<urn:hook:statefile:rdf-loop-state:cleared>
  <http://schema.org/target>
    "~/.claude/rdf-loop/state.ttl" .
<urn:hook:statefile:rdf-loop-state:cleared>
  <http://schema.org/result>
    "file deleted" .
```

---

## 3. Hook State Machine (Semantic Definition)

The lifecycle forms a directed state graph:

```
Declaration
    ↓
Registered
    ↓
Activated (on event match)
    ↓
Executing
    ├→ Timeout → TimedOut (not waiting for more completions)
    ├→ Completion (handler exits)
    │   ├→ Decision: "approve" → Completed+Persisted
    │   └→ Decision: "block" → Completed+Persisted+Blocking
    ↓
Persisted (state logged)
    ↓
Ready (for next activation)
    ↓
Revoked (disabled/removed)
    ↓
Inactive
```

### RDF State Transitions

```ntriples
# State transition: Declaration → Registered
<urn:hook:state-transition:decl-to-reg>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Action> .
<urn:hook:state-transition:decl-to-reg>
  <http://schema.org/fromState>
    <urn:hook:state:Declared> .
<urn:hook:state-transition:decl-to-reg>
  <http://schema.org/toState>
    <urn:hook:state:Registered> .
<urn:hook:state-transition:decl-to-reg>
  <http://schema.org/agent>
    "claude-code-runtime" .
<urn:hook:state-transition:decl-to-reg>
  <http://schema.org/trigger>
    "config loaded" .

# State transition: Registered → Activated
<urn:hook:state-transition:reg-to-active>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Action> .
<urn:hook:state-transition:reg-to-active>
  <http://schema.org/fromState>
    <urn:hook:state:Registered> .
<urn:hook:state-transition:reg-to-active>
  <http://schema.org/toState>
    <urn:hook:state:Activated> .
<urn:hook:state-transition:reg-to-active>
  <http://schema.org/trigger>
    <urn:hook:event:Stop> .

# Blocking state transition: Completed+Blocking → Ready (but session exit blocked)
<urn:hook:state-transition:blocking-ready>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Action> .
<urn:hook:state-transition:blocking-ready>
  <http://schema.org/fromState>
    <urn:hook:state:Completed> .
<urn:hook:state-transition:blocking-ready>
  <http://schema.org/toState>
    <urn:hook:state:Ready> .
<urn:hook:state-transition:blocking-ready>
  <http://claude.ai/hook/isBlocking>
    "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:state-transition:blocking-ready>
  <http://schema.org/potentialAction>
    "Session exit is prevented; next event will re-activate this hook" .
```

---

## 4. SPARQL Conformance Queries

### Query 1: List All Hooks in "Registered" State

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?event ?handler
WHERE {
  ?hook rdf:type hook:Hook .
  ?hook hook:state <urn:hook:state:Registered> .
  ?hook hook:event ?event .
  ?hook hook:handler ?handler .
}
ORDER BY ?hook
```

### Query 2: Find All Blocking Hooks

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?event ?isBlocking
WHERE {
  ?hook rdf:type hook:Hook .
  ?hook hook:isBlocking "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
  ?hook hook:event ?event .
}
```

### Query 3: Trace Execution Chain (Declaration → Registration → Activation → Execution → Completion)

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?decl_time ?reg_time ?exec_time ?comp_time ?decision
WHERE {
  ?hook rdf:type hook:Hook .
  ?decl prov:wasLoadedFrom ?hook ;
        prov:atTime ?decl_time .
  ?registration prov:wasRegisteredAt ?hook ;
                prov:atTime ?reg_time .
  ?activation prov:wasTriggeredBy ?hook ;
              prov:startedAtTime ?act_time .
  ?execution prov:wasStartedBy ?activation ;
             prov:startedAtTime ?exec_time .
  ?completion prov:wasEndedBy ?execution ;
              prov:endedAtTime ?comp_time ;
              hook:decision ?decision .
}
ORDER BY ?hook ?decl_time
```

### Query 4: Identify Hooks That Blocked Session Exit

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

ASK {
  ?completion rdf:type hook:HookCompletion .
  ?completion hook:decision ?decision .
  ?decision hook:status "block" .
  ?decision hook:isBlocking "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
}
```

### Query 5: Check RDF Loop State Persistence

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX loop: <http://claude.ai/loop/>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?iterationNumber ?maxIterations ?completionPattern ?lastOutput ?state
WHERE {
  <urn:loop:current> rdf:type prov:Activity .
  <urn:loop:current> loop:iterationNumber ?iterationNumber .
  <urn:loop:current> loop:maxIterations ?maxIterations .
  <urn:loop:current> loop:completionPattern ?completionPattern .
  <urn:loop:current> prov:value ?lastOutput .
  <urn:loop:current> <http://schema.org/actionStatus> ?state .
}
LIMIT 1
```

### Query 6: Find Hooks with Timeouts Exceeding Threshold

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?handler ?timeout
WHERE {
  ?hook rdf:type hook:Hook .
  ?hook hook:handler ?handler .
  ?handler hook:timeout ?timeout .
  FILTER (?timeout > 10)
}
ORDER BY DESC(?timeout)
```

### Query 7: Conformance: All Completed Hooks Have Decision Records

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

ASK {
  ?completion rdf:type hook:HookCompletion .
  ?completion hook:hook ?hook .
  FILTER NOT EXISTS {
    ?completion hook:decision ?decision .
  }
}
```
(Returns false if all completed hooks have decisions; true if any missing)

### Query 8: List All Active (Registered + Ready) Hooks

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?event ?state
WHERE {
  ?hook rdf:type hook:Hook .
  ?hook hook:state ?state .
  ?hook hook:event ?event .
  FILTER ( ?state IN (
    <urn:hook:state:Registered>,
    <urn:hook:state:Ready>
  ) )
}
ORDER BY ?hook
```

### Query 9: Identify Hooks with Revocation Records (Disabled/Removed)

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?revocation_reason ?revocation_time
WHERE {
  ?hook rdf:type hook:Hook .
  ?revocation rdf:type hook:HookRevocation .
  ?revocation hook:hook ?hook .
  ?revocation <http://schema.org/reason> ?revocation_reason .
  ?revocation <http://www.w3.org/ns/prov#atTime> ?revocation_time .
}
ORDER BY DESC(?revocation_time)
```

### Query 10: Temporal Ordering: Hook Execution Durations

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

SELECT ?hook ?start ?end ?duration_ms
WHERE {
  ?execution rdf:type prov:Activity .
  ?execution prov:startedAtTime ?start .
  ?execution prov:endedAtTime ?end .
  ?hook hook:handler ?handler .
  BIND (
    (xsd:dateTime(?end) - xsd:dateTime(?start)) * 1000 AS ?duration_ms
  )
}
ORDER BY DESC(?duration_ms)
```

---

## 5. Event Types and Semantic Definitions

### SessionStart Event

```ntriples
<urn:hook:event:SessionStart>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookEvent> .
<urn:hook:event:SessionStart>
  <http://www.w3.org/2000/01/rdf-schema#label>
    "SessionStart"@en .
<urn:hook:event:SessionStart>
  <http://www.w3.org/2000/01/rdf-schema#comment>
    "Fired once when Claude Code session is initialized"@en .
<urn:hook:event:SessionStart>
  <http://schema.org/trigger>
    "Session initialization" .
<urn:hook:event:SessionStart>
  <http://claude.ai/hook/isBlocking>
    "false"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:event:SessionStart>
  <http://claude.ai/hook/hooks>
    <urn:hook:explanatory-output-style:session-start> .
<urn:hook:event:SessionStart>
  <http://claude.ai/hook/hooks>
    <urn:hook:security-guidance:session-start> .
<urn:hook:event:SessionStart>
  <http://claude.ai/hook/cardinality>
    "1"@en .
```

### UserPromptSubmit Event

```ntriples
<urn:hook:event:UserPromptSubmit>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookEvent> .
<urn:hook:event:UserPromptSubmit>
  <http://www.w3.org/2000/01/rdf-schema#label>
    "UserPromptSubmit"@en .
<urn:hook:event:UserPromptSubmit>
  <http://www.w3.org/2000/01/rdf-schema#comment>
    "Fired each time the user submits a prompt/message"@en .
<urn:hook:event:UserPromptSubmit>
  <http://schema.org/trigger>
    "User submits prompt" .
<urn:hook:event:UserPromptSubmit>
  <http://claude.ai/hook/isBlocking>
    "false"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:event:UserPromptSubmit>
  <http://claude.ai/hook/hooks>
    <urn:hook:security-guidance:user-prompt> .
<urn:hook:event:UserPromptSubmit>
  <http://claude.ai/hook/cardinality>
    "many"@en .
```

### PreToolUse Event

```ntriples
<urn:hook:event:PreToolUse>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookEvent> .
<urn:hook:event:PreToolUse>
  <http://www.w3.org/2000/01/rdf-schema#label>
    "PreToolUse"@en .
<urn:hook:event:PreToolUse>
  <http://www.w3.org/2000/01/rdf-schema#comment>
    "Fired immediately before a tool is invoked"@en .
<urn:hook:event:PreToolUse>
  <http://schema.org/trigger>
    "Tool invocation pending" .
<urn:hook:event:PreToolUse>
  <http://claude.ai/hook/isBlocking>
    "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:event:PreToolUse>
  <http://claude.ai/hook/timeout>
    "10"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:event:PreToolUse>
  <http://claude.ai/hook/cardinality>
    "many"@en .
```

### PostToolUse Event

```ntriples
<urn:hook:event:PostToolUse>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookEvent> .
<urn:hook:event:PostToolUse>
  <http://www.w3.org/2000/01/rdf-schema#label>
    "PostToolUse"@en .
<urn:hook:event:PostToolUse>
  <http://www.w3.org/2000/01/rdf-schema#comment>
    "Fired immediately after a tool completes execution"@en .
<urn:hook:event:PostToolUse>
  <http://schema.org/trigger>
    "Tool execution completed" .
<urn:hook:event:PostToolUse>
  <http://claude.ai/hook/isBlocking>
    "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:event:PostToolUse>
  <http://claude.ai/hook/timeout>
    "10"^^<http://www.w3.org/2001/XMLSchema#integer> .
<urn:hook:event:PostToolUse>
  <http://claude.ai/hook/cardinality>
    "many"@en .
```

### Stop Event

```ntriples
<urn:hook:event:Stop>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookEvent> .
<urn:hook:event:Stop>
  <http://www.w3.org/2000/01/rdf-schema#label>
    "Stop"@en .
<urn:hook:event:Stop>
  <http://www.w3.org/2000/01/rdf-schema#comment>
    "Fired when user requests session exit (can be blocked)"@en .
<urn:hook:event:Stop>
  <http://schema.org/trigger>
    "User exit request" .
<urn:hook:event:Stop>
  <http://claude.ai/hook/isBlocking>
    "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
<urn:hook:event:Stop>
  <http://claude.ai/hook/cardinality>
    "1"@en .
<urn:hook:event:Stop>
  <http://claude.ai/hook/hooks>
    <urn:hook:rdf-stop-hook> .
<urn:hook:event:Stop>
  <http://claude.ai/hook/hooks>
    <urn:hook:ralph-loop:stop-hook> .
```

---

## 6. Hook Cancellation and Revocation Patterns

### Pattern A: Explicit Disable (settings.json modification)

```ntriples
<urn:hook:revocation:explicit-disable:20260601T110000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookRevocation> .
<urn:hook:revocation:explicit-disable:20260601T110000Z>
  <http://schema.org/action>
    "remove_from_settings_json" .
<urn:hook:revocation:explicit-disable:20260601T110000Z>
  <http://claude.ai/hook/hook>
    <urn:hook:rdf-stop-hook> .
<urn:hook:revocation:explicit-disable:20260601T110000Z>
  <http://schema.org/agent>
    "user" .
<urn:hook:revocation:explicit-disable:20260601T110000Z>
  <http://schema.org/method>
    "config_change" .
<urn:hook:revocation:explicit-disable:20260601T110000Z>
  <http://www.w3.org/ns/prov#atTime>
    "2026-06-01T11:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .

# Result state
<urn:hook:rdf-stop-hook>
  <http://claude.ai/hook/state>
    <urn:hook:state:Inactive> .
```

### Pattern B: Plugin Disable

```ntriples
<urn:hook:revocation:plugin-disable:20260601T120000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookRevocation> .
<urn:hook:revocation:plugin-disable:20260601T120000Z>
  <http://schema.org/action>
    "disable_plugin" .
<urn:hook:revocation:plugin-disable:20260601T120000Z>
  <http://claude.ai/hook/hook>
    <urn:hook:security-guidance:user-prompt> .
<urn:hook:revocation:plugin-disable:20260601T120000Z>
  <http://schema.org/targetPlugin>
    "security-guidance@claude-plugins-official" .
<urn:hook:revocation:plugin-disable:20260601T120000Z>
  <http://schema.org/agent>
    "user" .
<urn:hook:revocation:plugin-disable:20260601T120000Z>
  <http://www.w3.org/ns/prov#atTime>
    "2026-06-01T12:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
```

### Pattern C: State Cleanup (RDF Loop Completion)

```ntriples
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookRevocation> .
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://schema.org/action>
    "clear_state_file" .
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://claude.ai/hook/hook>
    <urn:hook:rdf-stop-hook> .
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://schema.org/target>
    "~/.claude/rdf-loop/state.ttl" .
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://schema.org/reason>
    "Loop completion pattern matched" .
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://schema.org/agent>
    "rdf-stop-hook handler" .
<urn:hook:revocation:state-cleanup:20260601T130000Z>
  <http://www.w3.org/ns/prov#atTime>
    "2026-06-01T13:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .

# Result: hook returns "approve" decision, session exit allowed
<urn:hook:decision:approve:loop-complete>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://claude.ai/hook/HookDecision> .
<urn:hook:decision:approve:loop-complete>
  <http://claude.ai/hook/status>
    "approve" .
<urn:hook:decision:approve:loop-complete>
  <http://schema.org/reason>
    "Loop completed; state cleared" .
```

---

## 7. Hook Persistence and Logging

### Persistence Model: Three Layers

#### Layer 1: Settings Declaration

```ntriples
<urn:config:settings-json>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Document> .
<urn:config:settings-json>
  <http://schema.org/url>
    "file:///Users/sac/.claude/settings.json" .
<urn:config:settings-json>
  <http://schema.org/contains>
    <urn:hook:rdf-stop-hook> .
<urn:config:settings-json>
  <http://schema.org/dateModified>
    "2026-06-01T08:00:00Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
```

#### Layer 2: Hook State File (RDF Turtle)

```ntriples
<urn:hook:statefile:rdf-loop>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Document> .
<urn:hook:statefile:rdf-loop>
  <http://schema.org/url>
    "file://~/.claude/rdf-loop/state.ttl" .
<urn:hook:statefile:rdf-loop>
  <http://schema.org/format>
    "text/turtle" .
<urn:hook:statefile:rdf-loop>
  <http://schema.org/encodingFormat>
    "UTF-8" .
<urn:hook:statefile:rdf-loop>
  <http://schema.org/dateModified>
    "2026-06-01T09:00:01Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<urn:hook:statefile:rdf-loop>
  <http://schema.org/about>
    <urn:loop:current> .
```

#### Layer 3: Execution/Completion Log (JSONL)

```ntriples
<urn:hook:log:execution-record>
  <http://www.w3.org/1999/02/22-rdf-syntax-ns#type>
    <http://schema.org/Document> .
<urn:hook:log:execution-record>
  <http://schema.org/format>
    "application/jsonl" .
<urn:hook:log:execution-record>
  <http://schema.org/contains>
    <urn:hook:execution:rdf-stop-hook:20260601T090000Z> .
<urn:hook:log:execution-record>
  <http://schema.org/contains>
    <urn:hook:completion:rdf-stop-hook:20260601T090000Z> .
<urn:hook:log:execution-record>
  <http://www.w3.org/ns/prov#wasGeneratedBy>
    <urn:hook:runtime:executor> .
```

---

## 8. Semantic Constraints and Invariants

### Invariant 1: Single Canonical Declaration

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

# No hook should have multiple active declarations
ASK {
  ?hook rdf:type hook:Hook .
  ?hook hook:source ?source1 .
  ?hook hook:source ?source2 .
  FILTER (?source1 != ?source2)
}
```
(Must return false)

### Invariant 2: No Gap Between Completion and Persistence

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

# All completions must have a corresponding persistence record
ASK {
  ?completion rdf:type hook:HookCompletion .
  ?completion prov:endedAtTime ?end_time .
  FILTER NOT EXISTS {
    ?persistence prov:wasInformedBy ?completion .
    ?persistence prov:atTime ?persist_time .
    FILTER (?persist_time > ?end_time)
  }
}
```
(Must return false)

### Invariant 3: Blocking Decision Enforcement

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

# If a Stop-event hook has decision "block", session exit must be prevented
ASK {
  ?hook rdf:type hook:Hook .
  ?hook hook:event <urn:hook:event:Stop> .
  ?completion hook:decision ?decision .
  ?decision hook:status "block" .
  FILTER NOT EXISTS {
    ?session rdf:type hook:SessionExitBlocked .
  }
}
```
(Must return false when blocking hook is active)

### Invariant 4: Revocation Followed by Inactivity

```sparql
PREFIX hook: <http://claude.ai/hook/>
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

# No hook should be active after revocation
ASK {
  ?hook rdf:type hook:Hook .
  ?revocation hook:hook ?hook .
  ?revocation prov:atTime ?rev_time .
  ?hook hook:state ?state .
  FILTER ( ?state IN (
    <urn:hook:state:Registered>,
    <urn:hook:state:Activated>,
    <urn:hook:state:Executing>,
    <urn:hook:state:Ready>
  ) && ?rev_time > NOW() - 1 SECOND )
}
```
(Must return false)

---

## 9. Key Observations: Claude Code Hook Lifecycle

### Facts About RDF Loop Hook (rdf-stop-hook)

1. **Declaration Source:** `/Users/sac/.claude/settings.json` (global configuration)
2. **Event Trigger:** `Stop` event (session exit requested)
3. **Blocking Nature:** Yes — can prevent session termination
4. **State Persistence:** RDF Turtle file (`~/.claude/rdf-loop/state.ttl`)
5. **Completion Pattern:** SPARQL ASK query matching against `loop:completionPattern`
6. **State Transitions:** Declared → Registered → Activated → Executing → Completed → Persisted → Ready (repeats until completion)
7. **Cancellation Trigger:** Loop completion pattern matched OR max iterations reached
8. **Handler Language:** Bash script using `unrdf` (RDF query/template tool)

### Facts About Hook Registry

- **Global Hooks:** 1 (rdf-stop-hook)
- **Plugin Hooks:** 8+ across multiple plugins
- **Event Types:** 5 (SessionStart, UserPromptSubmit, PreToolUse, PostToolUse, Stop)
- **Blocking Events:** PreToolUse, PostToolUse, Stop (can cancel/prevent operations)
- **Non-Blocking Events:** SessionStart, UserPromptSubmit (advisory only)

### Stateful Components

- **RDF Loop State File:** `~/.claude/rdf-loop/state.ttl` (PROV-O + loop: ontology)
- **Hook Configuration:** `/Users/sac/.claude/settings.json` (JSON)
- **Execution Logs:** Implicit in transcript (JSONL format)
- **Completion Tracking:** Via SPARQL ASK on state file

---

## 10. Summary: RDF Hook Lifecycle Graph

```
DECLARATION LAYER (JSON/Plugin manifests)
        ↓
[Hook, Event, Handler, Source metadata]
        ↓
REGISTRATION LAYER (Runtime hook registry)
        ↓
[Hook state = Registered; in-memory registry loaded]
        ↓
ACTIVATION LAYER (Event triggers)
        ↓
[Hook state = Activated; context captured]
        ↓
EXECUTION LAYER (Process spawned)
        ↓
[Hook state = Executing; PID tracked, timeout enforced]
        ↓
COMPLETION LAYER (Handler exits + decision returned)
        ↓
[Hook state = Completed; decision = approve|block]
        ↓
PERSISTENCE LAYER (State logged to RDF Turtle / JSONL)
        ↓
[Hook state = Persisted; state file updated, SPARQL ASK evaluated]
        ↓
READINESS LAYER (Awaits next event)
        ↓
[Hook state = Ready; re-registration for next activation]
        ↓
CANCELLATION LAYER (Revocation / cleanup)
        ↓
[Hook state = Inactive; state file deleted, registry entry removed]
```

Each transition is logged as semantic RDF triples using PROV-O + custom `hook:` vocabulary. Conformance is validated via SPARQL patterns.

---

## Appendix: Complete Hook Manifest (All Observed Hooks)

| Hook ID | Event | Source | Blocking | Handler | Timeout | State |
|---------|-------|--------|----------|---------|---------|-------|
| `rdf-stop-hook` | Stop | settings.json | ✅ | bash ~/.claude/rdf-loop/rdf-stop-hook.sh | 30s | Registered |
| `ralph-loop:stop-hook` | Stop | ralph-loop plugin | ✅ | plugin hook | ? | Registered |
| `explanatory-output-style:session-start` | SessionStart | plugin | ❌ | session-start.sh | ? | Registered |
| `security-guidance:session-start` | SessionStart | plugin | ❌ | sg-python.sh + ensure_agent_sdk.py | 180s | Registered |
| `security-guidance:user-prompt` | UserPromptSubmit | plugin | ❌ | security_reminder_hook.py | ? | Registered |
| `security-guidance:post-tool` | PostToolUse | plugin | ✅ | security_reminder_hook.py | ? | Registered |
| `hookify:pretooluse` | PreToolUse | plugin | ✅ | pretooluse.py | 10s | Unknown |
| `hookify:posttooluse` | PostToolUse | plugin | ✅ | posttooluse.py | 10s | Unknown |
| `hookify:stop` | Stop | plugin | ✅ | stop.py | ? | Unknown |

---

**End of RDF Hook Lifecycle Documentation**

Format: NTriples (.nt) format for all RDF triple declarations (compatible with SPARQL query engines)  
Export as: `/Users/sac/wasm4pm-compat/emitted/rdf-hook-lifecycle.md` (this file) + optional `.ttl` and `.nt` sidecar files for semantic triple stores.
