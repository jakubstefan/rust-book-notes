use adder::add;

mod common;

/* No need to annotate it with `#[cfg(test)]`. Cargo treats the `tests` directory
specially and compiles files in this directory only when we run `cargo test` */
#[test]
fn it_adds_two_plus_two() {
    common::setup();
    let result = add(2, 2);
    assert_eq!(result, 4);
}
