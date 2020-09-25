use crate::{Task, Language};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    TimeLimitedError,
    RuntimeError,
    MemoryLimitedError,
    OutputLimitedError,
    CompileError,
    AssertFailed,
    UnknownError,
}

impl JudgeStatus {
    pub fn to_string(&self) -> &str {
        match self {
            JudgeStatus::Accepted => "ACCEPTED",
            JudgeStatus::WrongAnswer => "WRONG ANSWER",
            JudgeStatus::TimeLimitedError => "TIME LIMITED ERROR",
            JudgeStatus::RuntimeError => "RUNTIME ERROR",
            JudgeStatus::MemoryLimitedError => "MEMORY LIMITED ERROR",
            JudgeStatus::OutputLimitedError => "OUTPUT LIMITED ERROR",
            JudgeStatus::CompileError => "COMPILE ERROR",
            JudgeStatus::AssertFailed => "ASSERT FAILED",
            JudgeStatus::UnknownError => "UNKNOWN ERROR",
        }
    }
}

pub trait Judge {
    fn judge(&self, task: &Task, input: &str, output: &str, lang: Language, core: bool) -> (JudgeStatus, u64, u64);
}