use std::io::BufReader;

use dockworker::Docker;

use sphinx_core::{JudgeOutputDecoder, JudgeStatus, Language, StdJudgeOutputDecoder, Task};

use crate::utils::run_cmd;

pub struct Judge<'a> {
    pub(crate) docker: &'a Docker,
    // todo: create command
    pub(crate) container_id: &'a str,
}

impl<'a> sphinx_core::Judge for Judge<'a> {
    fn judge(&self, task: &Task, input: &str, output: &str, lang: Language, core: bool) -> (JudgeStatus, u64, u64) {

        // todo: move running command
        let run = lang.running_command("/tmp".to_string());
        let inputfile = format!("/data/{}/{}.in", task.input, input);

        // generate container command
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
        let (status, info) = run_cmd(self.docker, self.container_id, cmd);

        // decode output and return
        if status != 0 {
            (JudgeStatus::UnknownError, 0, 0)
        } else {
            let decoder = StdJudgeOutputDecoder::new();
            let cfg = decoder.decode(
                &mut BufReader::new(info.as_bytes())).unwrap();
            (cfg.status, cfg.time_cost, cfg.memory_cost)
        }
    }
}
