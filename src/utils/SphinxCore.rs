use dockworker::{
    container::ContainerFilters, ContainerCreateOptions, CreateExecOptions, CreateExecResponse,
    Docker, StartExecOptions,
};
use std::string::String;
use std::path::Path;

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

pub fn CopyFiles(docker: &Docker, id: &str, code: &String, index: &u32) {
    DockerUtils::RunCmd(id, format!("mkdir  /code/{}", index), 1000)
        .expect("Create Code Fold  Failed");
    DockerUtils::RunCmd(id, format!("echo -e \"{}\" > /code/{}/main.cpp", code, index), 1000)
        .expect("Copy Code File Failed");
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
