use std::io::Read;

use dockworker::{CreateExecOptions, Docker, StartExecOptions};
use std::process::Command;


pub(crate) fn init_docker(docker: &Docker, base_url: &str) -> String {
    let output = Command::new("docker")
        .arg("create")
        .arg("--interactive")
        .arg("-v")
        .arg(format!("{}:/data", base_url))
        .arg("--tty")
        .arg("--cpu-quota")
        .arg("100000")
        .arg("--cpu-period")
        .arg("100000")
        .arg("--network")
        .arg("none")
        .arg("judge:1.0.0")
        .output()
        .expect("create docker failed");
    let stdout = String::from_utf8_lossy(&output.stdout[0..output.stdout.len() - 1]);
    docker.start_container(&stdout.to_string()).unwrap();
    stdout.to_string()
}


pub(crate) fn run_cmd(docker: &Docker, id: &str, cmd: String) -> (u32, String) {
    let mut buf: Vec<u8> = Vec::new();
    let idx = docker
        .exec_container(
            id,
            &CreateExecOptions::new()
                .tty(true)
                .cmd("sh".to_string())
                .cmd("-c".to_string())
                .cmd(cmd),
        )
        .unwrap()
        .id;
    docker
        .start_exec(&idx, &StartExecOptions::new().tty(true))
        .unwrap()
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let status = docker.exec_inspect(&idx).unwrap().ExitCode.unwrap();
    let info = String::from_utf8(buf).unwrap();
    (status, info)
}
