use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

use dockworker::Docker;

use super::SphinxCore::Env::*;
use super::Utils::DockerUtils;

#[derive(Eq, PartialEq)]
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
            let (code, info) = DockerUtils::RunCmd(
                docker,
                id,
                format!(
                    "timeout 3s g++ /code/{}/main.cpp -o /code/{}/o -O2 -Wall -std=c++17",
                    index, index
                ),
            );
            match code {
                0 => CompileResult {
                    status: CompileStatus::SUCCESS,
                    info,
                },
                _ => CompileResult {
                    status: CompileStatus::FAILED,
                    info,
                },
            }
        }
        Err(T) => CompileResult {
            status: CompileStatus::FAILED,
            info: T,
        },
    }
}
