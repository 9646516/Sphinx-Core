use crate::Language;

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

pub struct CompilerConfig {}

pub const DEFAULT_CONFIG: CompilerConfig = CompilerConfig {};

impl CompilerConfig {
    pub fn default() -> &'static CompilerConfig {
        &DEFAULT_CONFIG
    }
}

pub trait Compiler<'a> {
    fn config(&mut self, cfg: &'a CompilerConfig);
    fn compile(&self, id: &str, source: String, lang: Language) -> CompileResult;
}

