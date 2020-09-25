use std::string::String;

use dockworker::Docker;

use super::language::Language;

#[derive(Debug, Eq, PartialEq)]
pub enum CompileStatus {
    SUCCESS,
    FAILED,
}

#[derive(Debug)]
pub struct CompileResult {
    pub status: CompileStatus,
    pub info: String,
}

pub fn compiler(docker: &Docker, id: &str, source: String, lang: Language) -> CompileResult {
    let (code, info) = crate::client::docker::run_cmd(
        docker,
        id,
        format!("timeout 3s {}", lang.compile_command(source)),
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
