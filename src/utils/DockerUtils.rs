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
    containers.iter().foWr_each(|c| {
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

pub fn AddNew(docker: &Docker, image: &str, name: &str) -> String {
    let mut create = ContainerCreateOptions::new(image);
    create.open_stdin(true);
    let container = docker
        .create_container(Some(name), &create)
        .expect("Add new Failed");
    docker.start_container(&container.id);
    return container.id;
}

pub fn Test(docker: &Docker, id: &str, cmd: &str) -> String {
    let idx = docker
        .exec_container(id, &CreateExecOptions::new().cmd("ls".to_string()))
        .unwrap()
        .id;
    let mut res = docker
        .start_exec(&idx, &StartExecOptions::new())
        .unwrap()
        .unwrap();
    let mut buf = Vec::new();
    res.read_to_end(&mut buf);
    return String::from_utf8(buf).unwrap();
}
