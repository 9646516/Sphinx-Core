
use dockworker::Docker;
use std::path::Path;

use super::language::Language;
use super::core::env::*;
use crate::proto::{Task, ProblemConfig};
use crate::client::oj_server::kafka::update::update_real_time_info;
use crate::client::docker::run_cmd;

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

pub fn run(
    docker: &Docker,
    container_id: &str,
    task: &Task,
    input: &str,
    output: &str,
    lang: Language,
    core: bool,
) -> (JudgeStatus, u64, u64) {
    //generate command
    let run = lang.running_command("/tmp".to_string());
    let inputfile = format!("/data/{}/{}.in", task.input, input);
    let cmd = if !core {
        let outputfile = format!("/data/{}/{}.out", task.input, output);

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
    let (status, info) = run_cmd(docker, container_id, cmd);
    println!("{} {}", status, info);
    let res = json::parse(&info).unwrap();
    if res["result"].as_str().unwrap() == "Judger Error" {
        return (JudgeStatus::UnknownError, 0, 0);
    }
    let time = res["time_cost"].as_u64().unwrap();
    let mem = res["memory_cost"].as_u64().unwrap();
    if status == 0 {
        (
            match res["result"].as_str().unwrap() {
                "Runtime Error" => JudgeStatus::RuntimeError,
                "Time Limit Exceeded" => JudgeStatus::TimeLimitedError,
                "Memory Limit Exceeded" => JudgeStatus::MemoryLimitedError,
                "Output Limit Exceeded" => JudgeStatus::OutputLimitedError,
                "Accepted" => JudgeStatus::Accepted,
                "Wrong Answer" => JudgeStatus::WrongAnswer,
                "Assert Failed" => JudgeStatus::AssertFailed,
                _ => JudgeStatus::UnknownError,
            },
            time,
            mem,
        )
    } else {
        (JudgeStatus::UnknownError, time, mem)
    }
}

fn get_data(dir: &str, suf: &str) -> Vec<String> {
    println!("{}", dir);
    let path = Path::new(dir);
    let mut ret = Vec::new();
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let buf = entry.path();
            let prefix = buf.file_name().unwrap().to_str().unwrap();
            let suffix = buf.extension();
            if suffix.is_some() && suffix.unwrap().to_str().unwrap() == suf {
                ret.push(prefix.to_string().replace(&format!(".{}", suf), ""));
            }
        }
    }
    ret.sort();
    ret
}
pub fn judge(
    docker: &Docker,
    container_id: &str,
    uid: u64,
    judge_opt: &ProblemConfig,
    lang: Language,
    base_url: &str,
) {
    let acm = judge_opt.judge_type == "acm";
    let is_interactive = judge_opt.spj == INTERACTIVE_JUDGE;
    let mut last: u32 = 0;
    let mut data_sum: u32 = 0;

    let mut task_id = 0;
    if acm {
        for i in judge_opt.tasks.iter() {
            task_id += 1;
            let input = get_data(&format!("{}/{}", base_url, i.input), "in");
            let output = if is_interactive {
                Vec::new()
            } else {
                get_data(&format!("{}/{}", base_url, i.output), "out")
            };
            data_sum += input.len() as u32;
            if !is_interactive && input != output {
                update_real_time_info("DATA INVALID", 0, 0, uid, 0, 0, "input output dismatch");
                return;
            }
            for j in 0..input.len() {
                let (status, _t, _m) = run(
                    docker,
                    container_id,
                    i,
                    &input[j],
                    if is_interactive { "" } else { &output[j] },
                    lang.clone(),
                    is_interactive,
                );
                if status == JudgeStatus::Accepted {
                    update_real_time_info(
                        if last == data_sum - 1 && judge_opt.tasks.len() == task_id {
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
                    update_real_time_info(status.to_string(), _m, _t, uid, last, 0, "");
                    return;
                }
            }
        }
    } else {
        let mut score = 0;
        let mut res = "ACCEPTED".to_owned();
        for i in judge_opt.tasks.iter() {
            task_id += 1;
            let input = get_data(&format!("{}/{}", base_url, i.input), "in");
            let output = if is_interactive {
                Vec::new()
            } else {
                get_data(&format!("{}/{}", base_url, i.output), "out")
            };
            data_sum += input.len() as u32;
            if !is_interactive && input != output {
                update_real_time_info("DATA INVALID", 0, 0, uid, 0, 0, "input output dismatch");
                return;
            }
            for j in 0..input.len() {
                let (status, _t, _m) = run(
                    docker,
                    container_id,
                    i,
                    &input[j],
                    if is_interactive { "" } else { &output[j] },
                    lang.clone(),
                    is_interactive,
                );
                if status == JudgeStatus::Accepted {
                    score += i.score;
                } else {
                    res = status.to_string().to_owned();
                }
                update_real_time_info(
                    if last == data_sum - 1 && task_id == judge_opt.tasks.len() {
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
}
