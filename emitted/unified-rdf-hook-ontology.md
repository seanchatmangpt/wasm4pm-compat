# Unified RDF Hook Ontology

**Version:** 1.0  
**Date:** 2026-06-01  
**Authority:** wasm4pm-compat + process-mining-chicago-tdd + manufacturing-law-commit-gate  
**Scope:** Hook class hierarchy, semantic activation, authority model, composition, loss/receipt semantics

---

## Overview

This document synthesizes the unified ontology of hooks across the manufacturing pipeline, hook authority model, and RDF activation semantics. A hook is not a code routine—it is a **named structural transformation** that fires when a precondition is met, carries a witness, reports loss, and advances the object through its lifecycle. The ontology is grounded in:

1. **Process mining** (van der Aalst Chicago TDD)
2. **Manufacturing law** (zero-cost type-law surfaces in nightly Rust)
3. **Witness semantics** (admission/refusal at type-law boundaries)
4. **Loss accounting** (OCEL → XES flattening requires explicit projection + policy + report)
5. **Receipt chains** (BLAKE3 provenance, proof gates, object lifecycle soundness)

---

## Part 1: Hook Class Hierarchy

### 1.1 Core Hook Taxonomy

```turtle
@prefix hook: <http://wasm4pm.org/hook/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix schema: <https://schema.org/> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

### Root: AbstractHook (uninhabited, zero-cost marker)
hook:AbstractHook
    a owl:Class ;
    rdfs:label "Abstract Hook" ;
    skos:definition "Root class for all hook markers. Like Rust empty enums, hooks are type-level only and carry no runtime state." ;
    skos:note "Hooks are not executable code; they are named structural transformations. Code that declares a hook handler is the implementation, not the hook itself." .

### Structural Hooks (type-system enforcement)
hook:StructuralHook
    a owl:Class ;
    rdfs:subClassOf hook:AbstractHook ;
    rdfs:label "Structural Hook" ;
    skos:definition "Enforces invariants at the type/AST level. Activation is compile-time or static analysis, not runtime." ;
    skos:example "Admission gate (Raw → Admitted), Loss policy requirement, Witness type match." .

### Semantic Hooks (event log / process mining)
hook:SemanticHook
    a owl:Class ;
    rdfs:subClassOf hook:AbstractHook ;
    rdfs:label "Semantic Hook" ;
    skos:definition "Fires when an event log (OCEL) satisfies a precondition query. Activation is SPARQL ASK or pm4py discovery." ;
    skos:example "Object lifecycle soundness (all objects reach final state), Timestamp monotonicity, Token replay fitness threshold." .

### Execution Hooks (object-centric process)
hook:ExecutableHook
    a owl:Class ;
    rdfs:subClassOf hook:AbstractHook ;
    rdfs:label "Executable Hook" ;
    skos:definition "Carries the type-law surface into a manufacturing stage. Can emit artifacts, advance object state, or block." ;
    skos:example "Admit (Raw → Admitted), Project (Admitted → Exportable with loss report), Graduate (Admitted → wasm4pm)." .

### Authority Hooks (who can define/modify)
hook:AuthorityHook
    a owl:Class ;
    rdfs:subClassOf hook:AbstractHook ;
    rdfs:label "Authority Hook" ;
    skos:definition "Guards which entity (paper, standard, Rust law, engine) has the right to define a hook." ;
    skos:example "Witness::(KEY, TITLE, YEAR, FAMILY), Admit impl, Refusal reason type." .

### Composition Hooks (how hooks combine)
hook:CompositionHook
    a owl:Class ;
    rdfs:subClassOf hook:AbstractHook ;
    rdfs:label "Composition Hook" ;
    skos:definition "Defines how two hooks chain or branch. E.g., Admit → into_evidence → Projected, or Admit fails → Refused." ;
    skos:example "Typestate chain (Raw → Parsed → Admitted → Projected | Exportable | Receipted), Loss policy choice (RefuseLoss vs AllowNamedProjection)." .

### Refusal Hooks (named law violation)
hook:RefusalHook
    a owl:Class ;
    rdfs:subClassOf hook:AbstractHook ;
    rdfs:label "Refusal Hook" ;
    skos:definition "Fires when a precondition fails. Must name a specific law, not a catch-all error." ;
    skos:example "MissingObjectRelation (OCEL law), FlatteningLoss (format covenant), DeadTransition (WF-net soundness), UnreplayableClaim (receipt law)." .

### Loss Hooks (projection & flattening)
hook:LossHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "Loss Hook" ;
    skos:definition "Fires during lossy transformation (e.g., OCEL → XES). Must emit LossReport itemizing what was discarded." ;
    skos:example "OcelFlattenToXesByCase (via projection name), OcelToXesObjectIntroduction (introduces pseudo-object), XesToOcelLinkRecovery (incomplete reconstruction)." .

### Receipt Hooks (proof chain)
hook:ReceiptHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "Receipt Hook" ;
    skos:definition "Emits a BLAKE3-chained proof that an object passed a proof gate. Ordered, immutable." ;
    skos:example "TokenReplayReceipt (fitness > threshold), ConformanceReceipt (model-log alignment), WfNetSoundnessReceipt (Murata bipartite holds)." .

### Permission Hooks (access control)
hook:PermissionHook
    a owl:Class ;
    rdfs:subClassOf hook:AuthorityHook ;
    rdfs:label "Permission Hook" ;
    skos:definition "Gates whether an entity can invoke a hook. Enforced via witness match + authority chain." ;
    skos:example "Only StandardBody can define Ocel20 witness, Only Paper author can define PowlPaper witness, Only Engine can emit GraduationCandidate." .

### Blocking Hooks (stop/defer)
hook:BlockingHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "Blocking Hook" ;
    skos:definition "Can prevent forward motion. E.g., Stop event blocks session exit, unresolved object blocks release." ;
    skos:example "RDF loop termination control (ralph-loop Stop hook), RDF iteration (rdf-stop-hook.sh decision block)." .
```

### 1.2 Hook Specializations

```turtle
### Session-level hooks
hook:SessionHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "Session Hook" ;
    skos:definition "Fires at session lifecycle boundaries (start, prompt submit, tool use, stop)." .

hook:SessionStartHook rdfs:subClassOf hook:SessionHook ;
    rdfs:label "Session Start Hook" ;
    skos:definition "Fires when session initializes. Non-blocking. Used for setup, state initialization." .

hook:SessionStopHook rdfs:subClassOf hook:SessionHook ;
    rdfs:label "Session Stop Hook" ;
    skos:definition "Fires when user requests exit. Can block (e.g., ralph-loop iteration not complete)." ;
    skos:example "rdf-stop-hook.sh (checks iteration count, extraction pattern, state.ttl)." .

### Tool-level hooks
hook:ToolHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "Tool Hook" ;
    skos:definition "Fires before/after tool invocation. Can inspect, modify, or block tool execution." .

hook:PreToolUseHook rdfs:subClassOf hook:ToolHook ;
    rdfs:label "Pre-Tool Use Hook" ;
    skos:definition "Fires before tool invocation. May validate inputs, check permissions, or block." .

hook:PostToolUseHook rdfs:subClassOf hook:ToolHook ;
    rdfs:label "Post-Tool Use Hook" ;
    skos:definition "Fires after tool completion. May validate outputs, emit receipts, or record side effects." .

### User interaction hooks
hook:UserHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "User Hook" ;
    skos:definition "Fires on user action (prompt submit, command invocation)." .

hook:UserPromptSubmitHook rdfs:subClassOf hook:UserHook ;
    rdfs:label "User Prompt Submit Hook" ;
    skos:definition "Fires when user submits a prompt. Can inject context, validate, or warn." .

### Type-law hooks (nightly foundry)
hook:TypeLawHook
    a owl:Class ;
    rdfs:subClassOf hook:StructuralHook ;
    rdfs:label "Type-Law Hook" ;
    skos:definition "Compile-time enforcement via Rust nightly features (generic_const_exprs, adt_const_params, const_trait_impl, min_specialization, portable_simd)." ;
    skos:example "Admission<T, W> requires impl Admit, Evidence<T, State, W> is sealed, Refusal reason must be named law." .

### Admission hooks (boundary)
hook:AdmissionHook
    a owl:Class ;
    rdfs:subClassOf hook:StructuralHook ;
    rdfs:label "Admission Hook" ;
    skos:definition "The ONLY sanctioned Raw → Admitted path. Fires when impl Admit::admit() is called. Must return Admission<T, W> or Refusal<R, W>." ;
    skos:example "OcelAdmit (checks E2O, O2O, object link validity), XesAdmit (checks case, event, attribute structure)." .

### Graduation hooks (wasm4pm bridge)
hook:GraduationHook
    a owl:Class ;
    rdfs:subClassOf hook:ExecutableHook ;
    rdfs:label "Graduation Hook" ;
    skos:definition "Advances Admitted evidence to wasm4pm engine. Unlocks real verification (token replay, soundness checking, discovery)." ;
    skos:example "GraduateToWasm4pm impl, GraduationCandidate (grounded vs ungrounded), GraduationReason (hard signal vs weak signal)." .
```

---

## Part 2: Core Hook Properties

### 2.1 Hook Property Schema

```turtle
### Hook Identity
hook:name
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range xsd:string ;
    skos:definition "Machine-readable hook identifier (e.g., 'admit-ocel-20', 'loss-ocel-to-xes-by-case')." ;
    skos:example "AdmitOcel20, ProjectOcelToXesByCase, ReceiptTokenReplay." .

hook:title
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range xsd:string ;
    skos:definition "Human-readable hook title." ;
    skos:example "\"Admit OCEL 2.0\", \"Project OCEL to XES by Case Type\"." .

### Hook Activation (Precondition + Trigger)
hook:trigger
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range [ rdf:type owl:Class ; owl:oneOf (
        hook:TriggerTypeSession
        hook:TriggerTypeEvent
        hook:TriggerTypeQuery
        hook:TriggerTypeState
    )] ;
    skos:definition "What causes this hook to fire." .

hook:TriggerTypeSession a owl:Class ; rdfs:label "Session Trigger" ;
    skos:definition "Hook fires on session event (start, stop, prompt submit)." .

hook:TriggerTypeEvent a owl:Class ; rdfs:label "Event Trigger" ;
    skos:definition "Hook fires on object-centric event (object reaches state, activity completes, timestamp changes)." .

hook:TriggerTypeQuery a owl:Class ; rdfs:label "Query Trigger" ;
    skos:definition "Hook fires when SPARQL ASK or pm4py discovery returns true." .

hook:TriggerTypeState a owl:Class ; rdfs:label "State Trigger" ;
    skos:definition "Hook fires when object reaches a typestate milestone (Raw→Parsed, Admitted→Projected, etc.)." .

hook:precondition
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range [ rdf:type owl:Class ; owl:oneOf (
        hook:PreconditionNone
        hook:PreconditionStructural
        hook:PreconditionSparql
        hook:PreconditionLogical
    )] ;
    skos:definition "Must-be-true before hook fires. None = always fire, Structural = type match, SPARQL = query, Logical = complex rule." .

hook:PreconditionNone a owl:Class ; rdfs:label "No Precondition" ;
    skos:definition "Hook fires unconditionally." .

hook:PreconditionStructural a owl:Class ; rdfs:label "Structural Precondition" ;
    skos:definition "Hook fires if object shape matches (e.g., Evidence<T, Raw, W> for admission)." .

hook:PreconditionSparql a owl:Class ; rdfs:label "SPARQL Precondition" ;
    skos:definition "Hook fires if SPARQL ASK query returns true on event log." .

hook:PreconditionLogical a owl:Class ; rdfs:label "Logical Precondition" ;
    skos:definition "Hook fires if complex rule evaluates to true (e.g., fitness > 0.8 AND precision > 0.7)." .

### Hook Effect (What it does)
hook:effect
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range [ rdf:type owl:Class ; owl:oneOf (
        hook:EffectAdmit
        hook:EffectRefuse
        hook:EffectProject
        hook:EffectReceipt
        hook:EffectBlock
        hook:EffectAdvance
        hook:EffectReport
    )] ;
    skos:definition "The transformation or judgment this hook performs." .

hook:EffectAdmit a owl:Class ; rdfs:label "Admit Effect" ;
    skos:definition "Admits Raw evidence to Admitted state (via Admit impl)." .

hook:EffectRefuse a owl:Class ; rdfs:label "Refuse Effect" ;
    skos:definition "Refuses evidence with named reason (via Refusal<R, W>)." .

hook:EffectProject a owl:Class ; rdfs:label "Project Effect" ;
    skos:definition "Transforms with explicit loss policy, emits LossReport." .

hook:EffectReceipt a owl:Class ; rdfs:label "Receipt Effect" ;
    skos:definition "Emits BLAKE3-chained proof gate receipt." .

hook:EffectBlock a owl:Class ; rdfs:label "Block Effect" ;
    skos:definition "Blocks forward motion (e.g., prevents session exit until condition met)." .

hook:EffectAdvance a owl:Class ; rdfs:label "Advance Effect" ;
    skos:definition "Advances object to next typestate (Raw→Parsed, Parsed→Admitted, Admitted→Projected)." .

hook:EffectReport a owl:Class ; rdfs:label "Report Effect" ;
    skos:definition "Emits diagnostic, loss report, or status update." .

### Hook Authority (Who can define/invoke)
hook:authority
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range hook:Authority ;
    skos:definition "Which entity has the right to define or invoke this hook." ;
    skos:comment "Authority is enforced via witness type, not runtime checks. A hook without a matching witness is not invoked." .

hook:Authority a owl:Class ;
    rdfs:label "Authority" ;
    skos:definition "An entity that can author, modify, or invoke hooks." .

hook:AuthorityStandard a owl:Class ; rdfs:subClassOf hook:Authority ;
    rdfs:label "Standards Body Authority" ;
    skos:definition "Authority is a published standard (OCEL 2.0, XES 1849-2016). Only the standards body can amend the hook." ;
    hook:example "Ocel20 witness, Xes1849 witness." .

hook:AuthorityPaper a owl:Class ; rdfs:subClassOf hook:Authority ;
    rdfs:label "Paper Authority" ;
    skos:definition "Authority is an academic paper defining a model or invariant. Authors maintain the definitive hook." ;
    hook:example "WfNetSoundnessPaper witness (van der Aalst 1998), PowlPaper witness (Kourani 2023)." .

hook:AuthorityRustLaw a owl:Class ; rdfs:subClassOf hook:Authority ;
    rdfs:label "Rust Law Authority" ;
    skos:definition "Authority is the Rust language / nightly compiler. Hook is enforced via type system." ;
    hook:example "generic_const_exprs for WF-net bipartite, forbid(unsafe_code) for memory safety." .

hook:AuthorityEngine a owl:Class ; rdfs:subClassOf hook:Authority ;
    rdfs:label "Engine Authority" ;
    skos:definition "Authority is the wasm4pm execution engine. Only the engine can emit graduation receipts." ;
    hook:example "GraduateToWasm4pm impl, TokenReplayReceipt, ConformanceReceipt." .

hook:AuthorityUser a owl:Class ; rdfs:subClassOf hook:Authority ;
    rdfs:label "User Authority" ;
    skos:definition "User-defined hook (via settings.json or .local.md). Authority is the session user." ;
    hook:example "PreToolUse hook from hookify plugin, custom SPARQL query in rdf-stop-hook.sh." .

### Hook Witness (Proof of authority)
hook:witness
    a rdf:Property ;
    rdfs:domain hook:AbstractHook ;
    rdfs:range hook:Witness ;
    skos:definition "The authority marker this hook carries. Typed as per witness.rs: Witness trait impl." ;
    skos:note "Witness is zero-cost (PhantomData tag). It is not runtime state; it is type-level proof." .

hook:Witness a owl:Class ;
    rdfs:label "Witness" ;
    skos:definition "Type-level marker naming the canon a hook answers to (paper, standard, API grammar, Rust law, engine bridge)." .

hook:witnessKey
    a rdf:Property ;
    rdfs:domain hook:Witness ;
    rdfs:range xsd:string ;
    skos:definition "Stable machine-readable witness key (e.g., 'ocel-2.0', 'wfnet-soundness-paper')." .

hook:witnessTitle
    a rdf:Property ;
    rdfs:domain hook:Witness ;
    rdfs:range xsd:string ;
    skos:definition "Human-readable witness title (e.g., 'OCEL 2.0', 'WF-net Soundness (van der Aalst 1998)')." .

hook:witnessYear
    a rdf:Property ;
    rdfs:domain hook:Witness ;
    rdfs:range xsd:gYear ;
    skos:definition "Publication year of authority, if applicable." .

hook:witnessFamily
    a rdf:Property ;
    rdfs:domain hook:Witness ;
    rdfs:range [ rdf:type owl:Class ; owl:oneOf (
        hook:WitnessFamilyStandard
        hook:WitnessFamilyPaper
        hook:WitnessFamilyApiGrammar
        hook:WitnessFamilyRustLaw
        hook:WitnessFamilyInternalBridge
    )] ;
    skos:definition "Category of authority (Standard, Paper, API Grammar, Rust Law, Internal Bridge)." .

hook:WitnessFamilyStandard a owl:Class ; rdfs:label "Standard" .
hook:WitnessFamilyPaper a owl:Class ; rdfs:label "Paper" .
hook:WitnessFamilyApiGrammar a owl:Class ; rdfs:label "API Grammar" .
hook:WitnessFamilyRustLaw a owl:Class ; rdfs:label "Rust Law" .
hook:WitnessFamilyInternalBridge a owl:Class ; rdfs:label "Internal Bridge" .

### Hook Refusal (Named law violation)
hook:refusalReason
    a rdf:Property ;
    rdfs:domain hook:RefusalHook ;
    rdfs:range hook:RefusalReason ;
    skos:definition "The specific named law that was violated when this hook fires as a refusal." ;
    skos:note "Refusal reason is NEVER a catch-all error string. It MUST name a specific structural law." .

hook:RefusalReason a owl:Class ;
    rdfs:label "Refusal Reason" ;
    skos:definition "Enumeration of named laws that can cause refusal." .

hook:RefusalReasonMissingObjectRelation a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Missing Object Relation" ;
    skos:definition "OCEL law: E2O or O2O relation missing or dangling. Cannot admit as flat XES without explicit projection." ;
    hook:paper "Ghahfarokhi 2021 (OCEL)" .

hook:RefusalReasonFlatteningLoss a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Flattening Loss" ;
    skos:definition "Format law: OCEL → XES loses object-centric structure. Requires LossPolicy + LossReport." ;
    hook:witness "Xes1849" .

hook:RefusalReasonDeadTransition a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Dead Transition" ;
    skos:definition "WF-net soundness law: transition has no path to sink. Violates Murata 1989 bipartite constraint." ;
    hook:paper "WfNetSoundnessPaper" .

hook:RefusalReasonUnsoundWfNet a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Unsound WF-net" ;
    skos:definition "WF-net soundness law: source and sink reachability violated. Violates van der Aalst 1998." ;
    hook:paper "WfNetSoundnessPaper" .

hook:RefusalReasonInvalidPowlProjection a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Invalid POWL Projection" ;
    skos:definition "POWL law: partial-order composition violates depth or arity constraints. Violates Kourani 2023 / 2505.07052." ;
    hook:paper "PowlPaper" .

hook:RefusalReasonMissingWitness a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Missing Witness" ;
    skos:definition "Governance law: evidence carries no witness type. Cannot judge which authority governs it." ;
    hook:example "Evidence<T, Admitted, PhantomData> instead of Evidence<T, Admitted, Ocel20>." .

hook:RefusalReasonUnreplayableClaim a owl:Class ;
    rdfs:subClassOf hook:RefusalReason ;
    rdfs:label "Unreplayable Claim" ;
    skos:definition "Receipt law: claimed receipt has no valid token replay history in event log." ;
    hook:source "Receipt chain BLAKE3 mismatch or log tampering." .

### Hook Loss (Projection semantics)
hook:lossPolicy
    a rdf:Property ;
    rdfs:domain hook:LossHook ;
    rdfs:range [ rdf:type owl:Class ; owl:oneOf (
        hook:LossPolicyRefuseLoss
        hook:LossPolicyAllowNamedProjection
        hook:LossPolicyAllowLossWithReport
    )] ;
    skos:definition "What this lossy hook does when transformation loses information." .

hook:LossPolicyRefuseLoss a owl:Class ;
    rdfs:label "Refuse Loss" ;
    skos:definition "If lossy transformation would occur, refuse instead. No LossReport emitted." .

hook:LossPolicyAllowNamedProjection a owl:Class ;
    rdfs:label "Allow Named Projection" ;
    skos:definition "Allow lossy transformation under a named ProjectionName. LossReport MUST be emitted." .

hook:LossPolicyAllowLossWithReport a owl:Class ;
    rdfs:label "Allow Loss With Report" ;
    skos:definition "Allow lossy transformation and MUST emit LossReport itemizing discarded evidence." .

hook:projectionName
    a rdf:Property ;
    rdfs:domain hook:LossHook ;
    rdfs:range xsd:string ;
    skos:definition "Stable, human-readable name for this projection (e.g., 'ocel-flatten-to-xes:by-case-type')." ;
    skos:note "ProjectionName is a zero-cost newtype wrapper. Multiple projections can produce the same target format but carry different names." .

hook:lossReport
    a rdf:Property ;
    rdfs:domain hook:LossHook ;
    rdfs:range hook:LossReport ;
    skos:definition "Itemization of what was discarded during lossy transformation. Required for audit trail." .

hook:LossReport a owl:Class ;
    rdfs:label "Loss Report" ;
    skos:definition "First-class artifact documenting information lost during projection." ;
    skos:example "OcelToXesFlattening: E2O links → case ID (lossy map), O2O links → dropped, object changes → attributes." .

hook:lossReportFrom
    a rdf:Property ;
    rdfs:domain hook:LossReport ;
    rdfs:range xsd:string ;
    skos:definition "Source format/shape (e.g., 'OcelShape')." .

hook:lossReportTo
    a rdf:Property ;
    rdfs:domain hook:LossReport ;
    rdfs:range xsd:string ;
    skos:definition "Target format/shape (e.g., 'XesShape')." .

hook:lossReportItems
    a rdf:Property ;
    rdfs:domain hook:LossReport ;
    rdfs:range rdf:List ;
    skos:definition "List of items (fields, relations, values) that were discarded." ;
    skos:example "[{field: 'O2O_relations', reason: 'no direct mapping in XES', count: 142}]" .

### Hook Receipt (Proof chain)
hook:receipt
    a rdf:Property ;
    rdfs:domain hook:ReceiptHook ;
    rdfs:range hook:Receipt ;
    skos:definition "The proof object emitted when a proof gate is passed." .

hook:Receipt a owl:Class ;
    rdfs:label "Receipt" ;
    skos:definition "Immutable, BLAKE3-chained proof that an object passed a proof gate." ;
    skos:note "Receipt is never a simple hash. It is a first-class artifact carrying witness, timestamp, measured value, and causal chain." .

hook:receiptChain
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range xsd:string ;
    skos:definition "BLAKE3 hash chain pointer to previous receipt. Establishes causal ordering and immutability." ;
    skos:example "blake3(previous_receipt_hash || current_witness || measured_value || timestamp)." .

hook:receiptWitness
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range hook:Witness ;
    skos:definition "Which authority this receipt answers to. Enables receipt migration across boundaries." .

hook:receiptProofGate
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range xsd:string ;
    skos:definition "Name of the proof gate this receipt represents (e.g., 'TokenReplayFitness', 'WfNetSoundness')." .

hook:receiptMeasuredValue
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range xsd:double ;
    skos:definition "The measured quality dimension (e.g., fitness score, precision score, soundness proof bit)." .

hook:receiptTimestamp
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range xsd:dateTime ;
    skos:definition "ISO 8601 timestamp when receipt was emitted. Part of proof chain." .

hook:receiptObjectId
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range xsd:string ;
    skos:definition "The object-centric ID (e.g., case_id, artifact_id) this receipt applies to." .

hook:receiptReplayPointer
    a rdf:Property ;
    rdfs:domain hook:Receipt ;
    rdfs:range xsd:string ;
    skos:definition "Pointer into event log (OCEL object-centric pointer) enabling receipt verification via replay." ;
    skos:example "ocel:trace[case_id=C123, obj_id=O456, event_index=42]." .
```

---

## Part 3: Hook Activation Grammar (RDF Query Patterns)

### 3.1 SPARQL Query Patterns for Hook Firing

Hook activation is encoded as RDF triples + SPARQL queries. A hook fires when a precondition query returns true.

#### Pattern 1: Typestate Transition (Structural)

```sparql
# Hook fires when Evidence reaches Admitted state
PREFIX evidence: <http://wasm4pm.org/evidence/>
PREFIX state: <http://wasm4pm.org/state/>

ASK WHERE {
    ?ev a evidence:Evidence ;
        evidence:value ?val ;
        evidence:state state:Admitted ;
        evidence:witness ?w .
}
```

**RDF Triple Representation:**
```turtle
hook:AdmitTransition
    a hook:StructuralHook ;
    hook:trigger hook:TriggerTypeState ;
    hook:precondition hook:PreconditionStructural ;
    hook:effect hook:EffectAdvance ;
    hook:queryPattern "PREFIX evidence: <http://wasm4pm.org/evidence/> ... ASK WHERE { ?ev evidence:state state:Admitted . }" .
```

#### Pattern 2: Object-Centric Event Log (Semantic)

```sparql
# Hook fires when object reaches final state in OCEL
PREFIX ocel: <http://wasm4pm.org/ocel/>
PREFIX schema: <https://schema.org/>

ASK WHERE {
    ?obj a ocel:Object ;
        ocel:id ?id ;
        ocel:state ?state .
    FILTER (?state = ocel:Final) .
}
```

**RDF Triple Representation:**
```turtle
hook:ObjectReachesTerminal
    a hook:SemanticHook ;
    hook:trigger hook:TriggerTypeEvent ;
    hook:precondition hook:PreconditionSparql ;
    hook:effect hook:EffectReceipt ;
    hook:queryPattern "PREFIX ocel: <http://wasm4pm.org/ocel/> ... ASK WHERE { ?obj ocel:state ocel:Final . }" ;
    hook:receipt hook:ObjectTerminalReceipt .
```

#### Pattern 3: Loss Accounting (Projection)

```sparql
# Hook fires when lossy projection is requested WITHOUT LossPolicy
PREFIX formats: <http://wasm4pm.org/formats/>
PREFIX loss: <http://wasm4pm.org/loss/>

ASK WHERE {
    ?proj a formats:ProjectionRequest ;
        formats:from ?from ;
        formats:to ?to ;
        formats:isLossy true .
    FILTER NOT EXISTS {
        ?proj loss:policy ?pol .
    }
}
```

**Effect:** Hook refusals with `FlatteningLoss` reason.

#### Pattern 4: Session Loop Control (Blocking)

```sparql
# Hook fires when session Stop event requested & loop is not complete
PREFIX loop: <http://claude.ai/loop/>
PREFIX prov: <http://www.w3.org/ns/prov#>

ASK WHERE {
    <urn:loop:current>
        a prov:Activity ;
        loop:iterationNumber ?iter ;
        loop:maxIterations ?max .
    FILTER (?iter < ?max) .
}
```

**RDF State File Example:**
```turtle
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix loop: <http://claude.ai/loop/> .
@prefix schema: <https://schema.org/> .

<urn:loop:current>
    a prov:Activity, schema:Action ;
    dcterms:identifier "session-uuid-12345" ;
    schema:actionStatus schema:ActiveActionStatus ;
    loop:iterationNumber 3 ;
    loop:maxIterations 5 ;
    loop:completionPattern "ALIVE_001_sealed" ;
    prov:value "last_assistant_output_text" ;
    prov:startedAtTime "2026-06-01T14:23:45Z" .
```

**Effect:** Hook blocks exit; returns `{"decision": "block", "reason": "RDF iteration 3/5"}`.

#### Pattern 5: Timestamp Monotonicity (Conformance)

```sparql
# Hook fires when event timestamps violate monotonicity in event log
PREFIX event: <http://wasm4pm.org/event/>

ASK WHERE {
    ?e1 a event:Event ;
        event:timestamp ?ts1 ;
        event:nextEvent ?e2 .
    ?e2 a event:Event ;
        event:timestamp ?ts2 .
    FILTER (?ts1 > ?ts2) .
}
```

**Effect:** Hook refusal with `TemporalConformanceViolation` reason.

#### Pattern 6: Witness Matching (Authority)

```sparql
# Hook fires when witness is present and matches expected authority
PREFIX hook: <http://wasm4pm.org/hook/>
PREFIX witness: <http://wasm4pm.org/witness/>

ASK WHERE {
    ?hook hook:witness witness:Ocel20 ;
        hook:authority ?auth .
    FILTER (?auth = witness:Ocel20Authority) .
}
```

**Effect:** Hook is authorized to execute.

### 3.2 SPARQL Patterns for Hook Composition

#### Chaining Pattern: Admit → Project → Receipt

```sparql
# Query to find hooks that must fire in sequence
PREFIX hook: <http://wasm4pm.org/hook/>
PREFIX ev: <http://wasm4pm.org/evidence/>

SELECT ?hook1 ?hook2 WHERE {
    ?hook1 hook:effect hook:EffectAdmit ;
        hook:outputState ?state1 .
    ?hook2 hook:effect hook:EffectProject ;
        hook:inputState ?state1 .
    ?hook2 hook:effect hook:EffectReceipt ;
        hook:outputState ?state2 .
}
ORDER BY ?hook1 ?hook2
```

**Result:**
```
| hook1         | hook2              |
|---------------|--------------------|
| AdmitOcel20   | ProjectOcelToXes   |
| AdmitXes1849  | ProjectXesToOcel   |
```

---

## Part 4: Hook Composition (Typestate Chain)

### 4.1 Typestate Lifecycle Diagram (as RDF)

```turtle
@prefix state: <http://wasm4pm.org/state/> .
@prefix hook: <http://wasm4pm.org/hook/> .
@prefix arrow: <http://wasm4pm.org/arrow/> .

### States (type-level tags)
state:Raw
    a owl:Class ;
    rdfs:label "Raw" ;
    skos:definition "Untrusted external evidence. No witness yet." .

state:Parsed
    a owl:Class ;
    rdfs:label "Parsed" ;
    skos:definition "Structure validated (XML well-formed, JSON valid). Still untrusted." .

state:Admitted
    a owl:Class ;
    rdfs:label "Admitted" ;
    skos:definition "Witness assigned and boundary laws checked. Trust established." .

state:Projected
    a owl:Class ;
    rdfs:label "Projected" ;
    skos:definition "Transformed via named projection with loss policy and report." .

state:Exportable
    a owl:Class ;
    rdfs:label "Exportable" ;
    skos:definition "Ready to emit to external format." .

state:Receipted
    a owl:Class ;
    rdfs:label "Receipted" ;
    skos:definition "Proof chain sealed. Object locked for release." .

state:Refused
    a owl:Class ;
    rdfs:label "Refused" ;
    skos:definition "Terminal state. Named law violation." .

### State Transitions (one-way doors)
state:Raw arrow:to state:Parsed ;
    arrow:method "parse" ;
    arrow:hook hook:ParseHook ;
    arrow:witness "none" ;
    arrow:refusal [ ] .

state:Parsed arrow:to state:Admitted ;
    arrow:method "admit" ;
    arrow:hook hook:AdmissionHook ;
    arrow:witness ?w ;
    arrow:refusal hook:Refused ;
    arrow:docstring "Only sanctioned Raw → Admitted path. Via Admit impl." .

state:Admitted arrow:to state:Projected ;
    arrow:method "project" ;
    arrow:hook hook:ProjectionHook ;
    arrow:requirement "LossPolicy must be set" ;
    arrow:emission "LossReport" .

state:Projected arrow:to state:Exportable ;
    arrow:method "into_exportable" ;
    arrow:hook hook:ExportableHook ;
    arrow:witness ?w ;
    arrow:emission "ready for format crossing" .

state:Exportable arrow:to state:Receipted ;
    arrow:method "emit_receipt" ;
    arrow:hook hook:ReceiptHook ;
    arrow:witness ?w ;
    arrow:emission "BLAKE3-chained Receipt" ;
    arrow:docstring "Proof gate passed, object locked." .

state:Admitted arrow:to state:Receipted ;
    arrow:method "direct_receipt" ;
    arrow:hook hook:DirectReceiptHook ;
    arrow:requirement "No projection needed" ;
    arrow:witness ?w .

### Terminal States (no further transitions)
state:Refused arrow:terminal true ;
    arrow:docstring "No conversion from Refused to any other state. Immutable." .

state:Receipted arrow:terminal true ;
    arrow:docstring "Object sealed. No further state changes except release." .
```

### 4.2 Hook Dependency Graph (Composition Order)

```turtle
### Admission Hook Dependencies
hook:AdmissionHook
    hook:requires [
        rdf:type rdf:List ;
        rdf:first hook:ParseHook ;
        rdf:rest (hook:WitnessMatchHook)
    ] ;
    hook:blocks [ ] ; # No hook depends on Admission firing
    hook:emits [ hook:Admission, hook:Refusal ] ;
    hook:blocks hook:ProjectionHook ; # Projection requires Admitted input
    hook:docstring "Must fire before any projection or export." .

### Projection Hook Dependencies
hook:ProjectionHook
    hook:requires [
        rdf:type rdf:List ;
        rdf:first hook:AdmissionHook ;
        rdf:rest ( hook:LossPolicyHook )
    ] ;
    hook:emits [ hook:LossReport ] ;
    hook:blocks hook:ExportableHook ;
    hook:docstring "Requires Admitted input and explicit LossPolicy." .

### Receipt Hook Dependencies
hook:ReceiptHook
    hook:requires [
        rdf:type rdf:List ;
        rdf:first hook:ProjectionHook ;
        rdf:rest (hook:ConformanceCheckHook hook:TokenReplayHook)
    ] ;
    hook:emits hook:Receipt ;
    hook:blocks hook:ReleaseHook ;
    hook:docstring "Requires proof gates to pass before receipt emitted." .

### Refusal Hook (terminal, blocks all forward motion)
hook:RefusedHook
    hook:inputState state:Refused ;
    hook:blocks hook:ProjectionHook ;
    hook:blocks hook:ReceiptHook ;
    hook:blocks hook:ExportableHook ;
    hook:blocks hook:ReleaseHook ;
    hook:terminal true ;
    hook:docstring "No forward motion from Refusal." .
```

### 4.3 Composition Pattern: OR (Refusal Alternative)

```turtle
### When Admission fails, Refusal must be handled
hook:AdmitOrRefuse
    a hook:CompositionHook ;
    hook:pattern "or" ;
    hook:leftBranch hook:AdmissionHook ;
    hook:rightBranch hook:RefusedHook ;
    hook:mergeState state:Refused ;
    hook:docstring "Result<Admission<T,W>, Refusal<R,W>>" .

### When Projection loses data, LossReport must be emitted
hook:ProjectOrReport
    a hook:CompositionHook ;
    hook:pattern "or-with-side-effect" ;
    hook:leftBranch hook:ProjectionHook ;
    hook:sideEffect hook:EmitLossReport ;
    hook:docstring "Lossy transformation without LossReport is refusal." .

### Graduate or remain (wasm4pm bridge)
hook:GraduateOrStay
    a hook:CompositionHook ;
    hook:pattern "conditional" ;
    hook:condition "caller requires execution authority" ;
    hook:trueBranch hook:GraduationHook ;
    hook:falseBranch hook:RemainAdmittedHook ;
    hook:docstring "Graduation is opt-in. Admitted evidence can stop here." .
```

---

## Part 5: Hook Authority Model

### 5.1 Authority Hierarchy

```turtle
@prefix auth: <http://wasm4pm.org/authority/> .
@prefix hook: <http://wasm4pm.org/hook/> .

### Root Authority (Rust Nightly Compiler)
auth:RustCompiler
    a hook:Authority ;
    auth:level 0 ;
    rdfs:label "Rust Compiler" ;
    skos:definition "The source of truth for type-law enforcement. No higher authority." ;
    hook:canDefine [ hook:TypeLawHook, hook:AdmissionHook ] ;
    hook:canModify [ ] ;
    hook:witness [ hook:RustLawWitness ] ;
    hook:docstring "If Rust rejects a hook, it is rejected. No overrides." .

### Level 1: Published Standards
auth:StandardsBody
    a hook:Authority ;
    auth:level 1 ;
    auth:reportTo auth:RustCompiler ;
    rdfs:label "Standards Body" ;
    skos:definition "Authority over published data formats (OCEL 2.0, XES 1849-2016)." ;
    hook:canDefine [ hook:AdmissionHook, hook:FormatCrossingHook ] ;
    hook:canModify [ hook:FormatCrossingHook ] ;
    hook:witness [ hook:StandardWitness ] ;
    hook:examples [ "OCEL 2.0", "IEEE XES 1849-2016", "BPMN 2.0" ] .

### Level 2: Academic Papers
auth:PaperAuthority
    a hook:Authority ;
    auth:level 2 ;
    auth:reportTo auth:RustCompiler ;
    rdfs:label "Paper Authority" ;
    skos:definition "Authority over theoretical models and algorithms." ;
    hook:canDefine [ hook:ConformanceHook, hook:SoundnessHook ] ;
    hook:canModify [ hook:ConformanceHook ] ;
    hook:witness [ hook:PaperWitness ] ;
    hook:examples [ "van der Aalst (WF-net soundness)", "Kourani (POWL)", "Ghahfarokhi (OCEL)" ] .

### Level 3: Engine (wasm4pm)
auth:Engine
    a hook:Authority ;
    auth:level 3 ;
    auth:reportTo [ auth:StandardsBody, auth:PaperAuthority ] ;
    rdfs:label "wasm4pm Engine" ;
    skos:definition "Authority over verification and execution. Consumes structure-only crate." ;
    hook:canDefine [ hook:GraduationHook, hook:TokenReplayHook, hook:DiscoveryHook ] ;
    hook:canModify [ hook:TokenReplayHook, hook:DiscoveryHook ] ;
    hook:witness [ hook:EngineWitness ] ;
    hook:docstring "Only engine can emit real verification receipts (token replay, soundness proof)." .

### Level 4: User (Session)
auth:SessionUser
    a hook:Authority ;
    auth:level 4 ;
    auth:reportTo [ auth:StandardsBody, auth:Engine ] ;
    rdfs:label "Session User" ;
    skos:definition "Authority over custom hooks in settings.json or .local.md." ;
    hook:canDefine [ hook:SessionHook, hook:ToolHook ] ;
    hook:canModify [ hook:SessionHook, hook:ToolHook ] ;
    hook:witness [ hook:UserWitness ] ;
    hook:restrictions [ "Cannot override Rust law", "Cannot define admission hooks", "Cannot emit verification receipts" ] ;
    hook:examples [ "PreToolUse hook", "PostToolUse hook", "Custom SPARQL query in rdf-stop-hook.sh" ] .

### Authority Lattice (Partial Order)
auth:lattice
    a owl:Class ;
    rdfs:label "Authority Lattice" ;
    skos:definition "Hierarchy of who can define/modify hooks. Higher level cannot be overridden by lower level." ;
    hook:order "RustCompiler > StandardsBody > PaperAuthority > Engine > SessionUser" ;
    hook:docstring "
        A hook defined at level 0 (Rust) cannot be modified by level 4 (User).
        A hook at level 3 (Engine) can call or build on hooks at level 1-2.
        A hook at level 4 (User) cannot define hooks that violate level 0-3 invariants.
    " .
```

### 5.2 Authority Enforcement via Witness Type

```turtle
### Each Witness carries Authority
hook:OcelWitness
    a hook:Witness ;
    hook:witness_key "ocel-2.0" ;
    hook:witness_title "OCEL 2.0" ;
    hook:witness_authority auth:StandardsBody ;
    hook:witness_year 2023 ;
    hook:witness_body "Ghahfarokhi, van der Aalst" ;
    hook:witness_standard "https://www.processmining.org/ocel20" ;
    hook:docstring "Only OCEL standards body can redefine admission for OCEL 2.0 evidence." .

hook:WfNetSoundnessWitness
    a hook:Witness ;
    hook:witness_key "wfnet-soundness-paper" ;
    hook:witness_title "WF-net Soundness (van der Aalst 1998)" ;
    hook:witness_authority auth:PaperAuthority ;
    hook:witness_year 1998 ;
    hook:witness_paper "van der Aalst: The Application of Petri Nets to Workflow Management" ;
    hook:docstring "Only paper authors can redefine WF-net soundness proof requirements." .

hook:RustLawWitness
    a hook:Witness ;
    hook:witness_key "rust-law" ;
    hook:witness_title "Rust Language" ;
    hook:witness_authority auth:RustCompiler ;
    hook:witness_year 2015 ;
    hook:witness_feature "generic_const_exprs" ;
    hook:witness_feature "adt_const_params" ;
    hook:witness_feature "const_trait_impl" ;
    hook:witness_feature "min_specialization" ;
    hook:witness_feature "portable_simd" ;
    hook:docstring "Rust nightly features are the court. No override." .

hook:EngineWitness
    a hook:Witness ;
    hook:witness_key "wasm4pm-engine" ;
    hook:witness_title "wasm4pm Execution Engine" ;
    hook:witness_authority auth:Engine ;
    hook:witness_feature "token_replay" ;
    hook:witness_feature "discovery" ;
    hook:witness_feature "conformance_checking" ;
    hook:docstring "Engine is the only source of real verification receipts." .

hook:UserWitness
    a hook:Witness ;
    hook:witness_key "session-user" ;
    hook:witness_title "Session User" ;
    hook:witness_authority auth:SessionUser ;
    hook:docstring "User-defined hooks are scoped to session and cannot override lower-level authorities." .
```

### 5.3 Permission Check (Hook Invocation Gate)

```sparql
# SPARQL query to check if a hook can be invoked
PREFIX hook: <http://wasm4pm.org/hook/> .
PREFIX auth: <http://wasm4pm.org/authority/> .
PREFIX witness: <http://wasm4pm.org/witness/> .

SELECT ?hook ?allowed WHERE {
    ?hook a hook:AbstractHook ;
        hook:witness ?w ;
        hook:authority ?auth .
    
    # Caller must match witness authority
    ?witness rdfs:subClassOf ?w .
    ?callerAuth auth:level ?callerLevel .
    ?auth auth:level ?requiredLevel .
    
    # Caller authority level must be >= required level
    BIND (IF (?callerLevel >= ?requiredLevel, "yes", "no") AS ?allowed) .
}
```

**Permission Enforcement in Code (Rust):**

```rust
// Only RustLaw witness can define type-law hooks
impl Hook for AdmissionHook<T, RustLawWitness> {
    fn invoke() -> Result<Admission<T, RustLawWitness>, Refusal<InvalidShape, RustLawWitness>> {
        // ... type-level checks ...
    }
}

// Only StandardsBody witness can define OCEL admission
impl Admit for OcelShape {
    type Witness = Ocel20;
    fn admit(raw: Raw) -> Result<Admission<Self, Ocel20>, Refusal<OcelRefusalReason, Ocel20>> {
        // ... OCEL 2.0 admission checks ...
    }
}

// Only Engine witness can emit verification receipts
impl Hook for TokenReplayReceipt {
    type Witness = EngineWitness;
    fn invoke() -> Receipt<TokenReplay, EngineWitness> {
        // ... token replay verification ...
    }
}
```

---

## Part 6: Integration Example — Ralph Loop + RDF Stop Hook

This section shows how hooks compose in a real system: the Ralph Loop self-referential control with RDF-native state management.

### 6.1 Hook Chain: Session Stop Event

```
User requests: exit / Ctrl+C
         ↓
Claude Code triggers "Stop" Event
         ↓
Hook 1: Global RDF Stop Hook (bash ~/.claude/rdf-loop/rdf-stop-hook.sh)
   ├─ Reads state.ttl (RDF Turtle format)
   ├─ SPARQL ASK: loop:iterationNumber < loop:maxIterations?
   ├─ If false → {decision: "approve"} → EXIT ALLOWED
   ├─ If true → continue to Hook 2
   └─ Reads transcript JSONL, extract last assistant output
       Update state.ttl with new iteration + output
       SPARQL ASK: CONTAINS(output, completionPattern)?
       If true → {decision: "approve"} → EXIT ALLOWED
       If false → {decision: "block", reason: "RDF iteration N/MAX"} → EXIT BLOCKED
         ↓
Hook 2: Plugin RDF Stop Hook (ralph-loop → bash stop-hook.sh)
   ├─ Additional loop state validation
   ├─ Merge decisions (if either blocks, exit is blocked)
   └─ Return combined decision
         ↓
Execution Result:
   - All hooks approved? → Exit session
   - Any hook blocked? → Block session, re-prompt user
```

### 6.2 RDF State File as Hook Carrier

**File:** `~/.claude/rdf-loop/state.ttl`

```turtle
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix schema: <https://schema.org/> .
@prefix loop: <http://claude.ai/loop/> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

<urn:loop:current>
    a prov:Activity, schema:Action ;
    dcterms:identifier "session-abc123def456" ;
    schema:actionStatus schema:ActiveActionStatus ;
    loop:iterationNumber 3 ;
    loop:maxIterations 5 ;
    loop:completionPattern "ALIVE_001_sealed" ;
    prov:value "last_assistant_output_text_from_transcript" ;
    prov:startedAtTime "2026-06-01T14:23:45Z"^^xsd:dateTime ;
    prov:generatedAtTime "2026-06-01T14:45:22Z"^^xsd:dateTime .

# Hook precondition queries
loop:CheckIterationLessThanMax
    a loop:SparqlQuery ;
    loop:query """
PREFIX loop: <http://claude.ai/loop/>
ASK WHERE {
    <urn:loop:current>
        loop:iterationNumber ?iter ;
        loop:maxIterations ?max .
    FILTER (?iter < ?max) .
}
    """ ;
    loop:result true .

loop:CheckCompletionPattern
    a loop:SparqlQuery ;
    loop:query """
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX loop: <http://claude.ai/loop/>
ASK WHERE {
    <urn:loop:current>
        prov:value ?val ;
        loop:completionPattern ?pat .
    FILTER (CONTAINS(?val, ?pat)) .
}
    """ ;
    loop:result false . # Pattern not yet matched
```

### 6.3 Hook Precondition in SPARQL

```sparql
# Query 1: Is iteration count reached?
PREFIX loop: <http://claude.ai/loop/>

ASK WHERE {
    <urn:loop:current>
        loop:iterationNumber ?iter ;
        loop:maxIterations ?max .
    FILTER (?iter >= ?max) .  # Iteration reached max → allow exit
}
# Result: false (iteration 3 < max 5) → Continue to Query 2

# Query 2: Is completion pattern matched?
PREFIX prov: <http://www.w3.org/ns/prov#>
PREFIX loop: <http://claude.ai/loop/>

ASK WHERE {
    <urn:loop:current>
        prov:value ?val ;
        loop:completionPattern ?pat .
    FILTER (CONTAINS(?val, ?pat)) .
}
# Result: false (output doesn't contain "ALIVE_001_sealed") → Block exit

# Final decision: {"decision": "block", "reason": "RDF iteration 3/5"}
```

### 6.4 Hook Handler (Bash Script)

**File:** `~/.claude/rdf-loop/rdf-stop-hook.sh`

```bash
#!/bin/bash
set -euo pipefail

STATE_FILE="${HOME}/.claude/rdf-loop/state.ttl"
TRANSCRIPT_PATH="${CLAUDE_TRANSCRIPT_PATH}"
MAX_ITERATIONS="${MAX_ITERATIONS:-10}"

# Step 1: Check iteration count
ITERATION=$(rdfquery "$STATE_FILE" 'SELECT ?iter WHERE { <urn:loop:current> loop:iterationNumber ?iter . }' | cut -d' ' -f1)
if [[ "$ITERATION" -ge "$MAX_ITERATIONS" ]]; then
    echo '{"decision":"approve"}'
    exit 0
fi

# Step 2: Extract last output from transcript (JSONL)
LAST_OUTPUT=$(tail -1 "$TRANSCRIPT_PATH" | jq -r '.assistant_message // empty')

# Step 3: Update state.ttl
NEW_ITERATION=$((ITERATION + 1))
TIMESTAMP=$(date -u +'%Y-%m-%dT%H:%M:%SZ')
cat > "$STATE_FILE" <<EOF
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix loop: <http://claude.ai/loop/> .
@prefix dcterms: <http://purl.org/dc/terms/> .

<urn:loop:current>
    loop:iterationNumber $NEW_ITERATION ;
    loop:maxIterations $MAX_ITERATIONS ;
    prov:value "$LAST_OUTPUT" ;
    prov:generatedAtTime "$TIMESTAMP"^^xsd:dateTime .
EOF

# Step 4: Check completion pattern
PATTERN=$(rdfquery "$STATE_FILE" 'SELECT ?p WHERE { <urn:loop:current> loop:completionPattern ?p . }' | cut -d' ' -f1)
if [[ "$LAST_OUTPUT" == *"$PATTERN"* ]]; then
    echo '{"decision":"approve"}'
    exit 0
fi

# Step 5: Block exit, return status
echo "{\"decision\":\"block\",\"reason\":\"RDF iteration $ITERATION/$MAX_ITERATIONS\",\"systemMessage\":\"🔄 RDF iteration $ITERATION/$MAX_ITERATIONS\"}"
exit 0
```

### 6.5 Hook Composition (Multiple Hooks)

```turtle
### Global + Plugin hooks for Stop event
hook:StopEventHooks
    a hook:CompositionHook ;
    hook:event "Stop" ;
    hook:hooks [
        rdf:type rdf:List ;
        rdf:first hook:GlobalRdfStopHook ;
        rdf:rest ( hook:PluginRalphLoopStopHook hook:PluginHookifyStopHook )
    ] ;
    hook:mergeStrategy "any_block_blocks_all" ;
    hook:docstring "
        All Stop hooks fire in sequence.
        If ANY hook returns {decision: 'block'}, exit is blocked.
        If ALL approve, exit is allowed.
    " .

hook:GlobalRdfStopHook
    a hook:BlockingHook ;
    hook:event "Stop" ;
    hook:command "bash ~/.claude/rdf-loop/rdf-stop-hook.sh" ;
    hook:precondition hook:PreconditionSparql ;
    hook:queryFile "~/.claude/rdf-loop/state.ttl" ;
    hook:effect hook:EffectBlock .

hook:PluginRalphLoopStopHook
    a hook:BlockingHook ;
    hook:event "Stop" ;
    hook:plugin "ralph-loop@claude-plugins-official" ;
    hook:command "bash \"${CLAUDE_PLUGIN_ROOT}/hooks/stop-hook.sh\"" ;
    hook:effect hook:EffectBlock .

hook:PluginHookifyStopHook
    a hook:BlockingHook ;
    hook:event "Stop" ;
    hook:plugin "hookify@claude-plugins-official" ;
    hook:command "python3 \"${CLAUDE_PLUGIN_ROOT}/hooks/stop.py\"" ;
    hook:effect hook:EffectBlock .
```

---

## Part 7: Complete Knowledge Base Summary

### 7.1 All Hook Classes (Complete List)

```turtle
### Hook Classes (Root to Leaf)

hook:AbstractHook
├── hook:StructuralHook
│   ├── hook:TypeLawHook
│   ├── hook:AdmissionHook
│   └── hook:WitnessMatchHook
├── hook:SemanticHook
│   ├── hook:ObjectLifecycleHook
│   ├── hook:TimestampMonotonicityHook
│   ├── hook:ConformanceCheckHook
│   └── hook:TokenReplayHook
├── hook:ExecutableHook
│   ├── hook:LossHook
│   │   └── hook:ProjectionHook
│   ├── hook:ReceiptHook
│   │   ├── hook:TokenReplayReceipt
│   │   ├── hook:WfNetSoundnessReceipt
│   │   ├── hook:ConformanceReceipt
│   │   └── hook:ObjectTerminalReceipt
│   ├── hook:GraduationHook
│   ├── hook:BlockingHook
│   │   ├── hook:SessionStopHook
│   │   └── hook:IterationLimitHook
│   └── hook:ToolHook
│       ├── hook:PreToolUseHook
│       └── hook:PostToolUseHook
├── hook:AuthorityHook
│   ├── hook:StandardWitness
│   ├── hook:PaperWitness
│   ├── hook:RustLawWitness
│   ├── hook:EngineWitness
│   └── hook:UserWitness
├── hook:CompositionHook
│   ├── hook:ChainHook
│   ├── hook:OrHook
│   ├── hook:ConditionalHook
│   └── hook:ParallelHook
└── hook:RefusalHook
    ├── hook:MissingObjectRelationRefusal
    ├── hook:FlatteningLossRefusal
    ├── hook:DeadTransitionRefusal
    ├── hook:UnsoundWfNetRefusal
    ├── hook:InvalidPowlProjectionRefusal
    ├── hook:MissingWitnessRefusal
    ├── hook:UnreplayableClaimRefusal
    └── hook:TemporalConformanceViolationRefusal
```

### 7.2 All Hook Properties (Complete Reference)

```turtle
### Identity & Metadata
hook:name
hook:title
hook:description
hook:docstring

### Activation
hook:trigger (TriggerTypeSession | TriggerTypeEvent | TriggerTypeQuery | TriggerTypeState)
hook:precondition (PreconditionNone | PreconditionStructural | PreconditionSparql | PreconditionLogical)
hook:queryPattern (SPARQL ASK string)

### Effect
hook:effect (EffectAdmit | EffectRefuse | EffectProject | EffectReceipt | EffectBlock | EffectAdvance | EffectReport)
hook:inputState (e.g., state:Raw, state:Admitted)
hook:outputState (e.g., state:Admitted, state:Projected)

### Authority & Governance
hook:authority (StandardsBody | PaperAuthority | RustCompiler | Engine | SessionUser)
hook:witness (Witness type marker)
hook:refusalReason (RefusalReason enum)

### Loss Accounting
hook:lossPolicy (RefuseLoss | AllowNamedProjection | AllowLossWithReport)
hook:projectionName (ProjectionName string)
hook:lossReport (LossReport object)
hook:lossReportFrom (source shape)
hook:lossReportTo (target shape)
hook:lossReportItems (discarded items list)

### Receipt
hook:receipt (Receipt object)
hook:receiptChain (BLAKE3 hash)
hook:receiptWitness (Witness type)
hook:receiptProofGate (proof gate name)
hook:receiptMeasuredValue (quality dimension value)
hook:receiptTimestamp (ISO 8601)
hook:receiptObjectId (object ID)
hook:receiptReplayPointer (OCEL pointer)

### Composition
hook:requires (list of prerequisite hooks)
hook:blocks (list of blocked hooks)
hook:emits (artifacts emitted)
hook:mergeStrategy (how multiple hooks compose)
hook:pattern (composition pattern: chain | or | conditional | parallel)

### Execution
hook:command (bash script path or Python command)
hook:timeout (timeout in seconds)
hook:async (true | false)
hook:plugin (plugin identifier)
hook:event (SessionStart | SessionStop | UserPromptSubmit | PreToolUse | PostToolUse | Stop)
```

### 7.3 All Witness Markers (Complete Registry)

```turtle
### Standards (WitnessFamily: Standard)
hook:Ocel20 (OCEL 2.0, 2023)
hook:Xes1849 (IEEE XES 1849-2016, 2016)
hook:Bpmn2 (BPMN 2.0, 2011)
hook:Pnml (PNML, 2004)

### Papers (WitnessFamily: Paper)
hook:WfNetSoundnessPaper (van der Aalst 1998)
hook:PowlPaper (Kourani & van Zelst 2023)
hook:ObjectCentricPetriNetPaper (van der Aalst & Berti 2020)
hook:OcpqPaper (Küsters & vdA 2024)
hook:DeclareFamily (declarative 2007)
hook:InductiveMinerPaper (Leemans 2013)

### API Grammars (WitnessFamily: ApiGrammar)
hook:Pm4pyApiGrammar (pm4py call shape)
hook:PmaxConsumerGrammar (pmax consumer grammar)

### Rust Laws (WitnessFamily: RustLaw)
hook:RustLawWitness (generic_const_exprs, adt_const_params, const_trait_impl, min_specialization, portable_simd)
hook:ForbidUnsafeCodeLaw (forbid(unsafe_code))
hook:TypestateAdmissionLaw (Evidence<T, State, W> is sealed)

### Engine (WitnessFamily: InternalBridge)
hook:EngineWitness (wasm4pm execution authority)
hook:TokenReplayWitness (token replay verification)
hook:DiscoveryWitness (process discovery)
```

### 7.4 All Refusal Reasons (Complete Registry)

```turtle
hook:MissingObjectRelation
    rdfs:label "Missing Object Relation (E2O or O2O)" ;
    hook:standard "Ocel20" ;
    hook:law "OCEL 2.0 structural invariant" .

hook:FlatteningLoss
    rdfs:label "Flattening Loss (OCEL → XES)" ;
    hook:standard "Xes1849" ;
    hook:law "Format covenant: lossy transformation requires LossPolicy + LossReport" .

hook:DeadTransition
    rdfs:label "Dead Transition (Petri net)" ;
    hook:paper "WfNetSoundnessPaper" ;
    hook:law "Murata 1989 bipartite constraint: no path to sink" .

hook:UnsoundWfNet
    rdfs:label "Unsound WF-net" ;
    hook:paper "WfNetSoundnessPaper" ;
    hook:law "van der Aalst 1998: source/sink reachability violated" .

hook:InvalidPowlProjection
    rdfs:label "Invalid POWL Projection" ;
    hook:paper "PowlPaper" ;
    hook:law "Kourani 2023: partial-order depth or arity constraint violated" .

hook:MissingWitness
    rdfs:label "Missing Witness" ;
    hook:law "Governance: evidence must carry witness type" .

hook:UnreplayableClaim
    rdfs:label "Unreplayable Claim (Receipt)" ;
    hook:law "Receipt law: claimed proof has no valid replay history in log" .

hook:TemporalConformanceViolation
    rdfs:label "Temporal Conformance Violation" ;
    hook:law "Event timestamps must be monotonic within traces" .

hook:MissingFinalMarking
    rdfs:label "Missing Final Marking (Petri net)" ;
    hook:paper "WfNetSoundnessPaper" ;
    hook:law "Workflow net soundness: no path from current marking to final" .

hook:InvalidAdmissionBoundary
    rdfs:label "Invalid Admission Boundary" ;
    hook:law "Admission hook must be called from Raw state" .

hook:LossPolicyMissing
    rdfs:label "Loss Policy Missing (Projection)" ;
    hook:law "Format covenant: lossy transformation requires explicit LossPolicy" .

hook:ReportMissing
    rdfs:label "Loss Report Missing (Projection)" ;
    hook:law "Format covenant: lossy transformation must emit LossReport itemizing discards" .
```

---

## Conclusion: The Unified Ontology

This RDF hook ontology synthesizes:

1. **Hook class hierarchy** — from abstract markers (zero-cost, type-level) to concrete effects (admission, projection, receipt, graduation)
2. **Core hook properties** — trigger, precondition, effect, authority, witness, refusal, loss, receipt
3. **Hook activation grammar** — SPARQL ASK queries that fire hooks, RDF state files, precondition patterns
4. **Hook composition** — typestate chain (Raw → Parsed → Admitted → Projected → Exportable → Receipted), OR logic (Admit | Refuse), blocking (Stop hooks)
5. **Hook authority model** — hierarchy (Rust > Standards > Papers > Engine > User), witness-based enforcement, permission lattice
6. **Real-world example** — Ralph Loop + RDF Stop Hook showing how multiple hooks compose with blocking semantics

A hook is not code. A hook is a **named transformation** that:
- Carries a witness (authority label)
- Fires when a precondition is met (SPARQL ASK on event log, typestate match, or session event)
- Performs an effect (admit, refuse, project, emit receipt, block, advance)
- May emit loss reports (itemizing discards)
- May emit receipts (BLAKE3-chained proofs)
- Respects authority hierarchy (RustCompiler > StandardsBody > PaperAuthority > Engine > SessionUser)

The ontology is immutable by design: Rust nightly is the court, standards bodies maintain their witnesses, papers are eternal authorities, the engine is the only source of real verification receipts, and users can hook into the system without violating higher-level laws.

---

**Generated:** 2026-06-01  
**Authority:** wasm4pm-compat codebase + process-mining-chicago-tdd + manufacturing-terminology  
**Format:** Turtle (RDF) + Markdown documentation
