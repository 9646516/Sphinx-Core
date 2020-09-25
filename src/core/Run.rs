
extern crate tar;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use dockworker::Docker;

use tar::Builder;
use super::compiler::{CompileStatus, compiler};
use super::env::*;
use super::judge::judge;
use super::language::Language;
use crate::client::oj_server::kafka::update::update_real_time_info;
use crate::proto::ProblemConfig;

pub fn copy_files(
    docker: &Docker,
    container_id: &str,
    uid: u64,
    code: String,
    judge_opt: &ProblemConfig,
    lang: Language,
    base_url: &str,
) -> Result<(), String> {
    // Write code into Temp Dir
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
    match file.unwrap().write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(err) => return Err(format!("write file failed,{}", err)),
    };
    // Copy Jury , code and Core into Docker
    let tar_path = format!("{}/{}/foo.tar", WORK_DIR, uid);
    let file = File::create(&tar_path).unwrap();
    let mut a = Builder::new(file);

    a.append_file(
        format!("Main.{}", lang.extension()),
        &mut File::open(&code_path).unwrap(),
    )
    .unwrap();
    if judge_opt.spj == NORMAL_JUDGE {
        a.append_file("judger", &mut File::open(&JURY).unwrap())
            .unwrap();
    } else {
        a.append_file(
            "judger",
            &mut File::open(&format!("{}/{}", base_url, judge_opt.spj_path)).unwrap(),
        )
        .unwrap();
    }
    if judge_opt.spj != INTERACTIVE_JUDGE {
        a.append_file("core", &mut File::open(CORE1).unwrap())
            .unwrap();
    } else {
        a.append_file("core", &mut File::open(CORE2).unwrap())
            .unwrap();
    }

    docker
        .put_file(container_id, &Path::new(&tar_path), Path::new("/tmp"), true)
        .unwrap();
    Ok(())
}

pub fn run(
    submission_id: u64,
    lang: Language,
    judge_opt: ProblemConfig,
    code: String,
    base_url: &str,
) {
    let docker = Docker::connect_with_defaults().unwrap();
    let container_id = init_docker(&docker, base_url);
    match copy_files(
        &docker,
        &container_id,
        submission_id,
        code,
        &judge_opt,
        lang.clone(),
        base_url,
    ) {
        Ok(_) => {
            if lang.compile() {
                let res = compiler(&docker, &container_id, "/tmp".to_string(), lang.clone());
                if res.status == CompileStatus::FAILED {
                    update_real_time_info("COMPILE ERROR", 0, 0, submission_id, 0, 0, &res.info);
                    return;
                }
            }
            judge(
                &docker,
                &container_id,
                submission_id,
                &judge_opt,
                lang.clone(),
                base_url,
            );
        }
        Err(err) => {
            update_real_time_info("COMPILE ERROR", 0, 0, submission_id, 0, 0, &err);
        }
    }
    docker
        .remove_container(&container_id, Some(false), Some(true), Some(false))
        .unwrap();
}

fn init_docker(docker: &Docker, base_url: &str) -> String {
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
