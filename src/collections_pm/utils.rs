use std::env;

pub fn env_crash(k_crash: &str) {
    let v_crash = env::vars()
        .find(|(k, v)| k==k_crash)
        .map(|(k_crash, v_crash)| v_crash);

    if v_crash.is_some() && v_crash.unwrap()=="1" {
        eprintln!("[Intended Error] Process crashed");
        std::process::exit(0);
    } 
}