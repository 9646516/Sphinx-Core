use shiplift::Docker;
use shiplift::builder::ContainerListOptionsBuilder;
use tokio::prelude::Future;
use std::collections::HashMap;
use std::string::String;

fn main() {
    let docker = Docker::new();
    let mut builder = ContainerListOptionsBuilder::default();
    let fut = docker
        .containers()
        .list(&builder.all().build())
        .map(|containers| {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}
