use internal_adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, internal_adder::add_two(2));
}

