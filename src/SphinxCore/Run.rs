use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use dockworker::Docker;

use super::tar::Builder;

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
    let dir_path = format!("{}/{}", WORK_DIR, uid);
    let pdir = Path::new(&dir_path);
    if !pdir.exists() && fs::create_dir_all(pdir).is_err() {
        return Err(format!("make dir failed"));
    }
    let code_path = format!("{}/{}/Main.{}", WORK_DIR, uid, lang.extension());
    let file = File::create(&code_path);
    if file.is_err() {
        return Err("make file failed".to_string());
    }
    match file.unwrap().write_all(Code.as_bytes()) {
        Ok(T) => {}
        Err(T) => return Err("write file failed".to_string()),
    };

    let TarPath = format!("{}/{}/foo.tar", WORK_DIR, uid);
    let file = File::create(&TarPath).unwrap();
    let mut a = Builder::new(file);

    a.append_file(
        format!("Main.{}", lang.extension()),
        &mut File::open(&code_path).unwrap(),
    )
    .unwrap();
    println!("{:?}", &code_path);

    a.append_file(
        "judger",
        &mut File::open(&format!("{}/{}", JUDGE_DIR, judger)).unwrap(),
    )
    .unwrap();

    if *JudgeType != 2 {
        a.append_file("core", &mut File::open(CORE1).unwrap())
            .unwrap();
    } else {
        a.append_file("core", &mut File::open(CORE2).unwrap())
            .unwrap();
    }

    docker
        .put_file(ContainerId, &Path::new(&TarPath), Path::new("/tmp"), true)
        .unwrap();
    Ok(())
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
    match CopyFiles(
        &docker,
        &SubmissionID,
        &ContainerId,
        &Code,
        &Judger,
        lang.clone(),
        &JudgeType,
    ) {
        Ok(T) => {
            if lang.compile() {
                let res = Compiler(&docker, &ContainerId, "/tmp".to_string(), lang.clone());
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
        .arg(format!("{}/{}:/data", DATA_DIR, ProblemID))
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
