#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

use std::fs::read_to_string;

use dockworker::Docker;

pub mod SphinxCore;
pub mod Utils;

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let cpp = read_to_string("./test/cpp/a+b.c").unwrap();
    let idx = Utils::DockerUtils::GetContainers(&docker);
    let lang = SphinxCore::language::language::GNU;
    let res = SphinxCore::Compiler::Compiler(&docker, &idx[0], &cpp, &1u32, lang.clone());
    println!("{} {}", res.status, res.info);
    SphinxCore::Judge::Judge(&docker, &idx[0], &1u32, "a+b", lang.clone(), false);
}
