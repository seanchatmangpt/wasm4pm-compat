#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Comparison result between two vector clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorClockCompare {
    /// This vector clock happens strictly before the other.
    Before,
    /// This vector clock happens strictly after the other.
    After,
    /// The two vector clocks are concurrent (neither happens before the other).
    Concurrent,
    /// The two vector clocks are equal.
    Equal,
}

/// An 8-lane vector clock for tracking causal relationships in market events.
///
/// A vector clock is a logical timestamp mechanism that captures partial ordering
/// relationships. Each lane represents a distinct causality source or participant
/// (e.g., instrument, venue, or actor). Values never decrease per lane; they only
/// tick forward or merge upward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorClock8 {
    /// Eight 32-bit lanes representing logical time on independent causal axes.
    lanes: [u32; 8],
}

impl VectorClock8 {
    /// Create a vector clock with all lanes at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::VectorClock8;
    /// let clock = VectorClock8::zero();
    /// assert_eq!(clock.lanes(), [0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn zero() -> Self {
        Self { lanes: [0; 8] }
    }

    /// Get the current state of all lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::VectorClock8;
    /// let clock = VectorClock8::zero();
    /// assert_eq!(clock.lanes(), [0, 0, 0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn lanes(&self) -> [u32; 8] {
        self.lanes
    }

    /// Increment the logical clock on a specific lane.
    ///
    /// # Arguments
    ///
    /// * `i` - Lane index (0..8)
    ///
    /// # Panics
    ///
    /// Panics if `i >= 8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::VectorClock8;
    /// let mut clock = VectorClock8::zero();
    /// clock.tick_lane(0);
    /// assert_eq!(clock.lanes()[0], 1);
    /// ```
    pub fn tick_lane(&mut self, i: usize) {
        assert!(i < 8, "lane index out of bounds");
        self.lanes[i] = self.lanes[i].wrapping_add(1);
    }

    /// Merge (take component-wise maximum) with another vector clock.
    ///
    /// After merging, this clock causally dominates both its prior state and the other clock.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::VectorClock8;
    /// let mut clock1 = VectorClock8::zero();
    /// clock1.tick_lane(0);
    /// clock1.tick_lane(1);
    ///
    /// let mut clock2 = VectorClock8::zero();
    /// clock2.tick_lane(1);
    /// clock2.tick_lane(2);
    ///
    /// clock1.merge(&clock2);
    /// assert_eq!(clock1.lanes(), [1, 1, 1, 0, 0, 0, 0, 0]);
    /// ```
    pub fn merge(&mut self, other: &VectorClock8) {
        for i in 0..8 {
            self.lanes[i] = self.lanes[i].max(other.lanes[i]);
        }
    }

    /// Compare this vector clock with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::{VectorClock8, VectorClockCompare};
    /// let mut clock1 = VectorClock8::zero();
    /// clock1.tick_lane(0);
    ///
    /// let mut clock2 = VectorClock8::zero();
    /// clock2.tick_lane(1);
    ///
    /// assert_eq!(clock1.compare(&clock2), VectorClockCompare::Concurrent);
    /// ```
    pub fn compare(&self, other: &VectorClock8) -> VectorClockCompare {
        let mut less_or_equal = true;
        let mut greater_or_equal = true;

        for i in 0..8 {
            if self.lanes[i] < other.lanes[i] {
                greater_or_equal = false;
            } else if self.lanes[i] > other.lanes[i] {
                less_or_equal = false;
            }
        }

        if less_or_equal && greater_or_equal {
            VectorClockCompare::Equal
        } else if less_or_equal {
            VectorClockCompare::Before
        } else if greater_or_equal {
            VectorClockCompare::After
        } else {
            VectorClockCompare::Concurrent
        }
    }
}

/// A monotonic timestamp in nanoseconds that never decreases.
///
/// Monotonic timestamps provide absolute ordering guarantees within a single system clock.
/// They form the basis of causal consistency checks alongside vector clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonotonicStamp {
    /// Nanoseconds since an arbitrary epoch (e.g., system boot or process start).
    nanos: u64,
}

impl MonotonicStamp {
    /// Create a monotonic stamp from an explicit nanosecond value.
    ///
    /// # Arguments
    ///
    /// * `nanos` - Nanosecond count
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::MonotonicStamp;
    /// let stamp = MonotonicStamp::from_nanos(1_000_000_000);
    /// assert_eq!(stamp.nanos(), 1_000_000_000);
    /// ```
    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    /// Get the nanosecond value.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::MonotonicStamp;
    /// let stamp = MonotonicStamp::from_nanos(500);
    /// assert_eq!(stamp.nanos(), 500);
    /// ```
    pub fn nanos(&self) -> u64 {
        self.nanos
    }

    /// Assert that this stamp does not regress relative to a previous stamp.
    ///
    /// # Arguments
    ///
    /// * `prev` - The previous monotonic stamp
    ///
    /// # Panics
    ///
    /// Panics if `self.nanos < prev.nanos()` (a regression).
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_time::MonotonicStamp;
    /// let stamp1 = MonotonicStamp::from_nanos(100);
    /// let stamp2 = MonotonicStamp::from_nanos(200);
    /// stamp2.assert_not_before(&stamp1); // OK
    /// ```
    pub fn assert_not_before(&self, prev: &MonotonicStamp) {
        assert!(
            self.nanos >= prev.nanos,
            "monotonic regression: {} < {}",
            self.nanos,
            prev.nanos
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_clocks_compare() {
        // Equal clocks
        let clock1 = VectorClock8::zero();
        let clock2 = VectorClock8::zero();
        assert_eq!(clock1.compare(&clock2), VectorClockCompare::Equal);

        // clock1 before clock2
        let mut clock1 = VectorClock8::zero();
        clock1.tick_lane(0);

        let mut clock2 = VectorClock8::zero();
        clock2.tick_lane(0);
        clock2.tick_lane(1);

        assert_eq!(clock1.compare(&clock2), VectorClockCompare::Before);
        assert_eq!(clock2.compare(&clock1), VectorClockCompare::After);

        // Concurrent clocks
        let mut clock1 = VectorClock8::zero();
        clock1.tick_lane(0);

        let mut clock2 = VectorClock8::zero();
        clock2.tick_lane(1);

        assert_eq!(clock1.compare(&clock2), VectorClockCompare::Concurrent);
    }

    #[test]
    fn vector_clock_merge() {
        let mut clock1 = VectorClock8::zero();
        clock1.tick_lane(0);
        clock1.tick_lane(1);
        // clock1 = [1, 1, 0, 0, 0, 0, 0, 0]

        let mut clock2 = VectorClock8::zero();
        clock2.tick_lane(1);
        clock2.tick_lane(2);
        // clock2 = [0, 1, 1, 0, 0, 0, 0, 0]

        clock1.merge(&clock2);
        // After merge: [1, 1, 1, 0, 0, 0, 0, 0]
        assert_eq!(clock1.lanes(), [1, 1, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn monotonic_never_regress() {
        let stamp1 = MonotonicStamp::from_nanos(100);
        let stamp2 = MonotonicStamp::from_nanos(200);
        let stamp3 = MonotonicStamp::from_nanos(200); // Equal is OK

        stamp2.assert_not_before(&stamp1);
        stamp3.assert_not_before(&stamp1);
        stamp3.assert_not_before(&stamp3);
    }

    #[test]
    #[should_panic(expected = "monotonic regression")]
    fn monotonic_regression_panics() {
        let stamp1 = MonotonicStamp::from_nanos(200);
        let stamp2 = MonotonicStamp::from_nanos(100);
        stamp2.assert_not_before(&stamp1);
    }

    #[test]
    fn vector_clock_tick_lane() {
        let mut clock = VectorClock8::zero();
        clock.tick_lane(0);
        assert_eq!(clock.lanes()[0], 1);
        assert_eq!(clock.lanes()[1], 0);

        clock.tick_lane(7);
        assert_eq!(clock.lanes()[7], 1);
    }

    #[test]
    #[should_panic(expected = "lane index out of bounds")]
    fn vector_clock_invalid_lane() {
        let mut clock = VectorClock8::zero();
        clock.tick_lane(8);
    }
}
