
extern crate sphinx_core;

use sphinx_core::{ProblemConfigOptions, ProblemConfig};

fn main() {
    let option = ProblemConfigOptions {
        spj_path: "123".to_owned(),
    };

    let cfg = ProblemConfig::read("../sphinx-core/examples/bs.toml", &option).unwrap();

    println!("{}", cfg.judge_type);
}