use std::cmp::max;
use std::path::Path;

use dockworker::Docker;
use json;

use super::DockerUtils;
use super::Language::language;
use super::SphinxCore::Env::*;

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
    RunID: &u32,
    DataID: &str,
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
    let inputfile = format!("\"/data/{}/{}.in\"", DataID, prefix);
    let outputfile = format!("\"/data/{}/{}.out\"", DataID, prefix);
    let temp = format!("\"/code/{}/res\"", RunID);
    let run = lang.running_command(format!("/code/{}", RunID));
    let cmd = format!(
        "/code/core {} {} {} {} {} {} {} {} {}",
        opt.time, opt.mem, opt.output, opt.stack, inputfile, temp, outputfile, run, checker
    );
    //    println!("{}", cmd);
    let (status, info) = DockerUtils::RunCmd(docker, ContainerId, cmd);
    //    println!("{} {}", status, info);
    let res = json::parse(&info).unwrap();
    let time = res["time_cost"].as_u32().unwrap();
    let mem = res["memory_cost"].as_u32().unwrap();
    //    println!("{} {}", time, mem);
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
    DataUID: &str,
    lang: language,
    opt: &JudgeOption,
    SpecialJudge: bool,
) -> JudgeResult {
    let str = format!("{}{}", DATA_DIR, DataUID);
    let path = Path::new(str.as_str());
    let mut test_case = Vec::new();
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let buf = entry.path();
            let prefix = buf.file_name().unwrap().to_str().unwrap();
            let suffix = buf.extension();
            if suffix.is_some() && suffix.unwrap().to_str().unwrap() == "in" {
                if entry.path().with_extension("out").exists(){
                    test_case.push(prefix.to_string().replace(".in", ""));
                }
            }
        }
    }
    let mut last = 0;
    if SpecialJudge {
        DockerUtils::RunCmd(
            docker,
            ContainerId,
            lang.compile_command(format!("/data/{}", DataUID)),
        );
    }
    let p = format!("\"/data/{}/o\"", DataUID);
    let mut time_cost = 0;
    let mut memory_cost = 0;
    for i in &test_case {
        let (status, _t, _m) = Run(
            docker,
            ContainerId,
            SubmissionId,
            &DataUID,
            i,
            lang.clone(),
            opt,
            if SpecialJudge { Some(&p) } else { None },
        );
        if status == JudgeStatus::ACCEPTED {
            time_cost = max(time_cost, _t);
            memory_cost = max(memory_cost, _m);
            last += 1;
        } else {
            return JudgeResult {
                status,
                info: None,
                time_cost,
                memory_cost,
                last,
            };
        }
    }
    JudgeResult {
        status: JudgeStatus::ACCEPTED,
        info: None,
        time_cost,
        memory_cost,
        last,
    }
}
