use error::Error;
use std::{error, fmt};
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::{Language, Task};

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

#[derive(Debug, Clone)]
pub struct JudgeOutputError;

impl Display for JudgeOutputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("judge error")
    }
}

impl Error for JudgeOutputError {}

pub struct JudgeOutput {
    pub status: JudgeStatus,
    pub memory_cost: u64,
    pub time_cost: u64,
}

pub struct JudgeReply<'a> {
    pub submission_id: u64,
    pub last: u32,
    pub status: &'a str,
    pub mem: u64,
    pub time: u64,
    pub score: u64,
    pub info: &'a str,
}

pub trait JudgeOutputDecoder {
    fn decode<T: Read>(&self, reader: &mut T) -> Result<JudgeOutput, JudgeOutputError>;
}

pub struct StdJudgeOutputDecoder {}

impl StdJudgeOutputDecoder {
    pub fn new() -> StdJudgeOutputDecoder {
        StdJudgeOutputDecoder {}
    }
}

impl JudgeOutputDecoder for StdJudgeOutputDecoder {
    fn decode<T: Read>(&self, _reader: &mut T) -> Result<JudgeOutput, JudgeOutputError> {
        let mut buffer = Vec::new();
        _reader.read_to_end(&mut buffer).unwrap();
        let sb = String::from_utf8(buffer).unwrap();
        let res = json::parse(&sb).unwrap();
        if res["result"].as_str().unwrap() == "Judger Error" {
            return Ok(JudgeOutput {
                status: JudgeStatus::UnknownError,
                memory_cost: 0,
                time_cost: 0,
            });
        }
        let time_cost = res["time_cost"].as_u64().unwrap();
        let memory_cost = res["memory_cost"].as_u64().unwrap();
        return Ok(JudgeOutput {
            status: match res["result"].as_str().unwrap() {
                "Runtime Error" => JudgeStatus::RuntimeError,
                "Time Limit Exceeded" => JudgeStatus::TimeLimitedError,
                "Memory Limit Exceeded" => JudgeStatus::MemoryLimitedError,
                "Output Limit Exceeded" => JudgeStatus::OutputLimitedError,
                "Accepted" => JudgeStatus::Accepted,
                "Wrong Answer" => JudgeStatus::WrongAnswer,
                "Assert Failed" => JudgeStatus::AssertFailed,
                _ => JudgeStatus::UnknownError,
            },
            time_cost,
            memory_cost,
        });
    }
}

pub trait Judge {
    fn judge(&self, task: &Task, input: &str, output: &str, lang: Language, core: bool) -> (JudgeStatus, u64, u64);
}