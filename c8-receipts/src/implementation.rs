/// Metadata capture for reproducibility of a Construct8 implementation.
///
/// An implementation receipt records exactly how a computation was performed:
/// compiler version, feature flags, optimization level, time of execution, etc.
///
/// This allows downstream tools to verify that the same code running under the
/// same conditions would produce the same receipts.
#[derive(Clone, Debug)]
pub struct ImplementationReceipt {
    /// Rust compiler version (e.g., "nightly-2025-06-01")
    pub rustc_version: String,
    /// Construct8 crate version
    pub c8_version: String,
    /// Enabled feature flags (e.g., ["market", "petri", "strict"])
    pub features: Vec<String>,
    /// Optimization level ("debug", "release", etc.)
    pub opt_level: String,
    /// Target triple (e.g., "x86_64-unknown-linux-gnu")
    pub target: String,
    /// Timestamp of implementation (nanoseconds since epoch)
    pub compiled_at_ns: u64,
    /// Total number of receipts manufactured in this implementation
    pub receipt_count: u64,
    /// Total nanoseconds spent in receipt generation
    pub total_ns: u64,
}

impl ImplementationReceipt {
    /// Constructs a new implementation receipt.
    pub fn new(
        rustc_version: String,
        c8_version: String,
        features: Vec<String>,
        opt_level: String,
        target: String,
        compiled_at_ns: u64,
    ) -> Self {
        ImplementationReceipt {
            rustc_version,
            c8_version,
            features,
            opt_level,
            target,
            compiled_at_ns,
            receipt_count: 0,
            total_ns: 0,
        }
    }

    /// Records a receipt generation event.
    pub fn record_receipt(&mut self, elapsed_ns: u64) {
        self.receipt_count += 1;
        self.total_ns += elapsed_ns;
    }

    /// Computes the average time per receipt in nanoseconds.
    pub fn avg_ns_per_receipt(&self) -> u64 {
        if self.receipt_count == 0 {
            0
        } else {
            self.total_ns / self.receipt_count
        }
    }

    /// Writes this implementation receipt to a JSON representation.
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "rustc_version": self.rustc_version,
            "c8_version": self.c8_version,
            "features": self.features,
            "opt_level": self.opt_level,
            "target": self.target,
            "compiled_at_ns": self.compiled_at_ns,
            "receipt_count": self.receipt_count,
            "total_ns": self.total_ns,
            "avg_ns_per_receipt": self.avg_ns_per_receipt(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implementation_receipt_new() {
        let receipt = ImplementationReceipt::new(
            "nightly-2025-06-01".to_string(),
            "26.6.9".to_string(),
            vec!["market".to_string()],
            "release".to_string(),
            "x86_64-unknown-linux-gnu".to_string(),
            1000,
        );

        assert_eq!(receipt.rustc_version, "nightly-2025-06-01");
        assert_eq!(receipt.c8_version, "26.6.9");
        assert_eq!(receipt.receipt_count, 0);
    }

    #[test]
    fn implementation_receipt_record() {
        let mut receipt = ImplementationReceipt::new(
            "nightly".to_string(),
            "26.6.9".to_string(),
            vec![],
            "release".to_string(),
            "x86_64".to_string(),
            0,
        );

        receipt.record_receipt(100);
        receipt.record_receipt(200);

        assert_eq!(receipt.receipt_count, 2);
        assert_eq!(receipt.total_ns, 300);
        assert_eq!(receipt.avg_ns_per_receipt(), 150);
    }

    #[test]
    fn implementation_receipt_avg_empty() {
        let receipt = ImplementationReceipt::new(
            "nightly".to_string(),
            "26.6.9".to_string(),
            vec![],
            "release".to_string(),
            "x86_64".to_string(),
            0,
        );

        assert_eq!(receipt.avg_ns_per_receipt(), 0);
    }

    #[test]
    fn implementation_receipt_to_json() {
        let mut receipt = ImplementationReceipt::new(
            "nightly".to_string(),
            "26.6.9".to_string(),
            vec!["market".to_string()],
            "release".to_string(),
            "x86_64".to_string(),
            1000,
        );
        receipt.record_receipt(50);

        let json = receipt.to_json();
        assert!(json.get("rustc_version").is_some());
        assert_eq!(json.get("receipt_count").unwrap(), 1);
    }
}
