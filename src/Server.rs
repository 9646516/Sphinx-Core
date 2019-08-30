use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

use crossbeam;
use dockworker::Docker;
use json;
use json::JsonValue;

use crate::SphinxCore::Judge::JudgeOption;
use crate::SphinxCore::Language::language;

use super::SphinxCore;
use super::SphinxCore::Env::*;

pub struct ServeResult {
    pub status: bool,
    pub info: String,
}

impl ServeResult {
    pub fn FAILED(info: String) -> ServeResult {
        ServeResult {
            status: false,
            info,
        }
    }

    pub fn OK(info: String) -> ServeResult {
        ServeResult { status: true, info }
    }

    pub fn parse(&self) -> String {
        let mut data = json::JsonValue::new_object();
        data["status"] = (if self.status { "OK" } else { "FAILED" }).into();
        data["status"] = self.info.clone().into();
        data.dump()
    }
}

pub struct Server {
    sx: UdpSocket,
    rx: UdpSocket,
    Q: crossbeam::queue::ArrayQueue<String>,
}

impl Server {
    pub fn new(id: Vec<String>) -> Self {
        let Q = crossbeam::queue::ArrayQueue::new(DOCKER_SUM);
        for i in id {
            Q.push(i).unwrap();
        }
        Self {
            rx: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, RX_PORT))
                .expect("rx UDP Bind failed"),
            sx: UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, SX_PORT))
                .expect("sx UDP Bind failed"),
            Q,
        }
    }

    /// {"Method": str,"data":[{...}]}
    /// Method =>one of ["run","upload judge","upload data"]
    /// lang => one of ["GCC","GNU","CLANG","CLANGPP","JAVA","PY2","PY3","RUST"]
    /// solution size should be no more than 65536 characters
    /// {"Method":"run","Data":[{"spj":false,"time":1000,"mem":256_000_000,"id":1,problem:"a+b","lang":"PY3“,"sol":"print(sum(list(map(int,input().split()))))"}]}
    /// {"Method":"upload judge","Data":[{"id":1,"sol":"...."}]}
    /// {"Method":"upload data","Data":[{"id":1,"data":[ ["1.in","1 2"],["2.in","1 2"] ]}]}

    /// response {"status":"OK",info:[{...}]}
    /// status => one of ["OK","FAILED","RUNNING"]
    fn solve_run(&self, src: SocketAddr, info: JsonValue) {
        let id = info["id"].as_u32().unwrap();
        let problem = info["problem"].to_string();
        let sol = info["sol"].to_string();
        let spj = info["spj"].as_bool().unwrap();
        let opt = JudgeOption::new(
            info["time"].as_u32().unwrap(),
            info["mem"].as_u32().unwrap(),
        );
        let docker = Docker::connect_with_defaults().unwrap();
        let lang = match info["lang"].as_str().unwrap() {
            "GCC" => language::GCC,
            "GNU" => language::GNU,
            "CLANG" => language::CLANG,
            "CLANGPP" => language::CLANGPP,
            "JAVA" => language::JAVA,
            "RUST" => language::RUST,
            "PY2" => language::PY2,
            "PY3" => language::PY3,
            _ => panic!("lang err"),
        };
        loop {
            let jb = self.Q.pop();
            match jb {
                Err(T) => {
                    sleep(Duration::from_millis(500));
                }
                Ok(idx) => {
                    SphinxCore::Run::Run(
                        &self.sx, &docker, &idx, &id, &problem, lang, spj, &opt, &sol,
                    );
                    self.Q.push(idx).unwrap();
                    break;
                }
            }
        }
    }

    /// TODO :Link TestLib
    fn solve_judge(&self, src: SocketAddr, info: JsonValue) {}

    fn solve_data(&self, src: SocketAddr, info: JsonValue) {}

    pub fn run(&self) {
        let mut buf: Vec<u8> = vec![0; 700_000];
        crossbeam::thread::scope(|s| loop {
            let (amt, src) = match self.rx.recv_from(&mut buf) {
                Ok(x) => x,
                Err(x) => {
                    println!("{:?}", x);
                    continue;
                }
            };
            println!("{}", format!("Local received {} bytes : {:?}", amt, src));
            let fit = String::from_utf8(buf.clone());
            match fit {
                Ok(T) => {
                    let res = json::parse(T.as_str());
                    match res {
                        Ok(res) => {
                            let jb = res["Method"].as_str().unwrap();
                            let info = res["Data"][0].clone();
                            match jb {
                                "run" => s.spawn(move |_| self.solve_run(src, info)),
                                "upload judge" => s.spawn(move |_| self.solve_judge(src, info)),
                                "upload data" => s.spawn(move |_| self.solve_data(src, info)),
                                _ => panic!("method error"),
                            };
                        }
                        Err(T) => panic!("method error"),
                    }
                }
                Err(T) => panic!("method error"),
            }
        })
        .unwrap();
    }
}
