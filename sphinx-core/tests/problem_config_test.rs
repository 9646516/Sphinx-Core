extern crate sphinx_core;

#[cfg(test)]
mod test {
    use sphinx_core::{ProblemConfig, ProblemConfigOptions};

    #[test]
    fn test_problem_config() {
        let option = ProblemConfigOptions {
            spj_path: "123".to_owned(),
        };

        let cfg = ProblemConfig::read("examples/bs.toml", &option).unwrap();

        assert_eq!("oi", cfg.judge_type);
    }
}