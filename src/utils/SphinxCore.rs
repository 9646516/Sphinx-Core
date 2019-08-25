use dockworker::{
    container::ContainerFilters, ContainerCreateOptions, CreateExecOptions, CreateExecResponse,
    Docker, StartExecOptions,
};
use std::string::String;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

pub enum CompileStatus {
    SUCCESS,
    FAILED,
    TLE
}
impl std::fmt::Display for CompileStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileStatus::SUCCESS => write!(fmt, "SUCCESS"),
            CompileStatus::FAILED => write!(fmt, "FAILED"),
            CompileStatus::TLE => write!(fmt, "TLE"),
        }
    }
}

pub struct CompileResult{
    pub status:CompileStatus,
    pub info:String
}
use super::DockerUtils;
const WORK_DIR:&str = "/home/rinne/code/";
pub fn CopyFiles(docker: &Docker, id: &str, code: &String, index: &u32) {
    let dir=format!("{}/{}",WORK_DIR,index);
    let pdir=Path::new(&dir);
    if !pdir.exists(){
        fs::create_dir(pdir).expect("create dir failed");
    }
    let mut file = File::create(format!("{}/{}/main.cpp",WORK_DIR,index)).expect("create file failed");
    file.write_all(code.as_bytes()).expect("copy failed");
}

pub fn Compiler(docker: &Docker, id: &str, code: &String, index: &u32) ->CompileResult{
    CopyFiles(&docker, id, code, index);
    let res=DockerUtils::RunCmd(id,format!("g++ /code/{}/main.cpp -o /code/{}/o -O2 -Wall -std=c++17",index,index),3000);
    match res {
        Ok(T)=>{
            if !T.contains("error") {
                CompileResult{
                    status:CompileStatus::SUCCESS,
                    info:T
                }
            }else {
                CompileResult{
                    status:CompileStatus::FAILED,
                    info:T
                }
            }
        },
        Err(T)=>{
            CompileResult{
                status:CompileStatus::TLE,
                info:String::new()
            }
        }
    }
}
