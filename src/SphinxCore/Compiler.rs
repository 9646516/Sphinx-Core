use std::string::String;

use dockworker::Docker;

use super::Language::language;
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

pub fn Compiler(docker: &Docker, id: &str, index: &u32, lang: language) -> CompileResult {
    let (code, info) = DockerUtils::RunCmd(
        docker,
        id,
        format!(
            "timeout 3s {}",
            lang.compile_command(format!("/code/{}", index))
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
