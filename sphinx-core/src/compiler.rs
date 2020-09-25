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

pub trait Compiler {
    fn compile(&self, id: &str, source: String, lang: Language) -> CompileResult;
}

