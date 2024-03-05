use adder;

mod common;

// run `cargo test -p adder -- --show-output` to see effect of calling `setup`
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
