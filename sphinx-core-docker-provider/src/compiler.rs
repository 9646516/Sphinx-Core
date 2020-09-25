use std::string::String;

use dockworker::Docker;
use sphinx_core::{CompileResult, CompileStatus, Language};
use crate::utils::run_cmd;

pub struct Compiler<'a> {
    pub(crate) docker: &'a Docker
}

impl<'a> sphinx_core::Compiler for Compiler<'a> {
    fn compile(&self, id: &str, source: String, lang: Language) -> CompileResult {
        let (code, info) = run_cmd(
            self.docker,
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
}
