use std::process::Command;

use errors::Error;
use pod::Pod;

pub mod app;
pub mod errors;
pub mod input;
pub mod pod;
pub mod ui;

pub fn load_all_pods(namespace: &str) -> Result<Vec<Pod>, Error> {
    let output = Command::new("/usr/local/bin/kubectl")
        .args(["get", "pods"])
        .args(["-n", namespace])
        .output()?;

    let parsed_output = String::from_utf8_lossy(&output.stdout);

    let pods: Vec<Pod> = parsed_output
        .lines()
        .map(|it| it.parse())
        .filter(|it| it.is_ok())
        .map(|it| it.unwrap())
        .collect();

    Ok(pods)
}

pub fn load_namespaces() {
    let output = Command::new("/usr/local/bin/kubectl")
        .args(["get", "namespaces"])
        .output()
        .expect("Couldn't load namespaces");

    println!("Namespaces:\n{}", String::from_utf8_lossy(&output.stdout));
}

pub fn load_logs(pod_name: &str, namespace: &str) -> Result<Vec<String>, Error> {
    let output = Command::new("/usr/local/bin/kubectl")
        .args(["logs", pod_name])
        .args(["-n", namespace])
        .output()?;

    let parsed_output = String::from_utf8_lossy(&output.stdout);

    let logs = parsed_output.lines().map(|it| it.to_owned()).collect(); //Avoid cloning?
    Ok(logs)
}
