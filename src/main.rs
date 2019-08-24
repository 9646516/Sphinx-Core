use dockworker::{container::ContainerFilters, Docker};
use std::fs::read_to_string;
use std::process::exit;
use std::string::String;

pub mod utils;

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    //    utils::DockerUtils::RemoveAll(&docker, filter);
    //    let id = utils::DockerUtils::AddNew(&docker, "gcc:7.3.0", "wtmsb");
    let id = "wtmsb";
    let path = "test/1.cpp";
    let cpp = read_to_string(path).unwrap();
    utils::DockerUtils::CopyFiles(&docker, id, &cpp, &114514);
}
