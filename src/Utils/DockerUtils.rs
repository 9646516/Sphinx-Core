use std::io::Read;

use dockworker::{container::ContainerFilters, CreateExecOptions, Docker, StartExecOptions};

pub fn GetContainers(docker: &Docker) -> Vec<String> {
    let filter = ContainerFilters::new();
    let containers = docker
        .list_containers(Some(true), None, None, filter)
        .unwrap();
    let mut ret = Vec::new();
    for i in &containers {
        ret.push(i.Id.clone());
    }
    return ret;
}

pub fn RunCmd(id: &str, cmd: String, ttl: u64) -> Result<String, String> {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let idx = docker
        .exec_container(
            id,
            &CreateExecOptions::new()
                .tty(true)
                .cmd("sh".to_string())
                .cmd("-c".to_string())
                .cmd(format!("timeout {} {}", ttl as f32 / 1000.0, cmd)),
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
    match status {
        0 => Ok(info),
        124 => Err("Command Exec too Much Time".to_string()),
        _ => Err(info),
    }
}
