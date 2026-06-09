//! # Streaming Process Evidence
//!
//! Typed shapes for streaming event log evidence — online process monitoring.
//! A streaming evidence surface carries the same type law as batch evidence
//! but with an additional streaming-context marker.
//!
//! ## What this module IS
//!
//! - Structure-only typed markers for online vs. offline evidence collection.
//! - Zero-cost context tags that prevent an online monitoring window from being
//!   silently substituted for an offline analysis log at the type level.
//! - A companion to [`crate::evidence`]: streaming context is orthogonal to the
//!   `Raw → Admitted` lifecycle.
//!
//! ## What this module is NOT
//!
//! - **Not** a streaming runtime. No event ingestion, no window management, no
//!   sliding-window logic. Those concerns graduate to `wasm4pm`.
//! - **Not** a replacement for [`crate::evidence::Evidence`]. Use
//!   `ContextualEvidence<T, Context>` to add a collection-context tag to an
//!   evidence value; the lifecycle law is unchanged.
//!
//! ## Graduation
//!
//! When you need to actually ingest events from a stream, manage windows, or
//! perform online conformance checking, graduate to `wasm4pm`. The context
//! markers here travel with the evidence into the engine.

use core::marker::PhantomData;

/// Marker that this evidence is produced by a streaming source with a fixed
/// window size.
///
/// `WINDOW_SIZE` is a compile-time constant naming the maximum number of events
/// in a single evidence window. It is a structural constraint only — no buffer
/// is allocated here. Graduate to `wasm4pm` when windowed ingestion must run.
///
/// This is structure only. See [`crate::streaming`]. Graduate to `wasm4pm`
/// when streaming ingestion must be executed.
pub struct StreamingSource<const WINDOW_SIZE: usize>;

/// A streaming event evidence window of fixed size.
///
/// `SIZE` is a compile-time constant bounding the window. `T` is the element
/// type (e.g., an event shape). This is a structure-only envelope — no
/// elements are stored at this layer.
///
/// This is structure only. See [`crate::streaming`]. Graduate to `wasm4pm`
/// when window contents must be consumed by an online monitor.
pub struct EventWindow<T, const SIZE: usize> {
    pub buffer: [Option<T>; SIZE],
    pub head: usize,
    pub count: usize,
}

impl<T, const SIZE: usize> EventWindow<T, SIZE> {
    /// Construct a typed evidence window envelope.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::streaming::EventWindow;
    ///
    /// let _window: EventWindow<u8, 128> = EventWindow::new();
    /// ```
    pub fn new() -> Self {
        Self {
            buffer: std::array::from_fn(|_| None),
            head: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, event: T) -> Option<T> {
        let old = self.buffer[self.head].take();
        self.buffer[self.head] = Some(event);
        self.head = (self.head + 1) % SIZE;
        old
    }
}

impl<T, const SIZE: usize> Default for EventWindow<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

/// Online monitoring context — evidence produced in real-time from a live event
/// stream.
///
/// Use as the `Context` type parameter of [`ContextualEvidence`] when the
/// evidence was collected during active process execution.
///
/// This is structure only. See [`crate::streaming`]. Graduate to `wasm4pm`
/// when online monitoring logic must execute.
pub struct OnlineMonitoringContext;

/// Offline analysis context — evidence collected from a complete, static event
/// log.
///
/// Use as the `Context` type parameter of [`ContextualEvidence`] when the
/// evidence was collected from a finished log rather than a live stream.
///
/// This is structure only. See [`crate::streaming`]. Graduate to `wasm4pm`
/// when offline replay or conformance checking must execute.
pub struct OfflineAnalysisContext;

/// Evidence tagged with its collection context.
///
/// `Context` is one of [`OnlineMonitoringContext`] or [`OfflineAnalysisContext`]
/// (or a custom context type). The tag is zero-cost — `PhantomData` only.
///
/// An `ContextualEvidence<T, OnlineMonitoringContext>` is a different type from
/// `ContextualEvidence<T, OfflineAnalysisContext>`, so a function that demands
/// offline evidence cannot accidentally receive an online window.
///
/// ## Type aliases
///
/// See [`OnlineEvidence`] and [`OfflineEvidence`] for the common case.
///
/// This is structure only. See [`crate::streaming`]. Graduate to `wasm4pm`
/// when context-dependent processing must execute.
pub struct ContextualEvidence<T, Context> {
    /// The inner evidence value.
    pub inner: T,
    _ctx: PhantomData<Context>,
}

impl<T> ContextualEvidence<T, OfflineAnalysisContext> {
    /// Wrap `inner` as offline (complete-log) evidence.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::streaming::ContextualEvidence;
    ///
    /// let ev = ContextualEvidence::offline(42u32);
    /// assert_eq!(ev.inner, 42);
    /// ```
    pub fn offline(inner: T) -> Self {
        Self {
            inner,
            _ctx: PhantomData,
        }
    }
}

impl<T> ContextualEvidence<T, OnlineMonitoringContext> {
    /// Wrap `inner` as online (live-stream) evidence.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::streaming::ContextualEvidence;
    ///
    /// let ev = ContextualEvidence::online(42u32);
    /// assert_eq!(ev.inner, 42);
    /// ```
    pub fn online(inner: T) -> Self {
        Self {
            inner,
            _ctx: PhantomData,
        }
    }
}

/// Type alias for evidence collected in an online (live-stream) context.
///
/// Equivalent to `ContextualEvidence<T, OnlineMonitoringContext>`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::streaming::{OnlineEvidence, ContextualEvidence};
///
/// let ev: OnlineEvidence<u32> = ContextualEvidence::online(7u32);
/// assert_eq!(ev.inner, 7);
/// ```
pub type OnlineEvidence<T> = ContextualEvidence<T, OnlineMonitoringContext>;

/// Type alias for evidence collected in an offline (complete-log) context.
///
/// Equivalent to `ContextualEvidence<T, OfflineAnalysisContext>`.
///
/// # Examples
///
/// ```ignore
/// use wasm4pm_compat::streaming::{OfflineEvidence, ContextualEvidence};
///
/// let ev: OfflineEvidence<u32> = ContextualEvidence::offline(7u32);
/// assert_eq!(ev.inner, 7);
/// ```
pub type OfflineEvidence<T> = ContextualEvidence<T, OfflineAnalysisContext>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemporalOrderConfusion {
    pub current_timestamp: u64,
    pub offending_timestamp: u64,
}
