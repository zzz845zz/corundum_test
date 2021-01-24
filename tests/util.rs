use std::env;

#[test]
fn env_vars() {
    for (k, v) in env::vars() {
        println!("{}, {}", k, v);
    }
}