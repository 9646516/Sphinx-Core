#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::fs::read_to_string;
use std::process::exit;
use std::string::String;

use dockworker::{container::ContainerFilters, Docker};

pub mod utils;

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let cpp = read_to_string("./test/1.cpp").unwrap();
    let idx=utils::DockerUtils::GetContainers(&docker);
    let res = utils::SphinxCore::Compiler(&docker,&idx[0],&cpp,&1u32);
    println!("{} {}", res.status,res.info);
    println!("done");
    println!("{}",utils::DockerUtils::RunCmd(&idx[0],"apt".to_string(),1000u64).unwrap());
}
