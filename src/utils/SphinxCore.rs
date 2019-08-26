use dockworker::{
    container::ContainerFilters, ContainerCreateOptions, CreateExecOptions, CreateExecResponse,
    Docker, StartExecOptions,
};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

pub enum CompileStatus {
    SUCCESS,
    FAILED,
}

impl std::fmt::Display for CompileStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileStatus::SUCCESS => write!(fmt, "SUCCESS"),
            CompileStatus::FAILED => write!(fmt, "FAILED"),
        }
    }
}

pub struct CompileResult {
    pub status: CompileStatus,
    pub info: String,
}

use super::DockerUtils;

const WORK_DIR: &str = "/home/rinne/code/";

pub fn CopyFiles(docker: &Docker, id: &str, code: &String, index: &u32) -> Result<(), String> {
    let dir = format!("{}/{}", WORK_DIR, index);
    let pdir = Path::new(&dir);
    if !pdir.exists() && fs::create_dir_all(pdir).is_err() {
        return Err(format!("make dir failed"));
    }
    let file = File::create(format!("{}/{}/main.cpp", WORK_DIR, index));
    if file.is_err() {
        return Err("make file failed".to_string());
    }
    match file.unwrap().write_all(code.as_bytes()) {
        Ok(T) => Ok(()),
        Err(T) => Err("write file failed".to_string()),
    }
}

pub fn Compiler(docker: &Docker, id: &str, code: &String, index: &u32) -> CompileResult {
    match CopyFiles(&docker, id, code, index) {
        Ok(T) => {
            let res = DockerUtils::RunCmd(
                id,
                format!(
                    "g++ /code/{}/main.cpp -o /code/{}/o -O2 -Wall -std=c++17",
                    index, index
                ),
                3000,
            );
            match res {
                Ok(T) => CompileResult {
                    status: CompileStatus::SUCCESS,
                    info: T,
                },
                Err(T) => CompileResult {
                    status: CompileStatus::FAILED,
                    info: T,
                },
            }
        }
        Err(T) => CompileResult {
            status: CompileStatus::FAILED,
            info: T,
        },
    }
}

pub enum JudgeStatus {
    ACCEPTED,
    WRONG_ANSWER,
    TIME_LIMITED_ERROR,
    RUNTIME_ERROR,
    MEMORY_LIMITED_ERROR,
    UNKNOWN_ERROR,
    COMPILE_ERROR,
}

impl std::fmt::Display for JudgeStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JudgeStatus::ACCEPTED => write!(fmt, "ACCEPTED"),
            JudgeStatus::WRONG_ANSWER => write!(fmt, "WRONG_ANSWER"),
            JudgeStatus::TIME_LIMITED_ERROR => write!(fmt, "TIME_LIMITED_ERROR"),
            JudgeStatus::RUNTIME_ERROR => write!(fmt, "RUNTIME_ERROR"),
            JudgeStatus::MEMORY_LIMITED_ERROR => write!(fmt, "MEMORY_LIMITED_ERROR"),
            JudgeStatus::UNKNOWN_ERROR => write!(fmt, "UNKNOWN_ERROR"),
            JudgeStatus::COMPILE_ERROR => write!(fmt, "COMPILE_ERROR"),
        }
    }
}

pub struct JudgeResult {
    pub status: JudgeStatus,
    pub info: String,
}

pub fn Judge(id: &str, index: &u32) -> JudgeResult {
    JudgeResult {
        status: JudgeStatus::ACCEPTED,
        info: String::new(),
    }
}
