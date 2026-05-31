// COMPILE-PASS: State transition markers — proves all transition marker types construct and are distinct

use wasm4pm_compat::state::{
    AdmittedToExportable, AdmittedToProjected, ParsedToAdmitted, ParsedToRefused, RawToParsed,
};

fn accepts_raw_to_parsed(_: RawToParsed) {}
fn accepts_parsed_to_admitted(_: ParsedToAdmitted) {}
fn accepts_parsed_to_refused(_: ParsedToRefused) {}
fn accepts_admitted_to_projected(_: AdmittedToProjected) {}
fn accepts_admitted_to_exportable(_: AdmittedToExportable) {}

fn main() {
    accepts_raw_to_parsed(RawToParsed);
    accepts_parsed_to_admitted(ParsedToAdmitted);
    accepts_parsed_to_refused(ParsedToRefused);
    accepts_admitted_to_projected(AdmittedToProjected);
    accepts_admitted_to_exportable(AdmittedToExportable);
}
