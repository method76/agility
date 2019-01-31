//! Utilities for collecting metrics.

use chrono::offset::Utc;

/// Adds given metric with given value.
///
/// Metric name should be in the following format: `module_name.metric_name`, where `module_name`
/// is a high level name. For example `storage` or `node` (not `storage_proof_list_index`).
///
/// Value is a string and can be formatted similar to the `println!`. See `std::fmt` and example
/// for details.
///
/// Metrics output direction is determined by the corresponding `metrics-...` feature. If
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate exonum;
/// # fn main() {
/// let val = 10;
/// metric!("mod_name.metric_name", val);
/// # }
/// ```
#[macro_export]
macro_rules! metric {
    ($name:expr, $value:expr) => {{
        $crate::helpers::metrics::add_metric($name, $value as i64);
    }};
}

// Do not use directly, use `metric!` macro instead.
#[doc(hidden)]
#[allow(unused_variables)]
pub fn add_metric(metric_name: &str, value: i64) {
    let time = format!("{:?}", Utc::now());

    #[cfg(feature = "metrics-log")]
    {
        trace!("{} {} {}", metric_name, value, time);
    }
}
