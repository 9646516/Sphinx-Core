
use dockworker::Docker;
use sphinx_core::{Task, Language, JudgeStatus};
use crate::utils::run_cmd;

pub struct Judge<'a> {
    pub(crate) docker: &'a Docker,
    pub(crate) container_id: &'a str,
}

impl<'a> sphinx_core::Judge for Judge<'a> {
    fn judge(&self, task: &Task, input: &str, output: &str, lang: Language, core: bool) -> (JudgeStatus, u64, u64) {
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
        let (status, info) = run_cmd(self.docker, self.container_id, cmd);
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
}
