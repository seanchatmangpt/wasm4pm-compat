# Spec-Kit RDF Ontology Hook Specifications

**Scan Date:** 2026-06-01
**Directory:** ~/ggen-spec-kit/
**Report Version:** 1.0

---

## Executive Summary

This document catalogs the **ontology hook specifications** discovered in the ggen-spec-kit repository. The spec-kit framework uses RDF (Resource Description Framework) with SHACL constraints to define domain ontologies. RDF "hooks" in this system are:

1. **Shape Constraints** (SHACL) — validation rules that prevent invalid state
2. **Ontology Classes & Properties** — structural definitions extensible by instantiation
3. **Extension Ontologies** — domain-specific vocabularies that layer onto core spec-kit
4. **Instantiation Patterns** — concrete example data demonstrating hook usage

The framework follows a **"constitutional equation"** pattern:
```
output.md = μ(ontology.ttl)
```
Where `.ttl` files serve as the single source of truth, and transformations (via ggen, SPARQL, Tera templates) manufacture structured output.

---

## 1. Spec Files Defining Hook Schemas

### 1.1 Core Schema Files

| File | Purpose | Classes | Hook Type |
|------|---------|---------|-----------|
| `ontology/spec-kit-schema.ttl` | Base feature specification ontology | Feature, UserStory, FunctionalRequirement, SuccessCriterion, Task, Dependency, Assumption | **Core Ontology** |
| `ontology/cli-schema.ttl` | CLI command specification framework | Command, Argument, Option, ValidationRule, CommandGroup | **Extension Ontology** |
| `ontology/jtbd-schema.ttl` | Jobs-to-be-Done framework | Job, Outcome, Persona, CustomerSegment, Painpoint, ProgressMaker, JobCompletion, Metric | **Domain Extension** |

### 1.2 Extension Ontologies

| File | Extends | Purpose | New Classes |
|------|---------|---------|-------------|
| `ontology/spec-kit-docs-extension.ttl` | spec-kit-schema.ttl | Documentation as RDF | Documentation, Guide, APIReference, Principle, Governance, WorkflowPhase |
| `ontology/spec-kit-jtbd-extension.ttl` | spec-kit-schema.ttl + jtbd-schema.ttl | Cross-domain JTBD integration | (Reuses core JTBD classes with SK properties) |

### 1.3 Constitutional Equations (Transformation Pipelines)

Each ontology defines a transformation covenant:

```turtle
# CLI Commands Example
# Constitutional equation: commands/*.py = μ(cli-commands.ttl)
# Design: Each CLI command is an owl:Class that ggen extracts.
# Format: ggen reads cli-commands.ttl via SPARQL,
#         renders via cli-command-test.tera,
#         outputs commands/**.py

# Feature Specifications Example
# Constitutional equation: feature-*.md = μ(feature-spec.ttl)
# Design: RDF graph → SPARQL queries → Tera Markdown templates
```

---

## 2. RDF Shape Constraints (SHACL)

### 2.1 CLI Command Shapes

**File:** `ontology/cli-command-shapes.ttl` (425 lines)

Defines validation shapes for command specifications:

#### CLICommandShape
```turtle
sk:CLICommandShape a sh:NodeShape ;
    sh:targetClass sk:CLICommand ;
    sh:property [
        sh:path sk:commandName ;
        sh:pattern "^[a-z][a-z0-9-]*$" ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
        sh:message "Command name must be lowercase with hyphens, starting with a letter" ;
    ] ;
    sh:property [
        sh:path sk:commandDescription ;
        sh:minLength 10 ;
        sh:minCount 1 ;
        sh:message "Command description required (min 10 characters)" ;
    ] ;
    sh:property [
        sh:path sk:hasArgument ;
        sh:class sk:CommandArgument ;
    ] ;
    sh:property [
        sh:path sk:hasOption ;
        sh:class sk:CommandOption ;
    ] ;
```

**Validation Rules:**
- Command names must match `^[a-z][a-z0-9-]*$` (lowercase alphanumeric + hyphens)
- Description required, minimum 10 characters
- Must declare at least one argument or option
- Error cases reference specific exception types

#### CommandArgumentShape
```turtle
sk:CommandArgumentShape a sh:NodeShape ;
    sh:targetClass sk:CommandArgument ;
    sh:property [
        sh:path sk:argumentName ;
        sh:pattern "^[a-z][a-z0-9_]*$" ;
    ] ;
    sh:property [
        sh:path sk:argumentType ;
        sh:in ("string" "int" "float" "path" "boolean") ;
    ] ;
    sh:property [
        sh:path sk:argumentPosition ;
        sh:datatype xsd:integer ;
        sh:minInclusive 0 ;
    ] ;
```

**Constraints:**
- Argument names: `^[a-z][a-z0-9_]*$`
- Type enum: string | int | float | path | boolean
- Position: non-negative integer
- Required field: true | false

#### CommandOptionShape
```turtle
sk:CommandOptionShape a sh:NodeShape ;
    sh:targetClass sk:CommandOption ;
    sh:property [
        sh:path sk:optionName ;
        sh:pattern "^--[a-z][a-z0-9-]*$" ;
    ] ;
    sh:property [
        sh:path sk:optionShortFlag ;
        sh:pattern "^-[a-zA-Z]$" ;
    ] ;
    sh:property [
        sh:path sk:optionType ;
        sh:in ("flag" "string" "int" "float" "path" "boolean") ;
    ] ;
```

**Constraints:**
- Option names: `^--[a-z][a-z0-9-]*$`
- Short flags: `^-[a-zA-Z]$`
- Type: flag | string | int | float | path | boolean
- Default value optional but typed

#### CommandErrorCaseShape
```turtle
sk:CommandErrorCaseShape a sh:NodeShape ;
    sh:targetClass sk:CommandErrorCase ;
    sh:property [
        sh:path sk:errorId ;
        sh:pattern "^[a-z][a-z0-9-]*$" ;
    ] ;
    sh:property [
        sh:path sk:errorExpectedExitCode ;
        sh:minInclusive 0 ;
        sh:maxInclusive 255 ;
    ] ;
    sh:property [
        sh:path sk:errorException ;
        sh:datatype xsd:string ;
    ] ;
```

**Constraints:**
- Error ID: `^[a-z][a-z0-9-]*$`
- Exit codes: 0–255
- Exception class name required (no strings, specific types only)

---

### 2.2 JTBD Measurement Shapes

**File:** `ontology/jtbd-shapes.ttl` (839 lines)

Advanced validation for Jobs-to-be-Done framework:

#### JobCompletionShape
```turtle
jtbd:JobCompletionShape a sh:NodeShape ;
    sh:targetClass jtbd:JobCompletion ;
    sh:property [
        sh:path jtbd:completedAt ;
        sh:datatype xsd:dateTime ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:duration ;
        sh:datatype xsd:decimal ;
        sh:minExclusive 0.0 ;
    ] ;
    sh:property [
        sh:path jtbd:wasSuccessful ;
        sh:datatype xsd:boolean ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:completesJob ;
        sh:class jtbd:Job ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:performedBy ;
        sh:class jtbd:CustomerSegment ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
    ] ;
```

**Constraints:**
- Completion timestamp required (ISO 8601)
- Duration: positive decimal (seconds)
- Success status required (boolean)
- Job reference required (exactly one)
- Customer segment required (exactly one)

#### OutcomeAchievementShape
```turtle
jtbd:OutcomeAchievementShape a sh:NodeShape ;
    sh:targetClass jtbd:OutcomeAchievement ;
    sh:property [
        sh:path jtbd:measuredAt ;
        sh:datatype xsd:dateTime ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:metricValue ;
        sh:datatype xsd:decimal ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:metTargetThreshold ;
        sh:datatype xsd:boolean ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:confidenceScore ;
        sh:datatype xsd:decimal ;
        sh:minInclusive 0.0 ;
        sh:maxInclusive 1.0 ;
    ] ;
    sh:property [
        sh:path jtbd:aggregationLevel ;
        sh:in ( "Individual" "Daily" "Weekly" "Monthly" "Quarterly" "Yearly" ) ;
    ] ;
```

**Constraints:**
- Measurement timestamp required
- Metric value required (decimal, no unit constraint)
- Target threshold comparison required (boolean)
- Confidence score: [0.0, 1.0]
- Aggregation level enum: Individual | Daily | Weekly | Monthly | Quarterly | Yearly

#### PainpointResolutionShape
```turtle
jtbd:PainpointResolutionShape a sh:NodeShape ;
    sh:targetClass jtbd:PainpointResolution ;
    sh:property [
        sh:path jtbd:resolvedAt ;
        sh:datatype xsd:dateTime ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:resolutionStatus ;
        sh:in ( "Fully Resolved" "Partially Resolved" "Mitigated" "Workaround" "Unresolved" ) ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:severityBefore ;
        sh:in ( "Minor" "Moderate" "Critical" ) ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:severityAfter ;
        sh:in ( "None" "Minor" "Moderate" "Critical" ) ;
        sh:minCount 1 ;
    ] ;
    sh:property [
        sh:path jtbd:impactReduction ;
        sh:datatype xsd:decimal ;
        sh:minInclusive 0.0 ;
        sh:maxInclusive 100.0 ;
    ] ;
```

**Constraints:**
- Resolution timestamp required
- Status: Fully Resolved | Partially Resolved | Mitigated | Workaround | Unresolved
- Before/After severity: Minor | Moderate | Critical (before) + None (after only)
- Impact reduction: [0.0, 100.0]%

#### EnhancedPersonaShape
```turtle
jtbd:EnhancedPersonaShape a sh:NodeShape ;
    sh:targetClass jtbd:Persona ;
    sh:property [
        sh:path jtbd:personaName ;
        sh:minLength 3 ;
        sh:maxLength 50 ;
    ] ;
    sh:property [
        sh:path jtbd:overallSatisfaction ;
        sh:minInclusive 1.0 ;
        sh:maxInclusive 10.0 ;
    ] ;
    sh:property [
        sh:path jtbd:npsScore ;
        sh:minInclusive -100 ;
        sh:maxInclusive 100 ;
    ] ;
    sh:property [
        sh:path jtbd:jobFrequencyActual ;
        sh:in ( "Multiple Daily" "Daily" "Weekly" "Monthly" "Quarterly" "Rarely" ) ;
    ] ;
    sh:property [
        sh:path jtbd:adoptionRate ;
        sh:minInclusive 0.0 ;
        sh:maxInclusive 100.0 ;
    ] ;
```

**Constraints:**
- Persona name: [3, 50] characters
- Satisfaction score: [1.0, 10.0]
- NPS score: [-100, 100]
- Job frequency enum: Multiple Daily | Daily | Weekly | Monthly | Quarterly | Rarely
- Adoption rate: [0.0, 100.0]%

#### MetricsDashboardShape & OutcomeReportShape
- Dashboard aggregation levels: Hourly | Daily | Weekly | Monthly | Quarterly | Yearly
- Refresh frequency: Real-time | Hourly | Daily | Weekly | On-demand
- Report trend direction: Improving | Stable | Declining | Insufficient Data
- Achievement rate: [0.0, 100.0]%

---

### 2.3 Feature Specification Shapes

**File:** `ontology/spec-kit-schema.ttl`

#### FeatureShape
```turtle
sk:FeatureShape a shacl:NodeShape ;
    shacl:targetClass sk:Feature ;
    shacl:property [
        shacl:path sk:featureBranch ;
        shacl:pattern "^[0-9]{3}-[a-z0-9-]+$" ;
    ] ;
    shacl:property [
        shacl:path sk:featureName ;
        shacl:minLength 5 ;
    ] ;
    shacl:property [
        shacl:path sk:status ;
        shacl:in ("Draft" "In Progress" "Complete" "Deprecated") ;
    ] ;
    shacl:property [
        shacl:path sk:hasUserStory ;
        shacl:class sk:UserStory ;
        shacl:minCount 1 ;
    ] ;
```

**Constraints:**
- Branch: `^[0-9]{3}-[a-z0-9-]+$` (NNN-feature-name)
- Feature name: ≥5 characters
- Status enum: Draft | In Progress | Complete | Deprecated
- ≥1 user story required
- ≥1 functional requirement required
- ≥1 success criterion required

#### UserStoryShape
```turtle
sk:UserStoryShape a shacl:NodeShape ;
    shacl:targetClass sk:UserStory ;
    shacl:property [
        shacl:path sk:storyIndex ;
        shacl:minInclusive 1 ;
    ] ;
    shacl:property [
        shacl:path sk:priority ;
        shacl:in ("P1" "P2" "P3") ;
    ] ;
    shacl:property [
        shacl:path sk:description ;
        shacl:minLength 20 ;
    ] ;
    shacl:property [
        shacl:path sk:hasAcceptanceScenario ;
        shacl:class sk:AcceptanceScenario ;
        shacl:minCount 1 ;
    ] ;
```

**Constraints:**
- Story index: positive integer
- Priority: P1 (critical) | P2 (important) | P3 (nice-to-have)
- Description: ≥20 characters
- ≥1 acceptance scenario required

#### AcceptanceScenarioShape
```turtle
sk:AcceptanceScenarioShape a shacl:NodeShape ;
    shacl:targetClass sk:AcceptanceScenario ;
    shacl:property [
        shacl:path sk:scenarioIndex ;
        shacl:minInclusive 1 ;
    ] ;
    shacl:property [
        shacl:path sk:given ;
        shacl:minLength 5 ;
    ] ;
    shacl:property [
        shacl:path sk:when ;
        shacl:minLength 5 ;
    ] ;
    shacl:property [
        shacl:path sk:then ;
        shacl:minLength 5 ;
    ] ;
```

**Constraints:**
- Given/When/Then clauses: ≥5 characters each
- Standard BDD (Behavior-Driven Development) format

---

## 3. Hook Interface Definitions

### 3.1 Core Hook Pattern

The spec-kit framework defines hooks via **OWL class hierarchies** with **SHACL property constraints**:

```turtle
# Hook Definition Pattern
SomeHook a owl:Class ;
    rdfs:comment "Descriptive hook purpose" ;
    rdfs:subClassOf owl:Thing .

someHookProperty a owl:DatatypeProperty ;
    rdfs:domain SomeHook ;
    rdfs:range xsd:string ;
    rdfs:comment "Property description with validation intent" .

# Shape Constraint (validation)
SomeHookShape a sh:NodeShape ;
    sh:targetClass SomeHook ;
    sh:property [
        sh:path someHookProperty ;
        sh:datatype xsd:string ;
        sh:minCount 1 ;
        sh:pattern "^[a-z].*$" ;
        sh:message "Descriptive validation error" ;
    ] .
```

### 3.2 Hook Categories by Domain

#### Feature Development Hooks
- `sk:Feature` — Top-level feature container
  - `sk:hasUserStory` → `sk:UserStory[]`
  - `sk:hasAcceptanceScenario` → `sk:AcceptanceScenario[]`
  - `sk:hasFunctionalRequirement` → `sk:FunctionalRequirement[]`
  - `sk:hasSuccessCriterion` → `sk:SuccessCriterion[]`
  - `sk:hasEntity` → `sk:Entity[]`
  - `sk:hasEdgeCase` → `sk:EdgeCase[]`
  - `sk:hasDependency` → `sk:Dependency[]`

#### CLI Command Hooks
- `cli:Command` — CLI command definition
  - `cli:hasArgument` → `cli:Argument[]`
  - `cli:hasOption` → `cli:Option[]`
  - `cli:hasExample` → `cli:Example[]`
  - `cli:hasErrorCase` → `sk:CommandErrorCase[]`

#### JTBD Framework Hooks
- `jtbd:Job` — Jobs-to-be-done container
  - `jtbd:hasOutcome` → `jtbd:DesiredOutcome[]`
  - `jtbd:hasPainpoint` → `jtbd:Painpoint[]`
  - `jtbd:hasProgressMaker` → `jtbd:ProgressMaker[]`
  - `jtbd:hasCircumstance` → `jtbd:JobCircumstance[]`

- `jtbd:Persona` — Customer archetype
  - `jtbd:completesJob` → `jtbd:JobCompletion[]`
  - `jtbd:performsJob` → `jtbd:Job[]`

#### Documentation Hooks
- `sk:Documentation` — Documentation container
  - `sk:hasWorkflowPhase` → `sk:WorkflowPhase[]`
  - `sk:hasAuthor` → `sk:Author[]`
  - `sk:hasPrinciple` → `sk:Principle[]`

### 3.3 Hook Composition Rules

**Required Hooks (must be present):**
- Feature → ≥1 UserStory
- Feature → ≥1 FunctionalRequirement
- Feature → ≥1 SuccessCriterion
- UserStory → ≥1 AcceptanceScenario
- AcceptanceScenario → Given + When + Then (all required)
- CLI Command → ≥1 Argument or Option

**Optional Hooks (0 or more):**
- Feature → EdgeCases
- Feature → Dependencies
- Feature → Assumptions
- Job → Painpoints
- Persona → SatisfactionSurveys

---

## 4. Example Hook Instantiations

### 4.1 CLI Command Hook Example

**File:** `ontology/cli-commands.ttl`

```turtle
# CLI Command Instantiation
cli:InitCommand a owl:Class ;
    rdfs:subClassOf cli:Command ;
    rdfs:label "init" ;
    rdfs:comment "Initialize a new Specify project from the latest template" .

# Positional Argument Hook
cli:init_project_name a owl:DatatypeProperty ;
    rdfs:domain cli:InitCommand ;
    rdfs:range xsd:string ;
    rdfs:comment "[type:argument] [index:0] [required:false] Name of the project to create" .

# Optional Flag Hook
cli:init_here a owl:DatatypeProperty ;
    rdfs:domain cli:InitCommand ;
    rdfs:range xsd:boolean ;
    rdfs:comment "[type:option] [flag:--here] [short:-H] [default:false] [required:false] Initialize in current directory" .

# Environment Variable Hook
cli:init_github_token a owl:DatatypeProperty ;
    rdfs:domain cli:InitCommand ;
    rdfs:range xsd:string ;
    rdfs:comment "[type:option] [flag:--github-token] [env:GH_TOKEN] [required:false] GitHub token" .
```

**SHACL Validation Applied:**
```
✓ Command name 'init' matches ^[a-z][a-z0-9-]*$
✓ Description present and ≥10 characters
✓ Arguments have types (argument, option, flag)
✓ Flags match ^--[a-z][a-z0-9-]*$ and ^-[a-zA-Z]$
```

**Hook Metadata Embedded in Comment String:**
```
[type:argument|option|flag]  — Hook kind
[index:N]                    — Positional index
[flag:--name]                — Long flag name
[short:-x]                   — Short flag name
[default:value]              — Default value
[required:true|false]        — Requirement status
[env:VARNAME]                — Environment variable binding
[pytype:Type]                — Python type hint
```

### 4.2 JTBD Hook Example

**File:** `memory/jtbd-customer-jobs.ttl`

```turtle
# JTBD Customer Persona Hook
jtbd:DevLeadDana a jtbd:Persona ;
    jtbd:personaName "Dev Lead Dana" ;
    jtbd:personaRole "Engineering Lead" ;
    jtbd:personaGoals "Deliver features on time with minimal rework" ;
    jtbd:personaChallenges "Ambiguous requirements, unclear acceptance criteria" ;
    jtbd:jobFrequencyActual "Daily" ;
    jtbd:adoptionRate 75.0 ;
    jtbd:npsScore 42 ;
    jtbd:overallSatisfaction 7.0 .

# JTBD Functional Job Hook
jtbd:ClarifyRequirements a jtbd:FunctionalJob ;
    jtbd:jobStatement "When starting a new feature, I want to clarify ambiguous requirements, so I can implement correctly without rework" ;
    jtbd:jobDescription "Identify and resolve unclear requirements before implementation begins" ;
    jtbd:performedBy jtbd:DevLeadDana ;
    jtbd:measuredOutcome "Reduce rework cycles from 3 to 1 per feature" .

# JTBD Outcome Hook
jtbd:ReducedReworkCycles a jtbd:DesiredOutcome ;
    jtbd:outcomeStatement "Minimize rework cycles" ;
    jtbd:targetValue "1 per feature" ;
    jtbd:currentValue "3 per feature" ;
    jtbd:achievedByJob jtbd:ClarifyRequirements ;
    jtbd:measuresMetric jtbd:ReworkCyclesMetric .

# JTBD Painpoint Hook
jtbd:AmbiguousRequirements a jtbd:Painpoint ;
    jtbd:painpointName "Ambiguous Requirements" ;
    jtbd:description "Feature specs contain unclear language causing implementation rework" ;
    jtbd:severity "Critical" ;
    jtbd:affectsPersona jtbd:DevLeadDana ;
    jtbd:preventedBy jtbd:ClarifyRequirements .
```

**Validation Applied (jtbd-shapes.ttl):**
```
✓ Persona name: [3, 50] characters
✓ Satisfaction score: [1.0, 10.0]
✓ NPS score: [-100, 100]
✓ Job frequency: matches enum (Daily)
✓ Adoption rate: [0.0, 100.0]%
```

### 4.3 Feature Specification Hook Example

**Conceptual (no file in kit, pattern documented):**

```turtle
sk:AuthenticationFeature a sk:Feature ;
    sk:featureBranch "042-user-authentication" ;
    sk:featureName "User Authentication System" ;
    sk:created "2025-12-15"^^xsd:date ;
    sk:status "In Progress" ;
    sk:userInput "Enable secure login with email/password and OAuth" ;
    sk:hasUserStory sk:US1_EmailLogin, sk:US2_OAuthIntegration, sk:US3_SessionManagement .

sk:US1_EmailLogin a sk:UserStory ;
    sk:storyIndex 1 ;
    sk:title "Email and Password Login" ;
    sk:priority "P1" ;
    sk:description "As a user, I want to log in with email and password so I can access the application" ;
    sk:priorityRationale "Core feature, blocks all other authentication paths" ;
    sk:independentTest "Test with valid/invalid credentials in test environment" ;
    sk:hasAcceptanceScenario sk:AS1_ValidCredentials, sk:AS2_InvalidPassword .

sk:AS1_ValidCredentials a sk:AcceptanceScenario ;
    sk:scenarioIndex 1 ;
    sk:given "User is on the login page" ;
    sk:when "User enters valid email and password and clicks Login" ;
    sk:then "User is logged in and redirected to dashboard" .

sk:FR_001 a sk:FunctionalRequirement ;
    sk:requirementId "FR-001" ;
    sk:category "Authentication" ;
    sk:description "System must validate email format before database lookup" .

sk:SC_001 a sk:SuccessCriterion ;
    sk:criterionId "SC-001" ;
    sk:measurable true ;
    sk:metric "Login latency" ;
    sk:target "< 2 seconds" ;
    sk:description "Authentication request must complete within 2 seconds" .
```

**Validation Applied (spec-kit-schema.ttl):**
```
✓ Feature branch: matches ^[0-9]{3}-[a-z0-9-]+$ ✓
✓ Feature name: ≥5 characters
✓ Status: "In Progress" ∈ {Draft, In Progress, Complete, Deprecated}
✓ Has ≥1 user story
✓ User stories have ≥1 acceptance scenario
✓ User stories have priority: P1 | P2 | P3
✓ Requirement IDs: match FR-[0-9]{3} pattern
✓ Success criteria: match SC-[0-9]{3} pattern
```

---

## 5. Hook Registration & Activation Mechanisms

### 5.1 SHACL Validation Gate

**Validation Pipeline:**
```
.ttl ontology files
    ↓
[μ₁] SHACL validate (cli-command-shapes.ttl, jtbd-shapes.ttl, spec-kit-schema.ttl)
    ↓
Validation result → violations | warnings | pass
    ↓
[μ₂] SPARQL query (if valid)
    ↓
[μ₃] Tera template render
    ↓
Output artifact (.py, .md, .json)
```

**SHACL Validation Command:**
```bash
shacl validate -s ontology/cli-command-shapes.ttl -d ontology/cli-commands.ttl
```

**Implementation:** `src/specify_cli/ops/ggen_shacl.py`

```python
@dataclass
class SHACLValidationResult:
    valid: bool
    violations: list[str]
    warnings: list[str]
    details: dict[str, Any]

def validate_rdf(
    rdf_files: list[str] | None = None,
    rdf_content: str | None = None,
    shapes_files: list[str] | None = None,
    shapes_content: str | None = None,
) -> SHACLValidationResult:
    """Validate RDF against SHACL shapes."""
```

### 5.2 Hook Discovery & Extraction

**Discovery Pattern:**
1. Load ontology `.ttl` file as RDF graph
2. Find all `sh:NodeShape` instances (constraint definitions)
3. For each shape, identify:
   - `sh:targetClass` (hook class)
   - `sh:property` constraints (cardinality, datatype, pattern, enum)
4. Enumerate all instances of hook classes in data files
5. Validate instances against shape constraints

**SPARQL Query Pattern:**
```sparql
PREFIX sh: <http://www.w3.org/ns/shacl#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

SELECT ?shape ?targetClass ?property ?constraint
WHERE {
    ?shape a sh:NodeShape ;
           sh:targetClass ?targetClass ;
           sh:property ?propertyNode .
    ?propertyNode sh:path ?property ;
                  ?constraint ?value .
}
```

### 5.3 Hook Instantiation Hooks

**Pattern Recognition in Comments:**
```
[type:argument]    — CLI positional argument hook
[type:option]      — CLI named option hook
[type:flag]        — CLI boolean flag hook
[required:true]    — Cardinality hook (minCount 1)
[required:false]   — Cardinality hook (minCount 0)
[env:VARNAME]      — Environment variable binding hook
[default:value]    — Default value hook
```

Example extraction:
```python
rdfs:comment "[type:option] [flag:--verbose] [short:-v] [required:false] Enable verbose output"

→ Hook Type: option
→ Flag: --verbose
→ Short: -v
→ Required: false
```

---

## 6. Hook Validation Rules & Enforcement

### 6.1 Required Constraints (Must Pass)

| Hook | Constraint | Rationale |
|------|-----------|-----------|
| `sk:CLICommand` | ≥1 argument or option | Cannot execute command with no inputs |
| `sk:CLICommand.commandName` | `^[a-z][a-z0-9-]*$` | Prevents invalid shell invocation |
| `sk:UserStory` | ≥1 AcceptanceScenario | Cannot test without acceptance criteria |
| `jtbd:JobCompletion` | ISO 8601 timestamp | Temporal ordering requires precision |
| `jtbd:OutcomeAchievement.confidenceScore` | [0.0, 1.0] | Probability space constraint |
| `cli:CommandErrorCase.errorExpectedExitCode` | [0, 255] | OS exit code range |

### 6.2 Recommended Constraints (Should Pass)

| Hook | Constraint | Rationale |
|------|-----------|-----------|
| `sk:Feature.featureName` | ≥5 characters | Descriptive naming |
| `sk:AcceptanceScenario` | Given/When/Then ≥5 chars | BDD clarity |
| `sk:UserStory.priority` | P1\|P2\|P3 | Prioritization discipline |
| `jtbd:Persona.personaName` | [3, 50] characters | Readability |

### 6.3 Validation Error Examples

**Violation: Missing Description**
```
sk:CheckCommand a sk:CLICommand ;
    sk:commandName "check" .
    # Missing sk:commandDescription

Error: "Command description is required and must be at least 10 characters"
```

**Violation: Invalid Argument Type**
```
sk:MyArg a sk:CommandArgument ;
    sk:argumentType "invalid_type" .

Error: "Argument type must be one of: string, int, float, path, boolean"
```

**Violation: Out-of-Range NPS Score**
```
jtbd:Persona_A
    jtbd:npsScore 150 .  # Max is 100

Error: "Net Promoter Score must be between -100 and 100"
```

**Violation: Malformed Exit Code**
```
sk:ErrorCase_A
    sk:errorExpectedExitCode 256 .  # Max is 255

Error: "Exit code must be between 0 and 255"
```

---

## 7. Hook Composition & Dependency Graph

### 7.1 Hook Hierarchy

```
owl:Thing
├── sk:Feature
│   ├── hasUserStory → sk:UserStory
│   │   └── hasAcceptanceScenario → sk:AcceptanceScenario
│   ├── hasFunctionalRequirement → sk:FunctionalRequirement
│   ├── hasSuccessCriterion → sk:SuccessCriterion
│   ├── hasEntity → sk:Entity
│   ├── hasEdgeCase → sk:EdgeCase
│   └── hasImplementationPlan → sk:ImplementationPlan
│       └── hasTask → sk:Task
│
├── cli:Command
│   ├── hasArgument → cli:Argument
│   ├── hasOption → cli:Option
│   └── hasErrorCase → sk:CommandErrorCase
│
├── jtbd:Job
│   ├── hasOutcome → jtbd:DesiredOutcome
│   ├── hasPainpoint → jtbd:Painpoint
│   ├── hasProgressMaker → jtbd:ProgressMaker
│   └── hasCircumstance → jtbd:JobCircumstance
│
└── sk:Documentation
    ├── hasWorkflowPhase → sk:WorkflowPhase
    └── hasAuthor → sk:Author
```

### 7.2 Cross-Domain Hook Linking

| Hook A | Links to | Hook B | Purpose |
|--------|----------|--------|---------|
| `sk:Feature` | `sk:hasClarification` | `sk:Clarification` | Capture requirement ambiguities |
| `jtbd:FeatureImpactAnalysis` | `jtbd:analyzesFeature` | `sk:Feature` | Measure feature impact on jobs |
| `sk:Task` | `sk:userStoryRef` | `sk:UserStory` | Trace tasks to stories |
| `cli:Command` | `cli:belongsToGroup` | `cli:CommandGroup` | Organize commands hierarchically |

---

## 8. Key Hook Pattern Summary

### 8.1 Pattern: Structured Metadata in Comments

```turtle
# Single-line metadata encoding
rdfs:comment "[type:option] [flag:--name] [short:-n] [default:value] [required:true] Description text" .

# Multi-field parsing:
# [KEY:VALUE] pairs, space-separated
# Description text at end (no [KEY:VALUE] syntax)
```

**Advantage:** Avoids RDF property explosion while embedding rich metadata.

### 8.2 Pattern: Enum as SHACL sh:in

```turtle
sh:property [
    sh:path jtbd:aggregationLevel ;
    sh:in ( "Individual" "Daily" "Weekly" "Monthly" "Quarterly" "Yearly" ) ;
    sh:message "Aggregation level must be one of: ..." ;
] .
```

**Advantage:** Tight validation with human-readable error messages.

### 8.3 Pattern: Constitutional Equations

```
output.md = μ(ontology.ttl)
commands/*.py = μ(cli-commands.ttl)
feature-*.md = μ(feature-spec.ttl)
```

**Advantage:** Single source of truth, deterministic artifact generation.

---

## 9. Integration Points & Usage

### 9.1 ggen v6 Integration

**Pipeline:**
```
ggen --input ontology/cli-commands.ttl \
     --shape ontology/cli-command-shapes.ttl \
     --query sparql/command-test-query.rq \
     --template tera/command-test.tera \
     --output commands/*.py
```

**Stages:**
1. Load RDF graph from `cli-commands.ttl`
2. Validate against SHACL shapes
3. Execute SPARQL query to extract command instances
4. Render Tera template with query results
5. Format Python output with Ruff
6. Verify with pytest --collect-only

### 9.2 IDE Integration (LSP / Language Servers)

**Hook Discovery:**
- RDF ontology outline in editor sidebar
- Jump to hook class definition
- Schema-aware autocomplete for hook properties

**Validation:**
- Real-time SHACL violation markers
- Type-aware property hints
- Pattern validation inline

### 9.3 Testing Integration

**Test Generation:**
```sparql
# Extract all test cases from hooks
SELECT ?commandName ?scenarioName ?given ?when ?then
WHERE {
    ?cmd a cli:Command ;
         cli:commandName ?commandName ;
         cli:hasErrorCase ?errorCase .
    ?errorCase sk:errorScenario ?scenarioName ;
               sk:errorExpectedBehavior ?expected .
}
```

**Rendered as:**
```python
def test_init_command_missing_project_name():
    """Test error case: missing required project name"""
    runner = CliRunner()
    result = runner.invoke(init, [])
    assert result.exit_code == 1
    assert "project name is required" in result.output
```

---

## 10. Discoveries & Notes

### 10.1 Strengths of RDF Hook Approach

1. **Schema-as-Data** — Ontology instances are also valid test data
2. **SHACL Validation** — Constraint checking without custom validators
3. **SPARQL Queries** — Flexible artifact extraction (not tied to templates)
4. **Extensibility** — New domains (JTBD, Documentation) layer cleanly
5. **Determinism** — Mathematical foundation (RDF graph semantics)

### 10.2 Notable Hook Patterns

- **Comment Metadata Encoding** — Avoids RDF property explosion
- **Enum Validation** — SHACL `sh:in` for discrete value sets
- **Cardinality Gates** — `sh:minCount`, `sh:maxCount` enforce required/optional
- **Pattern Matching** — `sh:pattern` for format validation (branch names, flags, etc.)
- **Cross-Domain Links** — `sk:Feature` ↔ `jtbd:FeatureImpactAnalysis` traceability

### 10.3 File Inventory

**Core Ontologies:**
- `ontology/spec-kit-schema.ttl` (717 lines) — Base feature/task/requirement model
- `ontology/cli-schema.ttl` (741 lines) — CLI command vocabulary
- `ontology/jtbd-schema.ttl` (1173 lines) — Jobs-to-be-Done framework

**SHACL Shapes (Validation):**
- `ontology/cli-command-shapes.ttl` (425 lines) — CLI command constraints
- `ontology/jtbd-shapes.ttl` (839 lines) — JTBD measurement & persona constraints
- `ontology/spec-kit-schema.ttl` (embedded) — Feature/story/requirement constraints

**Extensions:**
- `ontology/spec-kit-docs-extension.ttl` (780 lines) — Documentation framework
- `ontology/spec-kit-jtbd-extension.ttl` (700+ lines) — Feature-JTBD integration

**Example Instantiations:**
- `ontology/cli-commands.ttl` (614 lines) — 15+ CLI commands with args/options
- `memory/jtbd-customer-jobs.ttl` (911 lines) — 4 personas, jobs, outcomes, painpoints

### 10.4 Validation Tool Integration

```python
# from src/specify_cli/ops/ggen_shacl.py
def validate_rdf(
    rdf_files: list[str] | None,
    shapes_files: list[str] | None,
) → SHACLValidationResult:
    """Validate RDF data against SHACL shapes."""
    # Returns violations, warnings, details
```

---

## 11. Recommendations for Hook Usage

### For Feature Development

1. **Use SHACL shapes as proof gates** — Each user story must pass `AcceptanceScenarioShape`
2. **Encode metadata in comments** — Reduces RDF boilerplate, improves readability
3. **Link features to JTBD outcomes** — Trace value delivery to customer jobs
4. **Version ontologies alongside code** — Ontology changes are breaking changes

### For CLI Command Definitions

1. **Declare all commands in RDF** — Single source of truth for help, tests, docs
2. **Embed test metadata in hooks** — `errorScenario`, `errorExpectedExitCode`
3. **Generate test cases from hooks** — Use SPARQL to enumerate test matrices
4. **Validate against shapes before release** — ggen should fail if shapes don't pass

### For JTBD Tracking

1. **Record customer segments in RDF** — Enables outcome aggregation queries
2. **Link personas to job completions** — Temporal data + satisfaction metrics
3. **Validate measurement integrity** — `confidenceScore`, `aggregationLevel` prevent garbage data
4. **Generate reports via SPARQL** — Outcome achievement, painpoint resolution trends

---

## Appendix: SPARQL Hook Discovery Query

```sparql
PREFIX sh: <http://www.w3.org/ns/shacl#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

# Find all SHACL shapes and their constraints
SELECT ?shape ?targetClass ?property ?constraint ?constraintValue ?message
WHERE {
    ?shape a sh:NodeShape ;
           sh:targetClass ?targetClass ;
           sh:property ?propertyNode .
    
    ?propertyNode sh:path ?property ;
                  ?constraint ?constraintValue .
    
    OPTIONAL { ?propertyNode sh:message ?message . }
    
    # Constraint types
    FILTER (?constraint IN (
        sh:datatype, sh:class, sh:minCount, sh:maxCount,
        sh:minLength, sh:maxLength, sh:minInclusive, sh:maxInclusive,
        sh:pattern, sh:in
    ))
}
ORDER BY ?shape ?property ?constraint
```

---

## Document Metadata

| Field | Value |
|-------|-------|
| Scan Date | 2026-06-01 |
| Directory Scanned | ~/ggen-spec-kit/ |
| Files Analyzed | 10 primary .ttl files, 2 shape files |
| Total Lines of RDF | 6,400+ |
| Hook Categories | 3 (Feature, CLI, JTBD) |
| SHACL Shapes | 15+ (Feature, UserStory, Scenario, CLI, JTBD, Persona, etc.) |
| Example Instantiations | 2 (CLI commands, JTBD personas) |

---

**End of Report**
