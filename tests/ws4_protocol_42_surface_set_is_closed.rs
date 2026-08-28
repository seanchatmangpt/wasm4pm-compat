use std::collections::BTreeSet;
use wasm4pm_compat::prelude::*;
#[test]
fn protocol_surface_set_is_exactly_four_unique_surfaces(){let set:BTreeSet<_>=PROTOCOL_SURFACES.into_iter().collect();assert_eq!(set.len(),4);assert!(set.contains(&SurfaceKind::Cli));assert!(set.contains(&SurfaceKind::HttpApi));assert!(set.contains(&SurfaceKind::Mcp));assert!(set.contains(&SurfaceKind::A2a));}
