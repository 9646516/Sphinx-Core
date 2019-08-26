use super::SphinxCore::Env::*;
use super::Utils::DockerUtils;
use crate::Utils::DockerUtils::RunCmd;
use dockworker::Docker;
use std::path::Path;

#[derive(Eq, PartialEq)]
pub enum JudgeStatus {
    ACCEPTED,
    WRONG_ANSWER,
    TIME_LIMITED_ERROR,
    RUNTIME_ERROR,
    MEMORY_LIMITED_ERROR,
    UNKNOWN_ERROR,
    COMPILE_ERROR,
}

impl std::fmt::Display for JudgeStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JudgeStatus::ACCEPTED => write!(fmt, "ACCEPTED"),
            JudgeStatus::WRONG_ANSWER => write!(fmt, "WRONG_ANSWER"),
            JudgeStatus::TIME_LIMITED_ERROR => write!(fmt, "TIME_LIMITED_ERROR"),
            JudgeStatus::RUNTIME_ERROR => write!(fmt, "RUNTIME_ERROR"),
            JudgeStatus::MEMORY_LIMITED_ERROR => write!(fmt, "MEMORY_LIMITED_ERROR"),
            JudgeStatus::UNKNOWN_ERROR => write!(fmt, "UNKNOWN_ERROR"),
            JudgeStatus::COMPILE_ERROR => write!(fmt, "COMPILE_ERROR"),
        }
    }
}

pub struct JudgeResult {
    pub status: JudgeStatus,
    pub last: u32,
}

pub fn Run(
    docker: &Docker,
    ContainerId: &str,
    RunID: &u32,
    DataID: &str,
    prefix: &String,
    SpecialJudge: Option<&str>,
) -> JudgeStatus {
    let checker = {
        match SpecialJudge {
            Some(judge) => judge,
            None => "\"/code/Jury\"",
        }
    };
    let inputfile = format!("\"/data/{}/{}.in\"", DataID, prefix);
    let outputfile = format!("\"/data/{}/{}.out\"", DataID, prefix);
    let temp = format!("\"/code/{}/res\"", RunID);
    let run = format!("\"/code/{}/o\"", RunID);
    let cmd = format!(
        "/code/core {} {} {} {} {} {} {} {} {}",
        1000, 256_000_000, 64_000_000, 512_000_000, inputfile, temp, outputfile, run, checker
    );
    println!("{}", cmd);
    let (status, info) = RunCmd(docker, ContainerId, cmd);
    println!("{} {}", status, info);
    JudgeStatus::ACCEPTED
}

pub fn Judge(
    docker: &Docker,
    ContainerId: &str,
    SubmissionId: &u32,
    DataUID: &str,
    SpecialJudge: bool,
) -> JudgeResult {
    let str = format!("{}{}", DATA_DIR, DataUID);
    let path = Path::new(str.as_str());
    let mut test_case = Vec::new();
    println!("{:?}", path);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let buf = entry.path();
            println!("{:?}", buf);
            let prefix = buf.file_name().unwrap().to_str().unwrap();
            let suffix = buf.extension();
            if suffix.is_some() && suffix.unwrap().to_str().unwrap() == "in" {
                test_case.push(prefix.to_string().replace(".in", ""));
            }
        }
    }
    println!("OUT");

    let mut last = 0;
    if SpecialJudge {
        DockerUtils::RunCmd(
            docker,
            ContainerId,
            format!("g++ /data/{}/judge.cpp -o /data/{}/o -O2", DataUID, DataUID),
        );
    }
    let p = format!("\"/data/{}/o\"", DataUID);
    for i in &test_case {
        println!("{}", i);
        let status = Run(
            docker,
            ContainerId,
            SubmissionId,
            &DataUID,
            i,
            if SpecialJudge { Some(&p) } else { None },
        );
        if status != JudgeStatus::ACCEPTED {
            return JudgeResult { status, last };
        } else {
            last += 1;
        }
    }
    JudgeResult {
        status: JudgeStatus::ACCEPTED,
        last,
    }
}
