use adder;

mod common;

#[test]
fn add_two_negative() {
    common::setup();
    assert_eq!(adder::add_two(-7), -5);
}
