#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

use wasm4pm_compat::xes::{XesEvent, XesExtensionNamespace};
use wasm4pm_compat::multiperspective::{ResourcePerspective, DataPerspective};

fn main() {
    let event = XesEvent::new()
        .with("org:resource", "Alice")
        .with("org:role", "Manager")
        .with("concept:name", "Shipment")
        .with("data:amount", "150.0")
        .with("custom_data:price", "200.0");

    // Resource Perspective Attributes
    assert!(event.has_extension::<ResourcePerspective>());
    let resource_attrs: Vec<_> = event.extension_attributes::<ResourcePerspective>().collect();
    assert_eq!(resource_attrs.len(), 2);
    assert_eq!(event.get_extension_attribute::<ResourcePerspective>("resource"), Some("Alice"));

    // Data Perspective Attributes (matches both data: and custom prefixes)
    assert!(event.has_extension::<DataPerspective>());
    let data_attrs: Vec<_> = event.extension_attributes::<DataPerspective>().collect();
    assert_eq!(data_attrs.len(), 2);
    assert_eq!(event.get_extension_attribute::<DataPerspective>("amount"), Some("150.0"));
}
