use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use dockworker::Docker;

use super::tar;

use super::Compiler::{CompileStatus, Compiler};
use super::Env::*;
use super::Judge::Judge;
use super::Language::language;
use super::SphinxCore::Judge::JudgeOption;
use super::Update::UpdateRealTimeInfo;
pub fn CopyFiles(
    docker: &Docker,
    uid: &u32,
    ContainerId: &str,
    Code: &String,
    judger: &str,
    lang: language,
    JudgeType: &u8,
) -> Result<(), String> {
    let pdir = Path::new(&WORK_DIR);
    if !pdir.exists() && fs::create_dir_all(pdir).is_err() {
        return Err(format!("make dir failed"));
    }
    let code_path = format!("{}/{}/Main.{}", WORK_DIR, uid, lang.extension());
    let file = File::create(code_path);
    if file.is_err() {
        return Err("make file failed".to_string());
    }
    match file.unwrap().write_all(Code.as_bytes()) {
        Ok(T) => Ok(()),
        Err(T) => Err("write file failed".to_string()),
    }
    let file = File::create(format!("{}/{}/foo.tar", WORK_DIR, uid)).unwrap();
    let mut a = Builder::new(file);
    a.append_path(code_path).unwrap();
    a.append_path(format!("{}/{}", JUDGE_DIR, judger)).unwrap();
    if JudgeType != 2 {
        a.append_path(CORE1).unwrap();
    } else {
        a.append_path(CORE2).unwrap();
    }
    docker.put_file(ContainerId, file, "/tmp", true).unwrap();
}

pub fn Run(
    SubmissionID: u32,
    ProblemID: String,
    lang: language,
    Judger: String,
    JudgeOpt: JudgeOption,
    Code: String,
    JudgeType: u8,
) {
    let docker = Docker::connect_with_defaults().unwrap();
    let ContainerId = InitDocker(&docker, &ProblemID);
    println!("copying...");
    match CopyFiles(
        &docker,
        &uid,
        &ContainerId,
        &Code,
        &Judger,
        lang.clone(),
        &JudgeType,
    ) {
        Ok(T) => {
            if lang.compile() {
                let res = Compiler(
                    &docker,
                    &ContainerId,
                    format!("/tmp", SubmissionID),
                    lang.clone(),
                );
                if res.status == CompileStatus::FAILED {
                    UpdateRealTimeInfo("COMPILE ERROR", &0, &0, &SubmissionID, &0, &res.info);
                    return;
                }
            }
            println!("judging....");
            Judge(
                &docker,
                &ContainerId,
                &SubmissionID,
                &ProblemID,
                lang.clone(),
                &JudgeOpt,
                &Judger,
                &JudgeType,
            );
        }
        Err(T) => {
            UpdateRealTimeInfo("COMPILE ERROR", &0, &0, &SubmissionID, &0, &T);
        }
    }
    docker
        .remove_container(&ContainerId, Some(false), Some(true), Some(false))
        .unwrap();
}

fn InitDocker(docker: &Docker, ProblemID: &str) -> String {
    let output = Command::new("docker")
        .arg("create")
        .arg("--interactive")
        .arg("-v")
        .arg(format!("/home/rinne/data/{}:/data", SubmissionID))
        .arg("--tty")
        .arg("--cpu-quota")
        .arg("100000")
        .arg("--cpu-period")
        .arg("100000")
        .arg("--network")
        .arg("none")
        .arg("judge:1.0.0")
        .output()
        .expect("create docker failed");
    let stdout = String::from_utf8_lossy(&output.stdout[0..output.stdout.len() - 1]);
    println!("{}", stdout);
    docker.start_container(&stdout.to_string()).unwrap();
    println!("cmd ok");
    stdout.to_string()
}
