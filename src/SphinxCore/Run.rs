use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use dockworker::Docker;

use super::Compiler::{CompileStatus, Compiler};
use super::Env::*;
use super::Judge::Judge;
use super::Language::language;
use super::SphinxCore::Judge::JudgeOption;
use super::Update::UpdateRealTimeInfo;

pub fn CopyFiles(
    docker: &Docker,
    id: &str,
    code: &String,
    index: &u32,
    lang: language,
) -> Result<(), String> {
    let dir = format!("{}/{}", WORK_DIR, index);
    let pdir = Path::new(&dir);
    if !pdir.exists() && fs::create_dir_all(pdir).is_err() {
        return Err(format!("make dir failed"));
    }
    let file = File::create(format!("{}/{}/Main.{}", WORK_DIR, index, lang.extension()));
    if file.is_err() {
        return Err("make file failed".to_string());
    }
    match file.unwrap().write_all(code.as_bytes()) {
        Ok(T) => Ok(()),
        Err(T) => Err("write file failed".to_string()),
    }
}

pub fn Run(
    uid: &u32,
    problem: &str,
    lang: language,
    SpecialJudge: bool,
    opt: &JudgeOption,
    code: &String,
) {
    let docker = Docker::connect_with_defaults().unwrap();
    let ContainerId = InitDocker();
    match CopyFiles(&docker, &ContainerId, code, uid, lang.clone()) {
        Ok(T) => {
            if lang.compile() {
                let res = Compiler(
                    &docker,
                    &ContainerId,
                    format!("/code/{}", uid),
                    lang.clone(),
                );
                if res.status == CompileStatus::FAILED {
                    UpdateRealTimeInfo("COMPILE ERROR", &0, &0, uid, &0, &res.info);
                    return;
                }
            }
            Judge(
                &docker,
                &ContainerId,
                uid,
                problem,
                lang.clone(),
                opt,
                SpecialJudge,
            );
        }
        Err(T) => {
            UpdateRealTimeInfo("COMPILE ERROR", &0, &0, uid, &0, &T);
        }
    }
    docker
        .remove_container(&ContainerId, Some(false), Some(true), Some(false))
        .unwrap();
}

fn InitDocker() -> String {
    let output = Command::new("docker")
        .arg("create")
        .arg("--interactive")
        .arg("-v")
        .arg("/home/rinne/code:/code")
        .arg("-v")
        .arg("/home/rinne/data:/data")
        .arg("--tty")
        .arg("--cpu-quota")
        .arg("100000")
        .arg("--cpu-period")
        .arg("100000")
        .arg("--network")
        .arg("none")
        .arg("9646516/judge_ubuntu:latest")
        .output()
        .expect("create docker failed");

    let stdout = String::from_utf8_lossy(&output.stdout[0..output.stdout.len() - 1]);
    println!("{:?}", stdout);
    stdout.to_string()
}
