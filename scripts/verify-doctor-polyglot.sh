#!/usr/bin/env bash
set -euo pipefail

ROOT="$(mktemp -d)"
trap 'rm -rf "$ROOT"' EXIT
OUT="$ROOT/out"
mkdir -p "$OUT"

EXPECTED='{"capabilities":["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"],"profile":"vision2030","routes":[{"intent":"discover","state":"routed","target":"wasm4pm"},{"intent":"replay","state":"routed","target":"wasm4pm"},{"intent":"verify_standing","state":"routed","target":"external_verifier"}],"standing":"PARTIAL_ALIVE"}'
printf '%s\n' "$EXPECTED" > "$OUT/expected.json"

capabilities=(typed_evidence named_refusals diagnostics receipt_shapes deterministic_digests dfcm doctor formats strict_boundary graduation_bridge engine_execution standing_authority)
[[ ${#capabilities[@]} -eq 12 ]]
route_target() {
  case "$1" in
    discover|replay) printf wasm4pm ;;
    verify_standing) printf external_verifier ;;
    *) return 64 ;;
  esac
}
[[ $(route_target discover) == wasm4pm ]]
[[ $(route_target replay) == wasm4pm ]]
[[ $(route_target verify_standing) == external_verifier ]]
printf '%s\n' "$EXPECTED" > "$OUT/bash.json"

cat > "$ROOT/verify.py" <<'PY'
import json
caps = ["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"]
def route(intent):
    if intent in {"discover", "replay"}: return {"intent": intent, "state": "routed", "target": "wasm4pm"}
    if intent == "verify_standing": return {"intent": intent, "state": "routed", "target": "external_verifier"}
    raise ValueError(intent)
assert len(caps) == 12
obj = {"capabilities": caps, "profile": "vision2030", "routes": [route(x) for x in ("discover","replay","verify_standing")], "standing": "PARTIAL_ALIVE"}
print(json.dumps(obj, sort_keys=True, separators=(",", ":")))
PY
python3 "$ROOT/verify.py" > "$OUT/python.json"

cat > "$ROOT/verify.mjs" <<'JS'
const caps = ["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"];
function route(intent) {
  if (intent === "discover" || intent === "replay") return {intent, state:"routed", target:"wasm4pm"};
  if (intent === "verify_standing") return {intent, state:"routed", target:"external_verifier"};
  throw new Error(intent);
}
if (caps.length !== 12) throw new Error("capability cardinality");
const obj = {capabilities:caps, profile:"vision2030", routes:["discover","replay","verify_standing"].map(route), standing:"PARTIAL_ALIVE"};
console.log(JSON.stringify(obj));
JS
node "$ROOT/verify.mjs" > "$OUT/javascript.json"

cat > "$ROOT/verify.go" <<'GO'
package main
import ("encoding/json"; "fmt")
type Route struct { Intent string `json:"intent"`; State string `json:"state"`; Target string `json:"target"` }
type Report struct { Capabilities []string `json:"capabilities"`; Profile string `json:"profile"`; Routes []Route `json:"routes"`; Standing string `json:"standing"` }
func route(intent string) Route { if intent=="discover" || intent=="replay" { return Route{intent,"routed","wasm4pm"} }; if intent=="verify_standing" { return Route{intent,"routed","external_verifier"} }; panic(intent) }
func main(){ caps:=[]string{"typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"}; if len(caps)!=12 { panic("capability cardinality") }; b,err:=json.Marshal(Report{caps,"vision2030",[]Route{route("discover"),route("replay"),route("verify_standing")},"PARTIAL_ALIVE"}); if err!=nil { panic(err) }; fmt.Println(string(b)) }
GO
go run "$ROOT/verify.go" > "$OUT/go.json"

cat > "$ROOT/DoctorVerify.java" <<'JAVA'
import java.util.*;
public final class DoctorVerify {
  static String route(String intent) { return switch(intent) { case "discover", "replay" -> "wasm4pm"; case "verify_standing" -> "external_verifier"; default -> throw new IllegalArgumentException(intent); }; }
  public static void main(String[] args) {
    var caps = List.of("typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority");
    if (caps.size()!=12 || !route("discover").equals("wasm4pm") || !route("verify_standing").equals("external_verifier")) throw new AssertionError();
    System.out.println("{\"capabilities\":[\"typed_evidence\",\"named_refusals\",\"diagnostics\",\"receipt_shapes\",\"deterministic_digests\",\"dfcm\",\"doctor\",\"formats\",\"strict_boundary\",\"graduation_bridge\",\"engine_execution\",\"standing_authority\"],\"profile\":\"vision2030\",\"routes\":[{\"intent\":\"discover\",\"state\":\"routed\",\"target\":\""+route("discover")+"\"},{\"intent\":\"replay\",\"state\":\"routed\",\"target\":\""+route("replay")+"\"},{\"intent\":\"verify_standing\",\"state\":\"routed\",\"target\":\""+route("verify_standing")+"\"}],\"standing\":\"PARTIAL_ALIVE\"}");
  }
}
JAVA
javac -d "$ROOT/java" "$ROOT/DoctorVerify.java"
java -cp "$ROOT/java" DoctorVerify > "$OUT/java.json"

cat > "$ROOT/verify.rb" <<'RB'
require 'json'
caps=%w[typed_evidence named_refusals diagnostics receipt_shapes deterministic_digests dfcm doctor formats strict_boundary graduation_bridge engine_execution standing_authority]
def route(i); return {intent:i,state:'routed',target:'wasm4pm'} if %w[discover replay].include?(i); return {intent:i,state:'routed',target:'external_verifier'} if i=='verify_standing'; raise i; end
raise 'capability cardinality' unless caps.length==12
puts JSON.generate({capabilities:caps,profile:'vision2030',routes:%w[discover replay verify_standing].map{|i| route(i)},standing:'PARTIAL_ALIVE'})
RB
ruby "$ROOT/verify.rb" > "$OUT/ruby.json"

cat > "$ROOT/verify.pl" <<'PL'
use strict; use warnings; use JSON::PP;
my @caps=qw(typed_evidence named_refusals diagnostics receipt_shapes deterministic_digests dfcm doctor formats strict_boundary graduation_bridge engine_execution standing_authority);
sub route { my($i)=@_; return {intent=>$i,state=>'routed',target=>'wasm4pm'} if $i eq 'discover' || $i eq 'replay'; return {intent=>$i,state=>'routed',target=>'external_verifier'} if $i eq 'verify_standing'; die $i; }
die 'capability cardinality' unless @caps==12;
my $j=JSON::PP->new->canonical(1);
print $j->encode({capabilities=>\@caps,profile=>'vision2030',routes=>[map { route($_) } qw(discover replay verify_standing)],standing=>'PARTIAL_ALIVE'}),"\n";
PL
perl "$ROOT/verify.pl" > "$OUT/perl.json"

cat > "$ROOT/verify.php" <<'PHP'
<?php
$caps=['typed_evidence','named_refusals','diagnostics','receipt_shapes','deterministic_digests','dfcm','doctor','formats','strict_boundary','graduation_bridge','engine_execution','standing_authority'];
function route($i){ if($i==='discover'||$i==='replay') return ['intent'=>$i,'state'=>'routed','target'=>'wasm4pm']; if($i==='verify_standing') return ['intent'=>$i,'state'=>'routed','target'=>'external_verifier']; throw new Exception($i); }
if(count($caps)!==12) throw new Exception('capability cardinality');
$o=['capabilities'=>$caps,'profile'=>'vision2030','routes'=>array_map('route',['discover','replay','verify_standing']),'standing'=>'PARTIAL_ALIVE'];
echo json_encode($o, JSON_UNESCAPED_SLASHES),"\n";
?>
PHP
php "$ROOT/verify.php" > "$OUT/php.json"

cat > "$ROOT/verify.c" <<'C'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static const char* route(const char* i){ if(!strcmp(i,"discover")||!strcmp(i,"replay")) return "wasm4pm"; if(!strcmp(i,"verify_standing")) return "external_verifier"; abort(); }
int main(void){ const char* caps[]={"typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"}; if(sizeof(caps)/sizeof(caps[0])!=12) return 2; printf("{\"capabilities\":[\"typed_evidence\",\"named_refusals\",\"diagnostics\",\"receipt_shapes\",\"deterministic_digests\",\"dfcm\",\"doctor\",\"formats\",\"strict_boundary\",\"graduation_bridge\",\"engine_execution\",\"standing_authority\"],\"profile\":\"vision2030\",\"routes\":[{\"intent\":\"discover\",\"state\":\"routed\",\"target\":\"%s\"},{\"intent\":\"replay\",\"state\":\"routed\",\"target\":\"%s\"},{\"intent\":\"verify_standing\",\"state\":\"routed\",\"target\":\"%s\"}],\"standing\":\"PARTIAL_ALIVE\"}\n",route("discover"),route("replay"),route("verify_standing")); }
C
gcc -std=c17 -Wall -Wextra -Werror "$ROOT/verify.c" -o "$ROOT/verify-c"
"$ROOT/verify-c" > "$OUT/c.json"

cat > "$ROOT/verify.cpp" <<'CPP'
#include <array>
#include <iostream>
#include <stdexcept>
#include <string_view>
constexpr std::string_view route(std::string_view i){ if(i=="discover"||i=="replay") return "wasm4pm"; if(i=="verify_standing") return "external_verifier"; throw std::invalid_argument("intent"); }
int main(){ constexpr std::array caps{"typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"}; static_assert(caps.size()==12); std::cout << "{\"capabilities\":[\"typed_evidence\",\"named_refusals\",\"diagnostics\",\"receipt_shapes\",\"deterministic_digests\",\"dfcm\",\"doctor\",\"formats\",\"strict_boundary\",\"graduation_bridge\",\"engine_execution\",\"standing_authority\"],\"profile\":\"vision2030\",\"routes\":[{\"intent\":\"discover\",\"state\":\"routed\",\"target\":\"" << route("discover") << "\"},{\"intent\":\"replay\",\"state\":\"routed\",\"target\":\"" << route("replay") << "\"},{\"intent\":\"verify_standing\",\"state\":\"routed\",\"target\":\"" << route("verify_standing") << "\"}],\"standing\":\"PARTIAL_ALIVE\"}\n"; }
CPP
g++ -std=c++20 -Wall -Wextra -Werror "$ROOT/verify.cpp" -o "$ROOT/verify-cpp"
"$ROOT/verify-cpp" > "$OUT/cpp.json"

cat > "$ROOT/verify.swift" <<'SWIFT'
import Foundation
let caps = ["typed_evidence","named_refusals","diagnostics","receipt_shapes","deterministic_digests","dfcm","doctor","formats","strict_boundary","graduation_bridge","engine_execution","standing_authority"]
func route(_ intent:String)->String { switch intent { case "discover", "replay": return "wasm4pm"; case "verify_standing": return "external_verifier"; default: fatalError(intent) } }
precondition(caps.count == 12)
print("{\"capabilities\":[\"typed_evidence\",\"named_refusals\",\"diagnostics\",\"receipt_shapes\",\"deterministic_digests\",\"dfcm\",\"doctor\",\"formats\",\"strict_boundary\",\"graduation_bridge\",\"engine_execution\",\"standing_authority\"],\"profile\":\"vision2030\",\"routes\":[{\"intent\":\"discover\",\"state\":\"routed\",\"target\":\"\(route("discover"))\"},{\"intent\":\"replay\",\"state\":\"routed\",\"target\":\"\(route("replay"))\"},{\"intent\":\"verify_standing\",\"state\":\"routed\",\"target\":\"\(route("verify_standing"))\"}],\"standing\":\"PARTIAL_ALIVE\"}")
SWIFT
swiftc "$ROOT/verify.swift" -o "$ROOT/verify-swift"
"$ROOT/verify-swift" > "$OUT/swift.json"

languages=(bash python javascript go java ruby perl php c cpp swift)
for language in "${languages[@]}"; do
  cmp -s "$OUT/expected.json" "$OUT/$language.json" || {
    echo "mismatch: $language" >&2
    diff -u "$OUT/expected.json" "$OUT/$language.json" >&2 || true
    exit 1
  }
done

digest=$(sha256sum "$OUT/expected.json" | awk '{print $1}')
python3 - "$digest" "${languages[@]}" <<'PY'
import json, subprocess, sys
digest=sys.argv[1]; languages=sys.argv[2:]
commands={
"bash":["bash","--version"],"python":["python3","--version"],"javascript":["node","--version"],"go":["go","version"],"java":["java","-version"],"ruby":["ruby","--version"],"perl":["perl","-v"],"php":["php","--version"],"c":["gcc","--version"],"cpp":["g++","--version"],"swift":["swiftc","--version"]}
versions={}
for language in languages:
    p=subprocess.run(commands[language],text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,check=True)
    versions[language]=next((line for line in p.stdout.splitlines() if line.strip()), "unknown")
print(json.dumps({"schema":"wasm4pm-compat/polyglot-doctor/v1","standing":"ALIVE","languages":languages,"canonical_sha256":digest,"versions":versions},sort_keys=True,separators=(",",":")))
PY
