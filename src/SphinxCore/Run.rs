use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use dockworker::Docker;

use super::Compiler::{Compiler, CompileStatus};
use super::Env::*;
use super::Judge::Judge;
use super::Language::language;
use super::SphinxCore::Config;
use super::tar::Builder;
use super::Update::UpdateRealTimeInfo;

pub fn gen_spj(source: &str, target: &str) {
    let output = Command::new("g++")
        .arg(source)
        .arg("-o")
        .arg(target)
        .arg("-std=c++17")
        .arg("-I")
        .arg(INCLUDE_PATH)
        .arg("-O2")
        .output()
        .expect(&format!("gen spj from {} to {} failed", source, target));
    let stdout = String::from_utf8_lossy(&output.stdout[0..output.stdout.len() - 1]);
    println!("{}", stdout.to_string());
}

pub fn CopyFiles(
    docker: &Docker,
    ContainerId: &str,
    uid: u64,
    Code: String,
    JudgeOpt: &Config::Config,
    lang: language,
    base_url: &str,
) -> Result<(), String> {
    // Write Code into Temp Dir
    let dir_path = format!("{}/{}", WORK_DIR, uid);
    let pdir = Path::new(&dir_path);
    if !pdir.exists() && fs::create_dir_all(pdir).is_err() {
        return Err("make dir failed".to_string());
    }
    let code_path = format!("{}/{}/Main.{}", WORK_DIR, uid, lang.extension());
    let file = File::create(&code_path);
    if file.is_err() {
        return Err("make file failed".to_string());
    }
    match file.unwrap().write_all(Code.as_bytes()) {
        Ok(_) => {}
        Err(T) => return Err(format!("write file failed,{}", T)),
    };
    // Copy Jury , Code and Core into Docker
    let TarPath = format!("{}/{}/foo.tar", WORK_DIR, uid);
    let file = File::create(&TarPath).unwrap();
    let mut a = Builder::new(file);

    a.append_file(
        format!("Main.{}", lang.extension()),
        &mut File::open(&code_path).unwrap(),
    )
        .unwrap();
    let jury_src = format!("{}/{}/jury", WORK_DIR, uid);
    if JudgeOpt.spj == NORMAL_JUDGE {
        gen_spj(&JURY, &jury_src);
    } else {
        gen_spj(&JURY, &format!("{}/{}", base_url, JudgeOpt.spj_path));
    }
    a.append_file(
        "judger",
        &mut File::open(&jury_src).unwrap(),
    )
        .unwrap();

    if JudgeOpt.spj != INTERACTIVE_JUDGE {
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
    SubmissionID: u64,
    lang: language,
    JudgeOpt: Config::Config,
    Code: String,
    base_url: &str,
) {
    let docker = Docker::connect_with_defaults().unwrap();
    let ContainerId = InitDocker(&docker, base_url);
    match CopyFiles(
        &docker,
        &ContainerId,
        SubmissionID,
        Code,
        &JudgeOpt,
        lang.clone(),
        base_url,
    ) {
        Ok(_) => {
            if lang.compile() {
                let res = Compiler(&docker, &ContainerId, "/tmp".to_string(), lang.clone());
                if res.status == CompileStatus::FAILED {
                    UpdateRealTimeInfo("COMPILE ERROR", 0, 0, SubmissionID, 0, 0, &res.info);
                    return;
                }
            }
            Judge(
                &docker,
                &ContainerId,
                SubmissionID,
                &JudgeOpt,
                lang.clone(),
                base_url,
            );
        }
        Err(T) => {
            UpdateRealTimeInfo("COMPILE ERROR", 0, 0, SubmissionID, 0, 0, &T);
        }
    }
    docker
        .remove_container(&ContainerId, Some(false), Some(true), Some(false))
        .unwrap();
}

fn InitDocker(docker: &Docker, base_url: &str) -> String {
    let output = Command::new("docker")
        .arg("create")
        .arg("--interactive")
        .arg("-v")
        .arg(format!("{}:/data", base_url))
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
    docker.start_container(&stdout.to_string()).unwrap();
    stdout.to_string()
}
