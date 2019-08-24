use std::io::Read;
use std::time::Duration;

use dockworker::{
    container::ContainerFilters, ContainerCreateOptions, CreateExecOptions, CreateExecResponse,
    Docker, StartExecOptions,
};

pub fn RemoveAll(docker: &Docker, filter: ContainerFilters) {
    let containers = docker
        .list_containers(Some(true), None, None, filter)
        .unwrap();
    containers.iter().for_each(|c| {
        docker
            .remove_container(&c.Id, None, Some(true), None)
            .expect("RemoveAll Failed");
    });
}

pub fn Remove(docker: &Docker, name: &str) {
    docker
        .remove_container(name, None, Some(true), None)
        .expect("Remove Failed");
}

pub fn Restart(docker: &Docker, name: &str) {
    docker
        .restart_container(name, Duration::from_micros(0))
        .expect("Restart Failed");
}

pub fn AddNew(docker: &Docker, image: &str, name: &str) -> String {
    let mut create = ContainerCreateOptions::new(image);
    create.tty(true);
    create.open_stdin(true);
    let container = docker
        .create_container(Some(name), &create)
        .expect("Add new Failed");
    docker.start_container(&container.id).unwrap();
    RunCmd(docker, &container.id, "mkdir /code".to_string());
    return container.id;
}

pub fn RunCmd(docker: &Docker, id: &str, cmd: String) -> String {
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
    let mut res = docker
        .start_exec(&idx, &StartExecOptions::new().tty(true))
        .unwrap()
        .unwrap();
    let mut buf = Vec::new();
    res.read_to_end(&mut buf);
    return String::from_utf8(buf).unwrap();
}

pub fn CopyFiles(docker: &Docker, id: &str, code: &String, index: &u32) {
    println!("{}", code);
    println!(
        "{}",
        RunCmd(
            docker,
            id,
            format!("echo \"{}\" > /code/{}.cpp", code, index),
        )
    );
}
