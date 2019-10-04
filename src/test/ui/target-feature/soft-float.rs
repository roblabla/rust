// run-pass
// compile-flags: -C target-feature=+soft-float

#[cfg(target_feature = "soft-float")]
fn main() {}

#[cfg(not(target_feature = "soft-float"))]
fn main() {
    panic!("Can't cfg on soft-float!");
}
