//! Declarative ergonomics for host-defined structure-only connectors.
//!
//! These macros manufacture marker types and immutable connector catalogs only.
//! They do not manufacture execution authority or perform transport.

/// Declare a zero-sized connector marker implementing
/// [`crate::import::connectors::Connector`].
///
/// The expansion contains no I/O and adds no dependencies.
#[macro_export]
macro_rules! compat_connector {
    (
        $vis:vis $name:ident,
        $id:expr,
        $format:expr,
        $direction:expr,
        $transport:expr,
        $media_type:expr,
        $extension:expr $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        $vis struct $name;

        impl $crate::import::connectors::Connector for $name {
            const SPEC: $crate::import::connectors::ConnectorSpec =
                $crate::import::connectors::ConnectorSpec::new(
                    $id,
                    $format,
                    $direction,
                    $transport,
                    $media_type,
                    $extension,
                );
        }
    };
}

/// Build an immutable catalog from connector marker types declared with
/// [`compat_connector!`](crate::compat_connector).
///
/// The empty catalog is lawful: it is the zero-cardinality member of the catalog
/// power set and manufactures no connector authority.
#[macro_export]
macro_rules! compat_connector_set {
    ($vis:vis const $name:ident = [$($connector:ty),* $(,)?] $(;)?) => {
        $vis const $name: &[$crate::import::connectors::ConnectorSpec] = &[
            $(
                <$connector as $crate::import::connectors::Connector>::SPEC
            ),*
        ];
    };
}
