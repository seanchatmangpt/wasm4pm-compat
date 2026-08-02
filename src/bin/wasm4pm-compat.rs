//! Developer CLI for the structure-only compatibility doctor.

use serde::Serialize;
use std::process::ExitCode;
use wasm4pm_compat::diagnostic::doctor::{
    capability_snapshot, diagnostic_catalog, explain_diagnostic, CompatDoctor, DoctorProfile,
    Intent,
};

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("wasm4pm-compat: {error}");
            eprintln!();
            eprintln!("{}", help_text());
            ExitCode::from(64)
        }
    }
}

fn run(mut args: Vec<String>) -> Result<u8, String> {
    let json = remove_flag(&mut args, "--json");
    let (command, tail): (&str, &[String]) = match args.split_first() {
        Some((command, tail)) => (command.as_str(), tail),
        None => ("doctor", &[]),
    };

    match command {
        "doctor" => run_doctor(tail, json),
        "capabilities" | "capability" => run_capabilities(json),
        "plan" | "route" => run_plan(tail, json),
        "explain" | "diagnostic" => run_explain(tail, json),
        "diagnostics" | "catalog" => run_catalog(json),
        "profiles" => run_profiles(json),
        "version" | "--version" | "-V" => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
        "help" | "--help" | "-h" => {
            println!("{}", help_text());
            Ok(0)
        }
        unknown => Err(format!("unknown command `{unknown}`")),
    }
}

fn run_doctor(args: &[String], json: bool) -> Result<u8, String> {
    if args.len() > 1 {
        return Err("doctor accepts at most one profile".to_string());
    }
    let profile = match args.first() {
        Some(value) => DoctorProfile::parse(value)
            .ok_or_else(|| format!("unknown doctor profile `{value}`"))?,
        None => DoctorProfile::Boundary,
    };
    let report = CompatDoctor::run(profile);
    let fingerprint = report.fingerprint().map_err(|error| error.to_string())?;

    if json {
        #[derive(Serialize)]
        struct Envelope<'a> {
            fingerprint: &'a str,
            report: &'a wasm4pm_compat::diagnostic::doctor::DoctorReport,
        }
        print_json(&Envelope {
            fingerprint: &fingerprint,
            report: &report,
        })?;
    } else {
        print!("{}", report.render_text());
        println!("fingerprint: {fingerprint}");
    }

    Ok(report.exit_code())
}

fn run_capabilities(json: bool) -> Result<u8, String> {
    let capabilities = capability_snapshot();
    if json {
        print_json(&capabilities)?;
    } else {
        for capability in capabilities {
            println!(
                "[{:?}] {} owner={} — {}",
                capability.state, capability.code, capability.owner, capability.reason
            );
        }
    }
    Ok(0)
}

fn run_plan(args: &[String], json: bool) -> Result<u8, String> {
    if args.is_empty() {
        return Err("plan requires at least one intent".to_string());
    }
    let intents = args
        .iter()
        .map(|value| Intent::parse(value).ok_or_else(|| format!("unknown intent `{value}`")))
        .collect::<Result<Vec<_>, _>>()?;
    let plan = CompatDoctor::plan(intents);
    let fingerprint = plan.fingerprint().map_err(|error| error.to_string())?;

    if json {
        #[derive(Serialize)]
        struct Envelope<'a> {
            fingerprint: &'a str,
            plan: &'a wasm4pm_compat::diagnostic::doctor::RoutePlan,
        }
        print_json(&Envelope {
            fingerprint: &fingerprint,
            plan: &plan,
        })?;
    } else {
        print!("{}", plan.render_text());
        println!("fingerprint: {fingerprint}");
    }

    Ok(match plan.standing {
        wasm4pm_compat::diagnostic::doctor::DoctorStanding::PartialAlive => 0,
        wasm4pm_compat::diagnostic::doctor::DoctorStanding::Unknown => 1,
        wasm4pm_compat::diagnostic::doctor::DoctorStanding::Blocked => 2,
        wasm4pm_compat::diagnostic::doctor::DoctorStanding::BuildBroken => 3,
        wasm4pm_compat::diagnostic::doctor::DoctorStanding::Unsupported => 4,
    })
}

fn run_explain(args: &[String], json: bool) -> Result<u8, String> {
    if args.len() != 1 {
        return Err("explain requires exactly one diagnostic code or variant name".to_string());
    }
    let entry = explain_diagnostic(&args[0])
        .ok_or_else(|| format!("unknown diagnostic `{}`", args[0]))?;
    if json {
        print_json(&entry)?;
    } else {
        println!("{} ({})", entry.code, entry.name);
        println!("severity: {}", entry.severity);
        println!("message: {}", entry.message);
        println!("repair: {}", entry.repair);
    }
    Ok(0)
}

fn run_catalog(json: bool) -> Result<u8, String> {
    let catalog = diagnostic_catalog();
    if json {
        print_json(&catalog)?;
    } else {
        for entry in catalog {
            println!("{} {} — {}", entry.code, entry.name, entry.message);
        }
    }
    Ok(0)
}

fn run_profiles(json: bool) -> Result<u8, String> {
    if json {
        print_json(&DoctorProfile::ALL)?;
    } else {
        for profile in DoctorProfile::ALL {
            println!("{}", profile.as_str());
        }
    }
    Ok(0)
}

fn print_json(value: &impl Serialize) -> Result<(), String> {
    let output = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    println!("{output}");
    Ok(())
}

fn remove_flag(args: &mut Vec<String>, flag: &str) -> bool {
    let before = args.len();
    args.retain(|argument| argument != flag);
    args.len() != before
}

fn help_text() -> &'static str {
    "wasm4pm-compat — structure-only compatibility doctor\n\
\n\
USAGE:\n\
  wasm4pm-compat doctor [core|boundary|interop|graduation|vision2030] [--json]\n\
  wasm4pm-compat capabilities [--json]\n\
  wasm4pm-compat plan <intent>... [--json]\n\
  wasm4pm-compat explain <diagnostic-code|variant> [--json]\n\
  wasm4pm-compat diagnostics [--json]\n\
  wasm4pm-compat profiles [--json]\n\
\n\
INTENTS:\n\
  admit strict-admit import project export receipt diagnose graduate\n\
  discover conformance replay optimize verify-standing\n\
\n\
The CLI never executes process-mining algorithms and never awards ALIVE."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_command_is_doctor() {
        let code = run(Vec::new()).unwrap();
        assert!(matches!(code, 0 | 2));
    }

    #[test]
    fn remove_flag_is_order_independent() {
        let mut args = vec!["doctor".to_string(), "--json".to_string()];
        assert!(remove_flag(&mut args, "--json"));
        assert_eq!(args, vec!["doctor"]);
    }

    #[test]
    fn unknown_command_is_usage_error() {
        assert!(run(vec!["wat".to_string()]).is_err());
    }
}
