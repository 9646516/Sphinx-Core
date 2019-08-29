#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;

use std::collections::HashMap;
use std::sync::RwLock;
use std::thread;

use crossbeam::crossbeam_channel::bounded;

use MQ_grpc::*;
use MQ::*;

use crate::SphinxCore::Judge::{JudgeResult, JudgeStatus};

pub mod Server;
pub mod SphinxCore;
pub mod MQ;
pub mod MQ_grpc;

struct GRPCServer {
    pub serv: Server::Server,
}

impl Mq for GRPCServer {
    fn submit(
        &self,
        _m: grpc::RequestOptions,
        req: submit_request,
    ) -> grpc::SingleResponse<submit_response> {
        let lang = req.lang;
        let solution = req.solution;
        let time = req.time;
        let mem = req.mem;
        let uid = req.uid;
        let mut r = submit_response::new();
        r.res = self.serv.push(lang, solution, time, mem, uid);
        grpc::SingleResponse::completed(r)
    }

    fn check(
        &self,
        _m: grpc::RequestOptions,
        req: check_request,
    ) -> grpc::SingleResponse<check_response> {
        let uid = req.uid;
        let mut r = check_response::new();
        match self.serv.get(uid) {
            Some(T) => {
                r.status = T.status.to_string();
                r.time = T.time_cost;
                r.mem = T.memory_cost;
                r.last = T.last;
            }
            None => {
                r.status = "UID NOT VALID".to_string();
            }
        }
        grpc::SingleResponse::completed(r)
    }
}

/// TODO :ADD A Message Queue
fn main() {
    let S = GRPCServer {
        serv: Server::Server::new(),
    };
    let mut _server = grpc::ServerBuilder::new_plain();
    _server.http.set_port(2333);
    _server.add_service(MqServer::new_service_def(S));
    _server.http.set_cpu_pool_threads(4);
    let server = _server.build().expect("grpc server failed");
    loop {
        thread::park();
    }
}
