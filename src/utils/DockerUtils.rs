use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use crossbeam::channel;
use crossbeam::crossbeam_channel;
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

pub fn AddNew(docker: &Docker, image: &str, name: &str) -> String {
    let mut create = ContainerCreateOptions::new(image);
    create.tty(true);
    create.open_stdin(true);
    let container = docker
        .create_container(Some(name), &create)
        .expect("Add new Failed");
    docker.start_container(&container.id).unwrap();
    RunCmd(&container.id, "mkdir /code".to_string(), 1000).expect("Create Fold Failed");
    return container.id;
}

pub fn RunCmd(id: &str, cmd: String, ttl: u64) -> Result<String, String> {
    let op = Instant::now();
    let mut done = false;
    let (sx, rx) = crossbeam_channel::bounded(1);
    let mut ret = String::new();
    crossbeam::thread::scope(|s| {
        s.spawn(|_| {
            let docker = Docker::connect_with_defaults().unwrap();
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
            ret = String::from_utf8(buf).unwrap();
            sx.send(()).unwrap();
        });

        s.spawn(|_| {
            while Instant::now().duration_since(op) < Duration::from_millis(ttl) {
                if rx.try_recv().is_ok() {
                    done = true;
                }
            }
            if !done {
                let docker = Docker::connect_with_defaults().unwrap();
                docker
                    .restart_container(id, Duration::from_micros(0))
                    .expect("Restart Failed");
            }
        });
    })
        .expect("Cmd Failed");
    match done {
        true => Ok(ret),
        false => Err("Time Limit Error".to_string()),
    }
}
