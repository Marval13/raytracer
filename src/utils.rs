pub(crate) const EPSILON: f64 = 0.0001;

#[must_use]
pub(crate) fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
