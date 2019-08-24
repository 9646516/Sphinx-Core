use dockworker::{container::ContainerFilters, Docker};
use std::process::exit;
use std::string::String;

pub mod utils;

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    utils::DockerUtils::RemoveAll(&docker, filter);
    let id = utils::DockerUtils::AddNew(&docker, "alpine", "wtmsb");
    println!("{}", utils::DockerUtils::Test(&docker, &id, "ls"));
}
