use std::path::Path;

use dockworker::Docker;

use super::DockerUtils;
use super::Language::language;
use super::SphinxCore::Env::*;
use super::Update::UpdateRealTimeInfo;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JudgeStatus {
    ACCEPTED,
    WRONG_ANSWER,
    TIME_LIMITED_ERROR,
    RUNTIME_ERROR,
    MEMORY_LIMITED_ERROR,
    OUTPUT_LIMITED_ERROR,
    COMPILE_ERROR,
    UNKNOWN_ERROR,
}

impl JudgeStatus {
    pub fn to_string(&self) -> &str {
        match self {
            JudgeStatus::ACCEPTED => "ACCEPTED",
            JudgeStatus::WRONG_ANSWER => "WRONG ANSWER",
            JudgeStatus::TIME_LIMITED_ERROR => "TIME LIMITED ERROR",
            JudgeStatus::RUNTIME_ERROR => "RUNTIME ERROR",
            JudgeStatus::MEMORY_LIMITED_ERROR => "MEMORY LIMITED ERROR",
            JudgeStatus::OUTPUT_LIMITED_ERROR => "OUTPUT LIMITED ERROR",
            JudgeStatus::COMPILE_ERROR => "COMPILE ERROR",
            JudgeStatus::UNKNOWN_ERROR => "UNKNOWN ERROR",
        }
    }
}

#[derive(Debug)]
pub struct JudgeResult {
    pub status: JudgeStatus,
    pub info: Option<String>,
    pub time_cost: u32,
    pub memory_cost: u32,
    pub last: u32,
}

#[derive(Debug)]
pub struct JudgeOption {
    pub time: u32,
    pub mem: u32,
    pub output: u32,
    pub stack: u32,
}

impl JudgeOption {
    pub fn new(time: u32, mem: u32) -> Self {
        Self {
            time,
            mem,
            output: 64_000_000,
            stack: 512_000_000,
        }
    }
    pub fn time(&mut self, time: u32) -> &mut Self {
        self.time = time;
        self
    }
    pub fn mem(&mut self, mem: u32) -> &mut Self {
        self.mem = mem;
        self
    }
    pub fn output(&mut self, output: u32) -> &mut Self {
        self.output = output;
        self
    }
    pub fn stack(&mut self, stack: u32) -> &mut Self {
        self.stack = stack;
        self
    }
}

pub fn Run(
    docker: &Docker,
    ContainerId: &str,
    SubmissionID: &u32,
    ProblemID: &str,
    prefix: &String,
    lang: language,
    opt: &JudgeOption,
    SpecialJudge: Option<&str>,
) -> (JudgeStatus, u32, u32) {
    let checker = {
        match SpecialJudge {
            Some(judge) => judge,
            None => "\"/code/Jury\"",
        }
    };
    let inputfile = format!("\"/data/{}/{}.in\"", ProblemID, prefix);
    let outputfile = format!("\"/data/{}/{}.out\"", ProblemID, prefix);
    let temp = format!("\"/code/{}/res\"", SubmissionID);
    let run = lang.running_command(format!("/code/{}", SubmissionID));
    let cmd = format!(
        "/code/core {} {} {} {} {} {} {} {} {}",
        opt.time, opt.mem, opt.output, opt.stack, inputfile, temp, outputfile, run, checker
    );
    let (status, info) = DockerUtils::RunCmd(docker, ContainerId, cmd);
    let res = json::parse(&info).unwrap();
    let time = res["time_cost"].as_u32().unwrap();
    let mem = res["memory_cost"].as_u32().unwrap();
    if status == 0 {
        (
            match res["result"].as_str().unwrap() {
                "Runtime Error" => JudgeStatus::RUNTIME_ERROR,
                "Time Limit Exceeded" => JudgeStatus::TIME_LIMITED_ERROR,
                "Memory Limit Exceeded" => JudgeStatus::MEMORY_LIMITED_ERROR,
                "Output Limit Exceeded" => JudgeStatus::OUTPUT_LIMITED_ERROR,
                "Accepted" => JudgeStatus::ACCEPTED,
                "Wrong Answer" => JudgeStatus::WRONG_ANSWER,
                _ => JudgeStatus::UNKNOWN_ERROR,
            },
            time,
            mem,
        )
    } else {
        (JudgeStatus::UNKNOWN_ERROR, time, mem)
    }
}

pub fn Judge(
    docker: &Docker,
    ContainerId: &str,
    SubmissionId: &u32,
    ProblemID: &str,
    lang: language,
    opt: &JudgeOption,
    SpecialJudge: &str,
) {
    let str = format!("{}{}", DATA_DIR, ProblemID);
    let path = Path::new(str.as_str());
    let mut test_case = Vec::new();
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let buf = entry.path();
            let prefix = buf.file_name().unwrap().to_str().unwrap();
            let suffix = buf.extension();
            if suffix.is_some() && suffix.unwrap().to_str().unwrap() == "in" {
                if entry.path().with_extension("out").exists() {
                    test_case.push(prefix.to_string().replace(".in", ""));
                }
            }
        }
    }
    let mut last = 0;
    for i in &test_case {
        let (status, _t, _m) = Run(
            docker,
            ContainerId,
            SubmissionId,
            &ProblemID,
            i,
            lang.clone(),
            opt,
            if !SpecialJudge.is_empty() {
                Some(SpecialJudge)
            } else {
                None
            },
        );
        if status == JudgeStatus::ACCEPTED {
            UpdateRealTimeInfo(
                if last == test_case.len() as u32 - 1 {
                    "ACCEPTED"
                } else {
                    "RUNNING"
                },
                &_m,
                &_t,
                SubmissionId,
                &last,
                "",
            );
            last += 1;
        } else {
            UpdateRealTimeInfo(status.to_string(), &_m, &_t, SubmissionId, &last, "");
            break;
        }
    }
}
