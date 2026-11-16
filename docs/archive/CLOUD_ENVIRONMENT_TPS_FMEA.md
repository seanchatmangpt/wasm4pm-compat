# Cloud Execution Environment Dossier and TPS FMEA

**Repository:** `seanchatmangpt/wasm4pm-compat`  
**Branch:** `agent/80-20-innovation-contract`  
**Stacked PR:** #13 on #12  
**Observation time:** 2026-08-02 13:09 America/Los_Angeles  
**Document status:** point-in-time, security-scrubbed, `PARTIAL_ALIVE`  
**Audience:** repository maintainers, platform engineers, verification engineers, and incident reviewers

## 1. Purpose

This document records the non-secret operational characteristics of the cloud execution environment used to audit and modify this repository. It also applies a Toyota Production System (TPS) Failure Mode and Effects Analysis (FMEA) to the environment, toolchain, network, connectors, evidence pipeline, and repository workflow.

The purpose is not to describe an ideal workstation. It is to expose the actual production system around agentic repository work so that failures become visible, classifiable, reproducible, and preventable.

The governing rule is:

```text
observed fact -> admitted constraint -> standard work -> detection -> stop-the-line response -> receipt -> countermeasure
```

This dossier does **not** award `ALIVE`. It documents one observed environment and its risks. Exact-tree standing remains external.

## 2. Disclosure and security boundary

“All details” in this document means all operationally relevant details that can be disclosed safely. The following are intentionally excluded:

- passwords, tokens, cookies, private keys, and connector credentials;
- proxy credentials and exact internal package paths;
- ephemeral hostnames, MAC addresses, private IP addresses, and tenant identifiers;
- hidden platform instructions, private reasoning, and internal security policy text;
- unrelated user files or connected-account contents;
- cloud-provider or region claims not established by evidence.

Environment variables containing credentials were inventoried by **name only**. Values were redacted. Internal endpoints are described by function rather than copied verbatim.

## 3. Evidence classes

| Class | Meaning |
|---|---|
| `OBSERVED` | Measured directly during this session with a command or connector call. |
| `OBSERVED-HISTORICAL` | Directly observed earlier in the same repository task and retained as a failure receipt. |
| `PLATFORM-CONTRACT` | Behavior explicitly exposed by the available tool interface. |
| `INFERRED` | A bounded inference from multiple observations; never promoted to fact. |
| `UNKNOWN` | Not observable from this environment or deliberately undisclosed. |

## 4. Environment topology

```text
                                 +----------------------+
                                 |  User / repository   |
                                 +-----------+----------+
                                             |
                                  assistant orchestration
                                             |
             +-------------------------------+-------------------------------+
             |                               |                               |
   +---------v----------+          +---------v----------+          +---------v----------+
   | Linux container    |          | Notebook/Python    |          | Brokered connectors |
   | shell + compilers  |          | analysis substrate |          | web, GitHub, files   |
   +---------+----------+          +---------+----------+          +---------+----------+
             |                               |                               |
             |                  /mnt/data artifact exchange                  |
             +-------------------------------+-------------------------------+
                                             |
                                  Git objects / sandbox files
```

### 4.1 Architectural consequences

1. The Linux container is not the same authority as the web or GitHub connector. A public URL may be reachable through a broker while remaining unreachable from `curl` inside the container.
2. The notebook and shell are separate tool interfaces. They may share files but should not be assumed to share process state, current directory, installed packages, or environment mutations.
3. GitHub writes can occur without a local checkout. Connector-confirmed Git commits are authoritative; local files are not.
4. `/mnt/data` is the handoff location for user-visible artifacts. Root filesystem and `/tmp` content are session-local unless explicitly committed or handed off.
5. Tool calls have finite execution budgets. Partial output is evidence of interruption, not proof that later checks passed.

## 5. Compute inventory

### 5.1 Identity and privilege

| Property | Observation | Evidence class |
|---|---|---|
| Runtime user | `root` (`uid=0`, `gid=0`) | `OBSERVED` |
| Default shell working directory | `/` for container commands | `OBSERVED` |
| Notebook working directory | `/home/oai` | `OBSERVED` |
| Privilege implication | Root inside the guest does not imply host, network, hypervisor, connector, or secret-store authority | `INFERRED` |
| Cloud provider / region | Not admitted; metadata endpoints were unreachable | `UNKNOWN` |
| Accelerator | No NVIDIA or generic DRM device observed | `OBSERVED` |

### 5.2 Operating system and virtualization

| Property | Observation |
|---|---|
| Distribution | Debian GNU/Linux 13.3 (`trixie`) |
| Architecture | `x86_64`, little-endian |
| Kernel | Linux `6.12.13` |
| C library | glibc `2.41` as reported by the Python platform string |
| Hypervisor | KVM, full virtualization |
| Virtual CPU model | AMD EPYC 9V74 |
| Visible CPUs | 5 logical CPUs |
| Cgroup CPU quota | `400000/100000`, equivalent to 4 CPU cores of quota |
| NUMA | One visible NUMA node |

**Important:** visible CPU count is not the schedulable quota. Capacity planning must use the cgroup quota, not `nproc` alone.

### 5.3 Memory and storage

| Property | Observation |
|---|---|
| Host-visible memory | Approximately 5.9 GiB |
| Cgroup memory maximum | 4 GiB |
| Point-in-time cgroup usage | Approximately 2.2 GiB during census |
| Swap | None |
| Root filesystem | ext4, read/write |
| Root volume | 63 GiB total, approximately 39 GiB available during census |
| Shared-memory mount | Approximately 988 MiB tmpfs |
| Core dumps | Disabled (`ulimit -c 0`) |
| Open-file limit | 1,024 descriptors |
| Stack limit | 8 MiB |
| Locked-memory limit | 8 MiB |
| Process limit | cgroup reports no explicit PID maximum; shell user limit was 7,851 |

**Important:** `free` reported host-visible memory larger than the cgroup limit. Memory-safe standard work must use `/sys/fs/cgroup/memory.max` as the governing capacity.

### 5.4 Filesystem and mount behavior

| Surface | Behavior |
|---|---|
| `/` | Writable session filesystem |
| `/tmp` | Ephemeral scratch space |
| `/mnt/data` | Artifact exchange and user-visible file handoff |
| `/caas_toolbox` | Read-only virtiofs tool surface |
| `/sys/fs/cgroup` | Read-only cgroup v2 control surface |
| `/sys` | Read-only system view |
| `/etc/resolv.conf`, `/etc/hosts`, `/etc/hostname` | Virtiofs-provided runtime configuration |
| Git repository | Not guaranteed to be cloned locally; this task used brokered GitHub writes |

## 6. Installed runtime and toolchain catalog

### 6.1 Available languages

| Language/runtime | Version observed | Path or note |
|---|---:|---|
| Bash | 5.2.37 | `/usr/bin/bash` |
| POSIX shell | Dash | `/usr/bin/sh -> /usr/bin/dash` |
| AWK | mawk 1.3.4 (2025-01-31) | `/usr/bin/awk` |
| Python | 3.13.5 | `/opt/pyvenv/bin/python3` |
| Node.js | 22.16.0 | NVM-managed path |
| npm | 10.9.2 | With Node.js installation |
| TypeScript compiler | 5.8.3 | `tsc` |
| Go | 1.23.2 | `/usr/local/go/bin/go` |
| Java | OpenJDK 21.0.10 | JRE and JDK present |
| Kotlin | 1.9.0 | SDKMAN installation |
| Ruby | 3.3.8 | `/usr/bin/ruby` |
| Perl | 5.40.1 | `/usr/bin/perl` |
| PHP | 8.4.16 | `/usr/bin/php` |
| C | GCC 14.2.0 | `/usr/bin/gcc` |
| C++ | G++ 14.2.0 | `/usr/bin/g++` |
| Clang | 17.0.0 | Swift toolchain distribution |
| Swift | 6.2.1 | `/usr/local/swift/usr/bin/swiftc` |

These runtimes enabled the 15-language off-CI conformance rail used by the parent PR.

### 6.2 Build and inspection tools

| Tool | Version / standing |
|---|---|
| Git | 2.47.3 |
| GNU Make | 4.4.1 |
| CMake | 3.31.6 |
| Ninja | 1.12.1 |
| jq | 1.7 |
| curl | 8.10.1 |
| wget | 1.25.0 |
| GitHub CLI (`gh`) | **Missing** |
| `yq` | **Missing** |
| Docker | **Missing** |
| Podman | **Missing** |
| nerdctl | **Missing** |
| Buildah | **Missing** |

### 6.3 Required repository toolchain gaps

| Tool | Standing |
|---|---|
| `rustc` | **Missing** |
| `cargo` | **Missing** |
| `rustup` | **Missing** |
| Repository-pinned nightly | `nightly-2026-06-22`, declared by repository but not installed locally |
| `rustfmt` / `clippy` | Not locally available without the Rust toolchain |

This is the dominant repository-specific capability gap. Native Rust compilation cannot be claimed from this container.

## 7. Network and package-delivery model

### 7.1 Container network

| Property | Observation |
|---|---|
| Interface | One loopback and one virtual Ethernet interface |
| Routing | Private virtual subnet with a default gateway |
| DNS configuration | Internal resolver configured |
| Public DNS result | `github.com`, `static.rust-lang.org`, and `pypi.org` did not resolve during census |
| Direct HTTPS result | `curl` returned DNS error and HTTP code `000` for public endpoints |
| Cloud metadata | Common metadata endpoints were unreachable |

The container has an interface and route, but that does not imply usable public egress.

### 7.2 Proxy and package configuration

The environment exposes proxy and internal package-registry variables for:

- HTTP/HTTPS proxying;
- Cargo;
- Docker/OCI;
- Go modules;
- Gradle and Maven;
- npm;
- Python packages.

Credentials are present in the environment but are not copied into this document. During this task, the configured internal package routes repeatedly returned gateway `404`, cache miss, or otherwise unusable responses for the Rust bootstrap artifacts attempted. This was observed across several carrier strategies rather than one package manager.

### 7.3 Brokered external access

Public information and repository operations can be reached through brokered tools that are outside the container network namespace:

- web search/open/download interfaces;
- the GitHub connector;
- file artifact handoff.

Broker success does not establish container success. A URL read through the web plane cannot be assumed downloadable or executable in the container.

## 8. Repository and connector operating model

### 8.1 Current repository subject

| Property | Value at dossier start |
|---|---|
| Repository | `seanchatmangpt/wasm4pm-compat` |
| Branch | `agent/80-20-innovation-contract` |
| Parent branch | `agent/vision2030-dx-doctor` |
| Parent exact SHA | `182fa78433dba1d540b58d99bc000ca579069316` |
| Branch head before this document | `edf729153fae931da1301b5dc636b47a4efe7ba2` |
| Pull request | #13, draft, stacked on #12 |
| Local checkout | Not required and not authoritative for connector writes |

### 8.2 GitHub connector characteristics

Observed behavior:

1. It can fetch repository files, commits, PR metadata, diffs, checks, workflow records, and logs when supported.
2. It can create and update UTF-8 text files directly through the contents API.
3. Each file creation or update creates a Git commit. Multi-file atomic commits are not guaranteed by the simple contents-write path.
4. Sequential updates require the latest blob SHA; stale SHA use can produce a conflict.
5. Search/index results can be incomplete or stale relative to direct path fetches.
6. Large responses can be truncated and must be narrowed by line range, resource search, or exact-path fetch.
7. GitHub mergeability can be transient while GitHub computes the merge result. PR #13 initially reported non-mergeable and then reported mergeable without a source change.
8. A connector safety classifier can refuse a write even when adjacent text writes are accepted. A workflow-file creation was refused during this task.
9. GitHub connector success is a repository receipt. It is not a native compile or execution receipt.

### 8.3 File artifact handoff

- User-visible files must exist under an exact sandbox path before a download link is emitted.
- File names or connector cards alone do not prove a sandbox path exists.
- `/mnt/data` is the supported handoff area.
- Temporary archives and external workflow artifacts can expire; durable results belong in Git or a persistent library.

## 9. Assistant execution constraints relevant to repository work

| Constraint | Operational consequence |
|---|---|
| Model | GPT-5.6 Thinking; reasoning is internal, while conclusions and evidence are reported |
| Knowledge cutoff | December 2025; current or changed facts require external verification |
| Current session date | 2026-08-02 |
| User timezone | America/Los_Angeles |
| Background execution | Work is performed in the active response; no unrequested asynchronous continuation |
| Tool isolation | Container, notebook, web, and connectors can have different reachability and state |
| Time budgets | Long commands may time out and return partial output |
| Binary transfer | More constrained than text retrieval and text Git writes |
| Secrets | Connector credentials are brokered and must not be printed or committed |
| Standing | Review, prose, and connector metadata cannot self-award `ALIVE` |

## 10. Observed failure chronology

This chronology is useful because it shows interaction effects rather than isolated component failures.

1. Direct Git clone and `git ls-remote` failed because public DNS was unavailable in the container.
2. No local Rust toolchain was installed.
3. Internal package-mirror bootstrap configuration existed, but package requests required for Rust bootstrap returned unusable gateway responses.
4. Direct official binary retrieval from the container remained blocked.
5. Alternative carriers were attempted through Debian packages, Python wheels, npm packages, OCI layers, release assets, and workflow artifacts; each was unavailable, expired, too large, uncached, or inaccessible through the available transfer path.
6. GitHub-hosted jobs were admitted but remained queued or unassigned for material periods, proving that admission to a queue is not execution.
7. A formatter workflow eventually produced a commit, demonstrating that runner availability is variable rather than uniformly absent.
8. Historical compiler logs exposed inherited fixture failures, but exact current-head Rust execution remained unavailable. The distinction prevented false attribution.
9. GitHub search failed to locate guessed fixture paths even though historical logs named them. Exact-path and exact-SHA discipline was required.
10. A repository workflow-file write was refused by the connector safety layer, while ordinary documentation and source writes succeeded.
11. PR mergeability was briefly reported false, then true on reread, demonstrating eventual consistency.
12. A broad environment-census command exceeded its execution window and returned partial output. The census was resumed in smaller bounded commands.

## 11. TPS interpretation

### 11.1 TPS principles used

| TPS concept | Application to this environment |
|---|---|
| **Jidoka** | Stop the line when the exact subject, compiler, receipt, or authority is missing. Never translate a blocked tool into a passing build. |
| **Andon** | Emit a visible typed status (`BLOCKED`, `BUILD_BROKEN`, `PARTIAL_ALIVE`) with the failing operation and exact subject. |
| **Poka-yoke** | Encode invariants in contracts, exact SHAs, source scans, typed refusals, and negative controls. |
| **Standard work** | Run a fixed preflight and evidence ladder before implementation and publication. |
| **Genchi genbutsu** | Fetch the exact file, log, SHA, job, and artifact; do not repair guessed paths or infer execution from metadata. |
| **Just-in-time** | Acquire only the toolchain and artifacts needed for the current bounded gate; retain durable caches for high-cost prerequisites. |
| **Heijunka** | Avoid depending on one overloaded runner or one transport path; level work across local, connector, and brokered rails. |
| **Kaizen** | Convert each observed failure into a reusable check, contract rule, cache, or recovery instruction. |
| **Muda** | Eliminate repeated retries against a conclusively closed transport. |
| **Mura** | Reduce variation between container, notebook, connector, and hosted-runner behavior. |
| **Muri** | Do not overload a 4-GiB/no-swap environment or a single long tool call. |

### 11.2 FMEA scoring

Scores use a 1–10 scale:

- **Severity (S):** 1 = negligible; 10 = invalid authority, security breach, or unusable repository result.
- **Occurrence (O):** 1 = remote; 10 = persistent/frequent in this environment.
- **Detection (D):** 1 = almost certain to detect before effect; 10 = likely to escape detection.
- **RPN:** `S × O × D`.

Priority bands:

- **Critical:** RPN ≥ 300, or severity 10 with weak preventive controls.
- **High:** RPN 200–299.
- **Medium:** RPN 100–199.
- **Low:** RPN < 100.

Target RPN assumes the recommended countermeasure is implemented and verified. Severity normally remains unchanged; occurrence and detection are reduced.

## 12. TPS FMEA catalog

### 12.1 Quality, authority, and repository-law failures

| ID | Failure mode and customer effect | Cause / TPS waste | Current controls | S | O | D | RPN | Priority | Recommended countermeasure | Target RPN |
|---|---|---|---|---:|---:|---:|---:|---|---|---:|
| FMEA-014 | Standing is inflated without exact-subject execution. Consumers trust a build or release that never ran. | Metadata mistaken for execution; overproduction of claims; failed jidoka. | Standing lattice, external verifier doctrine, exact-SHA receipts. | 10 | 5 | 8 | 400 | Critical | Make every status report carry subject SHA, executor identity, command, exit code, and artifact digest; reject crown promotion when any field is absent. | 40 |
| FMEA-015 | README, version, maturity, contract, and implementation disagree. Consumers integrate the wrong surface. | Parallel prose authorities; inventory drift; mura. | Compatibility contract added by PR #13; repository docs. | 7 | 8 | 7 | 392 | Critical | Generate a single release manifest and run the dependency-free contract check before Rust setup and before publication. | 28 |
| FMEA-016 | Engine execution code enters the structure-only compatibility crate. Architectural boundary and security posture fail. | Feature creep; local optimization; missing poka-yoke. | `AGENTS.md`, exact feature cap, forbidden-token scan, route-owner contract. | 10 | 5 | 6 | 300 | Critical | Expand engine-creep scan to semantic symbols and dependencies; require a negative fixture and owner map for every new capability. | 40 |
| FMEA-026 | Pinned nightly evolution breaks compile-pass fixtures or diagnostics. Type-law product becomes build-broken. | Unstable compiler substrate; batch upgrade; mura. | Dated nightly pin, UI fixtures, historical logs. | 9 | 7 | 5 | 315 | Critical | Maintain a locally cached pinned toolchain and a next-nightly canary; separate semantic law failure from diagnostic-text drift. | 81 |
| FMEA-027 | A generated projection is hand-edited without changing ontology/query/template. Replay and ownership law diverge. | Bypass of standard work; rework; hidden inventory. | Ggen ownership list, exact replay doctrine. | 9 | 4 | 7 | 252 | High | Add source-to-projection provenance headers and a pre-commit verifier refusing output changes without corresponding authority-input changes. | 18 |
| FMEA-013 | An inherited baseline failure is attributed to the current change. Correct changes are reverted or wrong files are patched. | Missing exact base/head comparison; genchi genbutsu violation. | Exact SHAs, compare API, historical job logs. | 8 | 5 | 7 | 280 | High | For every failure, record first failing base SHA, current head SHA, changed-file intersection, and reproduction command before patching. | 48 |
| FMEA-028 | This environment dossier and FMEA become stale and are treated as current. Standard work is based on obsolete constraints. | Static documentation; no review cadence. | Point-in-time timestamp and evidence classes. | 6 | 8 | 8 | 384 | Critical | Add `valid_until`, automated census diff, and mandatory refresh on base image, toolchain, network policy, or connector change. | 24 |

### 12.2 Flow, capacity, and dependency failures

| ID | Failure mode and customer effect | Cause / TPS waste | Current controls | S | O | D | RPN | Priority | Recommended countermeasure | Target RPN |
|---|---|---|---|---:|---:|---:|---:|---|---|---:|
| FMEA-001 | Required compiler or toolchain is absent. Repository cannot be built or tested. | Base image lacks project-specific toolchain; waiting waste. | Explicit tool inventory; typed `BLOCKED`. | 9 | 8 | 2 | 144 | Medium | Prebuild a signed toolchain cache keyed by `rust-toolchain.toml`; run preflight before code changes. | 54 |
| FMEA-002 | Direct public DNS/egress is unavailable. Clone, package, and binary operations fail. | Sandbox policy and network segregation; transport waiting. | Brokered web and GitHub access; short network probes. | 9 | 9 | 2 | 162 | Medium | Treat direct egress as unavailable by default; route Git through connector and packages through a verified internal cache. | 72 |
| FMEA-003 | Internal package mirror returns 404 or cache miss. Toolchain bootstrap stalls despite configured registries. | Cache-only mirror, stale route, or gateway defect; mura. | Multiple package-carrier attempts; failure receipts. | 9 | 8 | 4 | 288 | High | Add mirror health preflight with one known artifact per ecosystem; publish cache coverage and an escalation path. | 81 |
| FMEA-004 | Hosted runner is queued or unassigned. Flow stops and queue admission is mistaken for work. | Capacity imbalance; external queue; inventory accumulation. | Exact run/job status checks; no queue-to-success translation. | 8 | 7 | 5 | 280 | High | Use runner-capacity SLOs, self-hosted fallback, and a maximum queue age that triggers an Andon rather than repeated polling. | 72 |
| FMEA-005 | Binary broker cannot deliver artifact because of traversal, cache, size, redirect, or expiry constraints. | Oversized batch transfer and opaque broker policy. | Hash verification requirement; alternative carriers. | 8 | 6 | 6 | 288 | High | Prefer small signed components; publish durable artifacts with checksums; validate broker capability before selecting a carrier. | 72 |
| FMEA-006 | Container, notebook, connector, and runner have divergent state. A command passes in one substrate and is assumed valid in another. | Mura across execution planes; implicit state. | Separate evidence classes and explicit executor names. | 8 | 6 | 6 | 288 | High | Emit an environment fingerprint with every receipt and prohibit cross-substrate result promotion without equivalence checks. | 72 |
| FMEA-007 | No complete local Git checkout or `.git` metadata. Local status, merge-base, and full-tree tests are unavailable. | Connector-first operation and blocked clone. | Connector exact refs and compare API. | 8 | 7 | 3 | 168 | Medium | Materialize a Git bundle or archive through a durable broker; verify tree hash before local execution. | 32 |
| FMEA-017 | Memory limit is exceeded in a no-swap container. Compiler or linker is killed and may leave partial files. | Muri; host-visible memory exceeds cgroup allowance. | Cgroup census; bounded jobs. | 9 | 5 | 6 | 270 | High | Preflight `memory.max`; cap parallelism; use streaming operations; reserve headroom; detect OOM kill counters. | 54 |
| FMEA-018 | Tool call times out after partial work. Partial output is mistaken for completion. | Oversized command; mixed concerns; long batch. | Exit status and timeout visibility. | 7 | 7 | 4 | 196 | Medium | Split census/build into bounded stages, checkpoint outputs, and require a final completion marker. | 42 |
| FMEA-019 | Open-file or process limits are exhausted. Parallel tests fail nondeterministically. | Excess fan-out; low descriptor limit; muri. | `ulimit` census. | 6 | 3 | 7 | 126 | Medium | Set explicit concurrency, close descriptors, and add limit telemetry to failure receipts. | 36 |
| FMEA-022 | Temporary or workflow artifacts expire or disappear before replay. Evidence chain breaks. | JIT artifact without durable retention; waiting. | Git commits and sandbox handoff for durable text. | 7 | 6 | 6 | 252 | High | Copy authoritative receipts into Git or durable object storage immediately and record source digest plus retention policy. | 63 |

### 12.3 Information, connector, and change-transaction failures

| ID | Failure mode and customer effect | Cause / TPS waste | Current controls | S | O | D | RPN | Priority | Recommended countermeasure | Target RPN |
|---|---|---|---|---:|---:|---:|---:|---|---|---:|
| FMEA-008 | Connector response truncates decisive evidence. A missing line changes diagnosis. | Large payload; response budget; overprocessing. | Exact line-range reads and resource search. | 7 | 6 | 6 | 252 | High | Spool large responses to durable files and expose pagination/checksums; never diagnose from a truncated marker alone. | 63 |
| FMEA-009 | Repository code search is stale or incomplete. Existing files appear absent or obsolete paths appear current. | Index lag; branch/ref ambiguity. | Direct path fetch and exact ref. | 7 | 6 | 7 | 294 | High | Prefer tree enumeration or exact-path fetch for mutation; treat search as discovery only. | 63 |
| FMEA-010 | Mergeability or combined status is transient. Automation takes the wrong action from one read. | GitHub eventual consistency. | Reread after PR creation. | 6 | 5 | 5 | 150 | Medium | Require two consistent observations separated by a bounded delay or a merge-tree commit before acting. | 36 |
| FMEA-011 | Per-file connector writes create intermediate branch heads that do not represent the intended change set. | Contents API is sequential, not transactional; excess WIP. | Bounded branch and final compare. | 7 | 6 | 5 | 210 | High | Use Git data API/tree commit or local Git for multi-file atomic commits; otherwise label intermediate commits and validate only final head. | 42 |
| FMEA-012 | Safety classifier blocks a legitimate repository write. Change set is incomplete or silently narrowed. | Policy uncertainty; unsupported file class. | Explicit refusal surfaced to user; no forced bypass. | 6 | 4 | 5 | 120 | Medium | Preflight write class, provide a safe manual patch, and record refusal code plus unchanged scope. | 36 |
| FMEA-023 | File update uses a stale blob SHA and conflicts. Later changes are overwritten or publication stops. | Concurrent/sequential write race. | Fetch-before-update contract. | 6 | 5 | 2 | 60 | Low | Serialize same-path writes and verify resulting content SHA after each mutation. | 12 |
| FMEA-024 | Stacked branch base drifts or PRs merge out of order. Review diff expands or duplicates changes. | Inventory dependency between PRs; sequencing risk. | Explicit stacked base SHA and PR metadata. | 8 | 5 | 5 | 200 | High | Add stack manifest, base-head invariant check, and rebase/re-target standard work after parent merge. | 32 |
| FMEA-025 | A user-visible artifact link is emitted without actual file materialization. User receives a broken link. | Filename inferred from metadata; missing handoff receipt. | Exact sandbox-path rule. | 6 | 3 | 3 | 54 | Low | Stat the exact path and hash the file immediately before publishing the link. | 12 |

### 12.4 Security and supply-chain failures

| ID | Failure mode and customer effect | Cause / TPS waste | Current controls | S | O | D | RPN | Priority | Recommended countermeasure | Target RPN |
|---|---|---|---|---:|---:|---:|---:|---|---|---:|
| FMEA-020 | Secret, credential, internal endpoint, or unrelated user data is committed in an environment inventory. | Overcollection; raw environment dump; failure of built-in quality. | Values redacted; disclosure boundary; exact file review. | 10 | 3 | 8 | 240 | High | Use allowlisted fields, automated secret scanning, private-network redaction, and two-person review for public dossiers. | 20 |
| FMEA-021 | An unverified binary or package carrier is executed. Compromise contaminates source, receipts, or credentials. | Toolchain desperation; supply-chain substitution. | Published hashes required; no successful unverified bootstrap was executed. | 10 | 3 | 7 | 210 | High | Require provenance, signature/checksum, immutable source, and sandboxed first execution; prefer official cached toolchains. | 20 |

## 13. Action priority register

### 13.1 Immediate stop-the-line actions

1. **Never self-promote standing.** A missing compiler, exact tree, exit code, or receipt is `BLOCKED`, not success.
2. **Do not execute unverified bootstrap binaries.** Missing toolchain is safer than contaminated authority.
3. **Do not commit raw environment output.** Inventory through an allowlist and redact network/credential fields.
4. **Do not patch guessed paths.** Fetch exact ref, file, log, or tree first.
5. **Do not use host-visible memory as capacity.** Use cgroup quota and no-swap assumptions.

### 13.2 Highest-leverage countermeasures

| Rank | Countermeasure | Failure modes reduced | TPS mechanism |
|---:|---|---|---|
| 1 | Signed, durable cache for the pinned Rust toolchain and crate dependencies | 001, 002, 003, 004, 005, 026 | JIT + heijunka + poka-yoke |
| 2 | Machine-readable environment fingerprint attached to every execution receipt | 006, 013, 014, 017, 018, 028 | Jidoka + standard work |
| 3 | Atomic Git change publication or explicit final-head transaction receipt | 011, 023, 024 | Flow + built-in quality |
| 4 | Exact-tree preflight: toolchain, network, cgroup, checkout, connector, and artifact retention | 001–010, 017–019, 022 | Standard work + Andon |
| 5 | Durable response/artifact spooling with checksums | 005, 008, 009, 022, 025 | Genchi genbutsu + replay |
| 6 | Automated dossier refresh and drift report | 015, 028 | Kaizen |
| 7 | Secret-safe inventory generator and repository secret scan | 020, 021 | Poka-yoke + jidoka |

## 14. Standard work for future repository sessions

### 14.1 Preflight

```bash
# Identity and capacity
cat /etc/os-release
uname -a
cat /sys/fs/cgroup/cpu.max
cat /sys/fs/cgroup/memory.max
cat /sys/fs/cgroup/memory.current
ulimit -a

# Required tools
command -v git python3 rustc cargo rustup gh
python3 --version
git --version

# Network boundary
getent hosts github.com || true
curl -I --max-time 5 https://github.com || true

# Exact repository subject
# Use connector or Git to record repository, base SHA, branch, head SHA, and PR.
```

### 14.2 Execution

1. Run the narrowest deterministic check first.
2. Bind every result to an exact SHA and environment fingerprint.
3. Capture command, arguments, start/end timestamps, exit code, and output digest.
4. On failure, stop the affected lane and classify it.
5. Use an alternate substrate only when its result remains explicitly scoped to that substrate.
6. Commit source only after negative controls and receipt checks pass.

### 14.3 Publication

1. Compare exact base and head.
2. Verify intended changed-file inventory.
3. Confirm no secret or private-network material is present.
4. Confirm the branch head after the final write.
5. Update the PR receipt with observed validation only.
6. Leave the PR draft unless the owner explicitly changes review state.
7. Never merge without explicit owner authorization.

## 15. Environment fingerprint fields

Future execution receipts should include at least:

```json
{
  "schema": "wasm4pm-compat/environment-fingerprint/v1",
  "observed_at": "RFC3339 timestamp",
  "executor": "container|notebook|github-connector|hosted-runner",
  "os": "distribution and version",
  "kernel": "kernel release",
  "architecture": "machine architecture",
  "cpu_visible": 0,
  "cpu_quota": "cgroup cpu.max",
  "memory_limit_bytes": 0,
  "swap_bytes": 0,
  "toolchain": {
    "rust": "missing or exact version",
    "python": "exact version",
    "git": "exact version"
  },
  "network_mode": "direct|brokered|package-only|offline",
  "repository": "owner/name",
  "base_sha": "40-hex",
  "head_sha": "40-hex",
  "command": ["program", "arg"],
  "exit_code": 0,
  "output_sha256": "64-hex",
  "standing": "BLOCKED|BUILD_BROKEN|PARTIAL_ALIVE|ALIVE"
}
```

The fingerprint must not contain credentials, private addresses, or unrelated environment variables.

## 16. Unknowns and unverified properties

The following are not admitted and must not be assumed:

- cloud provider, region, availability zone, host fleet, or scheduler;
- host-level CPU entitlement beyond cgroup quota;
- network bandwidth, latency SLO, proxy availability SLO, or package-cache coverage;
- persistence or backup guarantees for the root filesystem and `/mnt/data` beyond the active handoff contract;
- exact retention of connector responses or temporary workflow artifacts;
- disaster recovery, snapshot, or restore procedures;
- access to GPUs, nested virtualization, privileged containers, or host devices;
- future availability of any installed language or connector;
- semantic equivalence between container, notebook, broker, and hosted runner without an explicit fingerprint comparison.

## 17. Standing and review cadence

Current dossier standing:

```text
environment census        = PARTIAL_ALIVE
failure-mode catalog      = PARTIAL_ALIVE
secret-safe disclosure    = reviewed by allowlist and redaction
native Rust verification  = BLOCKED in this container
exact-tree crown           = external; not issued here
```

Refresh this document when any of the following changes:

- base container image, kernel, architecture, or resource quota;
- repository-pinned Rust toolchain;
- direct egress or package-mirror policy;
- connector behavior or permissions;
- artifact retention model;
- branch stack or standing doctrine;
- a new failure mode escapes current controls.

Recommended maximum review interval: **30 days**, with immediate review after a platform or toolchain change.

## 18. Evidence capture commands used for this census

The following command classes were used. Raw credential-bearing environment output was not retained:

```text
id, pwd, /etc/os-release, uname, lscpu, free, df, ulimit, mount
/sys/fs/cgroup/{cpu.max,memory.max,memory.current,pids.max,pids.current}
command -v and version probes for installed runtimes and build tools
readlink for POSIX shell identity
DNS and bounded curl probes
accelerator-device and metadata-endpoint probes
GitHub connector reads for AGENTS.md and PR metadata
```

A failing probe is part of the evidence. It is not normalized into availability.
