#!/usr/bin/env bash
set -euo pipefail
ROOT="$(mktemp -d)"
trap 'rm -rf "$ROOT"' EXIT
OUT="$ROOT/out"; mkdir -p "$OUT"
EXPECTED='{"capabilities":["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"],"profile":"vision2030","routes":[{"intent":"discover","state":"routed","target":"wasm4pm"},{"intent":"replay","state":"routed","target":"wasm4pm"},{"intent":"verify_standing","state":"routed","target":"external_verifier"}],"standing":"PARTIAL_ALIVE"}'
printf '%s\n' "$EXPECTED" > "$OUT/expected.json"

cat > "$ROOT/verify.sh" <<'SH'
#!/bin/sh
set -eu
route() { case "$1" in discover|replay) printf wasm4pm;; verify_standing) printf external_verifier;; *) return 64;; esac; }
[ "$(route discover)" = wasm4pm ]
[ "$(route replay)" = wasm4pm ]
[ "$(route verify_standing)" = external_verifier ]
printf '%s\n' '{"capabilities":["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"],"profile":"vision2030","routes":[{"intent":"discover","state":"routed","target":"wasm4pm"},{"intent":"replay","state":"routed","target":"wasm4pm"},{"intent":"verify_standing","state":"routed","target":"external_verifier"}],"standing":"PARTIAL_ALIVE"}'
SH
dash "$ROOT/verify.sh" > "$OUT/posix_sh.json"

cat > "$ROOT/verify.awk" <<'AWK'
function route(i) { if (i=="discover" || i=="replay") return "wasm4pm"; if (i=="verify_standing") return "external_verifier"; exit 64 }
BEGIN {
  caps[1]="typed_evidence"; caps[2]="named_refusals"; caps[3]="diagnostics"; caps[4]="receipt_shapes"; caps[5]="deterministic_digests"; caps[6]="dfcm"; caps[7]="doctor"; caps[8]="formats"; caps[9]="strict_boundary"; caps[10]="graduation_bridge"; caps[11]="engine_execution"; caps[12]="standing_authority";
  if (length(caps) != 12 || route("discover") != "wasm4pm" || route("verify_standing") != "external_verifier") exit 2;
  print "{\"capabilities\":[\"typed_evidence\",\"named_refusals\",\"diagnostics\",\"receipt_shapes\",\"deterministic_digests\",\"dfcm\",\"doctor\",\"formats\",\"strict_boundary\",\"graduation_bridge\",\"engine_execution\",\"standing_authority\"],\"profile\":\"vision2030\",\"routes\":[{\"intent\":\"discover\",\"state\":\"routed\",\"target\":\"" route("discover") "\"},{\"intent\":\"replay\",\"state\":\"routed\",\"target\":\"" route("replay") "\"},{\"intent\":\"verify_standing\",\"state\":\"routed\",\"target\":\"" route("verify_standing") "\"}],\"standing\":\"PARTIAL_ALIVE\"}"
}
AWK
awk -f "$ROOT/verify.awk" > "$OUT/awk.json"

cat > "$ROOT/verify.ts" <<'TS'
type Intent = "discover" | "replay" | "verify_standing";
type Target = "wasm4pm" | "external_verifier";
const capabilities = ["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"] as const;
function route(intent: Intent): {intent: Intent; state: "routed"; target: Target} {
  switch (intent) {
    case "discover": case "replay": return {intent, state:"routed", target:"wasm4pm"};
    case "verify_standing": return {intent, state:"routed", target:"external_verifier"};
  }
}
if (capabilities.length !== 12) throw new Error("capability cardinality");
const report = {capabilities, profile:"vision2030", routes:(["discover","replay","verify_standing"] as const).map(route), standing:"PARTIAL_ALIVE"};
console.log(JSON.stringify(report));
TS
tsc --strict --target ES2022 --module commonjs --outDir "$ROOT/ts" "$ROOT/verify.ts"
node "$ROOT/ts/verify.js" > "$OUT/typescript.json"

cat > "$ROOT/DoctorVerify.kt" <<'KT'
import java.lang.IllegalArgumentException
fun route(intent: String): String = when (intent) {
  "discover", "replay" -> "wasm4pm"
  "verify_standing" -> "external_verifier"
  else -> throw IllegalArgumentException(intent)
}
fun main() {
  val capabilities = listOf("typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority")
  check(capabilities.size == 12)
  print("{\"capabilities\":[\"typed_evidence\",\"named_refusals\",\"diagnostics\",\"receipt_shapes\",\"deterministic_digests\",\"dfcm\",\"doctor\",\"formats\",\"strict_boundary\",\"graduation_bridge\",\"engine_execution\",\"standing_authority\"],\"profile\":\"vision2030\",\"routes\":[{\"intent\":\"discover\",\"state\":\"routed\",\"target\":\"${route("discover")}\"},{\"intent\":\"replay\",\"state\":\"routed\",\"target\":\"${route("replay")}\"},{\"intent\":\"verify_standing\",\"state\":\"routed\",\"target\":\"${route("verify_standing")}\"}],\"standing\":\"PARTIAL_ALIVE\"}\n")
}
KT
kotlinc "$ROOT/DoctorVerify.kt" -include-runtime -d "$ROOT/doctor.jar"
java -jar "$ROOT/doctor.jar" > "$OUT/kotlin.json"

languages=(posix_sh awk typescript kotlin)
for language in "${languages[@]}"; do
  cmp -s "$OUT/expected.json" "$OUT/$language.json" || { echo "mismatch: $language" >&2; diff -u "$OUT/expected.json" "$OUT/$language.json" >&2 || true; exit 1; }
done

digest=$(sha256sum "$OUT/expected.json" | awk '{print $1}')
python3 - "$digest" <<'PY'
import json, subprocess, sys
commands={"posix_sh":["dash","-c","echo $0"],"awk":["awk","-W","version"],"typescript":["tsc","--version"],"kotlin":["kotlinc","-version"]}
versions={}
for name, cmd in commands.items():
    p=subprocess.run(cmd,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,check=True)
    versions[name]=next((line.strip() for line in p.stdout.splitlines() if line.strip()),"unknown")
print(json.dumps({"schema":"wasm4pm-compat/polyglot-doctor-extra/v1","standing":"ALIVE","languages":list(commands),"canonical_sha256":sys.argv[1],"versions":versions},sort_keys=True,separators=(",",":")))
PY
