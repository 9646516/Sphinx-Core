use dockworker::Docker;
use std::path::Path;

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
    input: &str,
    output: &str,
    lang: language,
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
pub fn Judge(
    docker: &Docker,
    ContainerId: &str,
    uid: u64,
    JudgeOpt: &Config::Config,
    lang: language,
    base_url: &str,
) {
    let acm = JudgeOpt.judge_type == "acm";
    let is_interactive = JudgeOpt.spj == INTERACTIVE_JUDGE;
    let mut last: u32 = 0;
    let mut data_sum: u32 = 0;

    let mut task_id = 0;
    if acm {
        for i in JudgeOpt.tasks.iter() {
            task_id += 1;
            let input = get_data(&format!("{}/{}", base_url, i.input), "in");
            let output = if is_interactive {
                Vec::new()
            } else {
                get_data(&format!("{}/{}", base_url, i.output), "out")
            };
            data_sum += input.len() as u32;
            if !is_interactive && input != output {
                UpdateRealTimeInfo("DATA INVALID", 0, 0, uid, 0, 0, "input output dismatch");
                return;
            }
            for j in 0..input.len() {
                let (status, _t, _m) = Run(
                    docker,
                    ContainerId,
                    i,
                    &input[j],
                    if is_interactive { "" } else { &output[j] },
                    lang.clone(),
                    is_interactive,
                );
                if status == JudgeStatus::ACCEPTED {
                    UpdateRealTimeInfo(
                        if last == data_sum - 1 && JudgeOpt.tasks.len() == task_id {
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
                    return;
                }
            }
        }
    } else {
        let mut score = 0;
        let mut res = "ACCEPTED".to_owned();
        for i in JudgeOpt.tasks.iter() {
            task_id += 1;
            let input = get_data(&format!("{}/{}", base_url, i.input), "in");
            let output = if is_interactive {
                Vec::new()
            } else {
                get_data(&format!("{}/{}", base_url, i.output), "out")
            };
            data_sum += input.len() as u32;
            if !is_interactive && input != output {
                UpdateRealTimeInfo("DATA INVALID", 0, 0, uid, 0, 0, "input output dismatch");
                return;
            }
            for j in 0..input.len() {
                let (status, _t, _m) = Run(
                    docker,
                    ContainerId,
                    i,
                    &input[j],
                    if is_interactive { "" } else { &output[j] },
                    lang.clone(),
                    is_interactive,
                );
                if status == JudgeStatus::ACCEPTED {
                    score += i.score;
                } else {
                    res = status.to_string().to_owned();
                }
                UpdateRealTimeInfo(
                    if last == data_sum - 1 && task_id == JudgeOpt.tasks.len() {
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
