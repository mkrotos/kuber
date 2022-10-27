use std::process::Command;

pub mod errors;
pub mod event;
pub mod menu;
pub mod ui;
pub mod pod;

pub fn load_all_pods(namespace: &str) {
    let output = Command::new("/usr/local/bin/kubectl")
        .args(["get", "pods"])
        .args(["-n", namespace])
        .output()
        .expect("Couldn't load pods");

    println!("Pods:\n{}", String::from_utf8_lossy(&output.stdout));
}

pub fn load_namespaces() {
    let output = Command::new("/usr/local/bin/kubectl")
        .args(["get", "namespaces"])
        .output()
        .expect("Couldn't load namespaces");

    println!("Namespaces:\n{}", String::from_utf8_lossy(&output.stdout));
}
