
extern crate tar;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use dockworker::Docker;

use tar::Builder;
use super::env::*;
use crate::client::oj_server::kafka::update::update_real_time_info;
use sphinx_core::{ProblemConfig, Language, CompileStatus, Compiler, JudgeStatus, Judge};
use crate::utils::init_docker;

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
    let compiler = crate::Compiler { docker: &docker };

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
                let res = compiler.compile(&container_id, "/tmp".to_string(), lang.clone());
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
    let inner_judge = crate::Judge { docker: &docker, container_id: &container_id};

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
                let (status, _t, _m) = inner_judge.judge(
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
                let (status, _t, _m) = inner_judge.judge(
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
