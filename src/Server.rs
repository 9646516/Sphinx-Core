use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

use crossbeam;
use dockworker::Docker;
use json;
use json::JsonValue;

use crate::SphinxCore::Judge::{JudgeOption, JudgeStatus};
use crate::SphinxCore::Language::language;

use super::JudgeResult;
use super::SphinxCore;
use super::SphinxCore::Env::*;

pub struct Server {
    pub mp: RwLock<HashMap<u32, JudgeResult>>,
    Q: crossbeam::queue::ArrayQueue<String>,
}

impl Server {
    pub fn new() -> Self {
        let id: Vec<String> = Vec::new();
        let Q = crossbeam::queue::ArrayQueue::new(DOCKER_SUM);
        for i in id {
            Q.push(i).unwrap();
        }
        Self {
            mp: RwLock::new(HashMap::new()),
            Q,
        }
    }

    fn solve_run(&self, src: SocketAddr, info: JsonValue) {
        //        let id = info["id"].as_u32().unwrap();
        //        let problem = info["problem"].to_string();
        //        let sol = info["sol"].to_string();
        //        let spj = info["spj"].as_bool().unwrap();
        //        let opt = JudgeOption::new(
        //            info["time"].as_u32().unwrap(),
        //            info["mem"].as_u32().unwrap(),
        //        );
        //        let docker = Docker::connect_with_defaults().unwrap();
        //        let lang = match info["lang"].as_str().unwrap() {
        //            "GCC" => language::GCC,
        //            "GNU" => language::GNU,
        //            "CLANG" => language::CLANG,
        //            "CLANGPP" => language::CLANGPP,
        //            "JAVA" => language::JAVA,
        //            "RUST" => language::RUST,
        //            "PY2" => language::PY2,
        //            "PY3" => language::PY3,
        //            _ => panic!("lang err"),
        //        };
        //        loop {
        //            let jb = self.Q.pop();
        //            match jb {
        //                Err(T) => {
        //                    sleep(Duration::from_millis(500));
        //                }
        //                Ok(idx) => {
        //                    SphinxCore::Run::Run(&docker, &idx, &id, &problem, lang, spj, &opt, &sol);
        //                    self.Q.push(idx).unwrap();
        //                    break;
        //                }
        //            }
        //        }
    }

    pub fn run(&self) {}

    pub fn push(&self, lang: String, solution: String, time: u32, mem: u32, uid: u32) -> bool {
        true
    }
    pub fn get(&self, uid: u32) -> Option<JudgeResult> {
        None
    }
}
