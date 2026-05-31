// COMPILE-FAIL: Rejects a lossy transformation that omits LossPolicy — proves silent structure loss is a compile-time defect enforced by the Project trait.
//
// Law: loss-project-requires-named-policy
// Paper: "Object-Centric Process Mining" (van der Aalst et al.)
//
// The Project trait's `project` method requires a LossPolicy argument that names
// how loss is handled *before* loss occurs. This is the compile-time gate that
// prevents silent structure loss: a caller cannot invoke project() without
// explicitly declaring a LossPolicy variant.
//
// This fixture attempts to call project() by passing a plain bool instead of a
// LossPolicy. The type system must reject this — there is no implicit coercion
// from bool (or any non-LossPolicy type) to LossPolicy, so omitting an explicit
// named policy is a compile-time defect, not a runtime surprise.
//
// Expected error: mismatched types — bool is not LossPolicy.
use wasm4pm_compat::loss::{LossPolicy, LossReport, Project, ProjectionName};

struct OcelFlatten {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

enum OcelShape {}
enum XesShape {}

impl Project for OcelFlatten {
    type From = OcelShape;
    type To = XesShape;
    type Lost = Vec<&'static str>;
    type Reason = &'static str;

    fn project(
        self,
        policy: LossPolicy,
    ) -> Result<LossReport<Self::From, Self::To, Self::Lost>, Self::Reason> {
        let dropped: Vec<&'static str> = self
            .object_types
            .iter()
            .copied()
            .filter(|t| *t != self.case_type)
            .collect();
        if !dropped.is_empty() && policy == LossPolicy::RefuseLoss {
            return Err("FlatteningLoss");
        }
        Ok(LossReport::new(
            ProjectionName("ocel-flatten-to-xes:by-case"),
            policy,
            dropped,
        ))
    }
}

fn main() {
    // VIOLATION: project() requires a LossPolicy — a named, explicit policy that
    // governs how structure loss is handled. Passing `true` (a bool) instead of a
    // LossPolicy variant omits the required named policy and must be rejected by
    // the type system. Silent structure loss is a compile-time defect.
    let _ = OcelFlatten {
        object_types: vec!["order", "item"],
        case_type: "order",
    }
    .project(true);
}
