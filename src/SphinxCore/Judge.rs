use dockworker::Docker;
use std::path::Path;

const DATA_DIR: &str = "/home/rinne/data/";

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
    SubmissionId: &u32,
    InputDir: &Path,
    OutputDir: &Path,
) -> JudgeStatus {
    JudgeStatus::ACCEPTED
}

pub fn Judge(docker: &Docker, ContainerId: &str, SubmissionId: &u32, DataUID: &str) -> JudgeResult {
    let str = format!("{}{}", DATA_DIR, DataUID);
    let path = Path::new(str.as_str());
    let mut test_case = Vec::new();
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let dir = entry.path().to_str().unwrap().to_string();
            if dir.contains(".in") {
                let kano = dir.replace(".in", ".out");
                test_case.push(vec![dir, kano]);
            }
        }
    }
    let mut last = 0;
    for i in &test_case {
        print!("{} {}", i[0], i[1]);
        let status = Run(
            docker,
            ContainerId,
            SubmissionId,
            Path::new(&i[0]),
            Path::new(&i[1]),
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
