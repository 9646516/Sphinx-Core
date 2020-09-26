use std::string::String;

use dockworker::Docker;
use sphinx_core::{CompileResult, CompileStatus, Language, CompilerConfig};
use crate::utils::run_cmd;

pub struct Compiler<'a> {
    pub(crate) docker: &'a Docker,
    cfg: &'a CompilerConfig,
}

impl<'a> Compiler<'a> {
    pub fn new(docker: &Docker) -> Compiler {
        Compiler{
            docker,
            cfg: &CompilerConfig::default(),
        }
    }
}

impl<'a> sphinx_core::Compiler<'a> for Compiler<'a> {
    fn config(&mut self, cfg: &'a CompilerConfig) {
        self.cfg = cfg;
    }

    fn compile(&self, id: &str, source: String, lang: Language) -> CompileResult {
        let compile_command = format!("timeout 3s {}", lang.compile_command(source));

        let (code, info) = run_cmd(self.docker, id, compile_command, );
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
