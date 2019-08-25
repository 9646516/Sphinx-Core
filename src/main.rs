#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::fs::read_to_string;
use std::process::exit;
use std::string::String;

use dockworker::{container::ContainerFilters, Docker};

pub mod utils;

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    //    utils::DockerUtils::RemoveAll(&docker, filter);
    //    let id = utils::DockerUtils::AddNew(&docker, "gcc:7.3.0", "wtmsb");
    let id = "wtmsb";
    let path = "test/1.cpp";
    let cpp = read_to_string(path).unwrap();
    //    utils::DockerUtils::CopyFiles(&docker, id, &cpp, &114514);
    let res = utils::SphinxCore::Compiler(&docker,id,&cpp,&1u32);
    println!("{} {}", res.info,res.status);
    println!("DONE");
}
