use std::env;

#[test]
fn env_vars() {
    for (k, v) in env::vars() {
        println!("{}, {}", k, v);
    }
}

#[test]
fn env_var_get_recover() {
    for (k, v) in env::vars() {
        if k=="recover" {
            println!("{}", v);
        }
    }
}

#[test]
fn env_var_find_recover() {
    let a = env::vars()
        .find(|(k, v)| k=="recover")
        .map(|(k_recover, v_recover)| v_recover);
    
    println!("{:?}", a);
    
}