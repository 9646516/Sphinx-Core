use super::Env;
use std::fs::*;
use toml;
#[derive(Debug)]
pub struct Config {
    pub judge_type: String,
    pub tasks: Vec<Task>,
    pub spj: u8,
    pub spj_path: String,
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub input: String,
    pub output: String,
    pub time: u64,
    pub mem: u64,
    pub score: u64,
}
impl Config {
    pub fn read(path: &str) -> Result<Config, String> {
        let jb = read_to_string(path);
        match jb {
            Ok(_) => {}
            Err(T) => return Err(T.to_string()),
        }
        let file: toml::Value = jb.unwrap().parse().unwrap();
        let judge = file.get("judge").unwrap();
        let judge_type = judge
            .get("judge-type")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let mut tasks = Vec::new();
        let task = judge.get("tasks").unwrap().as_array().unwrap();
        for i in task.iter() {
            tasks.push(Task {
                name: i.get("name").unwrap().as_str().unwrap().to_string(),
                input: i.get("input-path").unwrap().as_str().unwrap().to_string(),
                output: i.get("output-path").unwrap().as_str().unwrap().to_string(),
                time: i.get("time-limit").unwrap().as_integer().unwrap() as u64,
                mem: i.get("memory-limit").unwrap().as_integer().unwrap() as u64,
                score: i.get("score").unwrap().as_integer().unwrap() as u64,
            })
        }
        let spj = file.get("special-judge").unwrap();
        let enable = spj.get("enable").unwrap().as_integer().unwrap() as u8;
        let spj_path = if enable != 0 {
            spj.get("file-path").unwrap().as_str().unwrap().to_string()
        } else {
            Env::JURY.to_owned()
        };
        Ok(Config {
            judge_type,
            tasks,
            spj: enable,
            spj_path,
        })
    }
}
