use crate::common::Binary;

mod common;

#[test]
fn test_init() {
    Binary::new().run(["init"]).expect("should succeed");
}
