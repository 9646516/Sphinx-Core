#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use std::fs::read_to_string;

use dockworker::Docker;

pub mod SphinxCore;
pub mod Utils;

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let cpp = read_to_string("./test/a+b/Main.java").unwrap();
    let idx = Utils::DockerUtils::GetContainers(&docker);
    let lang = SphinxCore::Language::language::JAVA;
    let jb = SphinxCore::Run::Run(&docker, &idx[0], &1, "a+b", lang, false, &cpp);
    println!("{:?}", jb);
}
