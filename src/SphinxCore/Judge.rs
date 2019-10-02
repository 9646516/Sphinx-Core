use dockworker::Docker;

use super::Config;
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
    ASSERT_FAILED,
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
            JudgeStatus::ASSERT_FAILED => "ASSERT FAILED",
            JudgeStatus::UNKNOWN_ERROR => "UNKNOWN ERROR",
        }
    }
}

pub fn Run(
    docker: &Docker,
    ContainerId: &str,
    task: &Config::Task,
    lang: language,
    core: bool,
) -> (JudgeStatus, u64, u64) {
    //generate command
    let run = lang.running_command("/tmp".to_string());
    let inputfile = format!("/data/{}", task.input);
    let cmd = if !core {
        let outputfile = format!("/data/{}", task.output);

        format!(
            "/tmp/core {} {} {} {} {} \"/tmp/res\" {} {} \"/tmp/judger\"",
            task.time, task.mem, 256_000_000, 256_000_000, inputfile, outputfile, run
        )
    } else {
        format!(
            "/tmp/core {} {} {} {} {} \"/tmp/res\" {} \"/tmp/judger\"",
            task.time, task.mem, 256_000_000, 256_000_000, inputfile, run
        )
    };
    //exec
    println!("{}", cmd);
    let (status, info) = DockerUtils::RunCmd(docker, ContainerId, cmd);
    println!("{} {}", status, info);
    let res = json::parse(&info).unwrap();
    if res["result"].as_str().unwrap() == "Judger Error" {
        return (JudgeStatus::UNKNOWN_ERROR, 0, 0);
    }
    let time = res["time_cost"].as_u64().unwrap();
    let mem = res["memory_cost"].as_u64().unwrap();
    if status == 0 {
        (
            match res["result"].as_str().unwrap() {
                "Runtime Error" => JudgeStatus::RUNTIME_ERROR,
                "Time Limit Exceeded" => JudgeStatus::TIME_LIMITED_ERROR,
                "Memory Limit Exceeded" => JudgeStatus::MEMORY_LIMITED_ERROR,
                "Output Limit Exceeded" => JudgeStatus::OUTPUT_LIMITED_ERROR,
                "Accepted" => JudgeStatus::ACCEPTED,
                "Wrong Answer" => JudgeStatus::WRONG_ANSWER,
                "Assert Failed" => JudgeStatus::ASSERT_FAILED,
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
    uid: u64,
    JudgeOpt: &Config::Config,
    lang: language,
) {
    let Task_Sum = JudgeOpt.tasks.len() as u32 - 1;
    let acm = JudgeOpt.judge_type == "acm";
    //use interactive core
    let core = JudgeOpt.spj == INTERACTIVE_JUDGE;
    let mut last: u32 = 0;
    if acm {
        for i in JudgeOpt.tasks.iter() {
            let (status, _t, _m) = Run(docker, ContainerId, i, lang.clone(), core);
            if status == JudgeStatus::ACCEPTED {
                UpdateRealTimeInfo(
                    if last == Task_Sum {
                        "ACCEPTED"
                    } else {
                        "RUNNING"
                    },
                    _m,
                    _t,
                    uid,
                    last,
                    0,
                    "",
                );
                last += 1;
            } else {
                UpdateRealTimeInfo(status.to_string(), _m, _t, uid, last, 0, "");
                break;
            }
        }
    } else {
        let mut score = 0;
        let mut res = "ACCEPTED".to_owned();
        for i in JudgeOpt.tasks.iter() {
            let (status, _t, _m) = Run(docker, ContainerId, i, lang.clone(), core);
            if status == JudgeStatus::ACCEPTED {
                score += i.score;
            } else {
                res = status.to_string().to_owned();
            }
            UpdateRealTimeInfo(
                if last == Task_Sum {
                    res.as_str()
                } else {
                    "RUNNING"
                },
                _m,
                _t,
                uid,
                last,
                score,
                "",
            );
            last += 1;
        }
    }
}
