use std::string::String;

use dockworker::Docker;

use super::Language::language;
use super::Utils::DockerUtils;

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

pub fn Compiler(docker: &Docker, id: &str, index: &u32, lang: language) -> CompileResult {
    let (code, info) = DockerUtils::RunCmd(
        docker,
        id,
        format!(
            "timeout 3s {}",
            lang.compile_command(format!("/code/{}", index))
        ),
    );
    println!("{} {}", code, info);
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
