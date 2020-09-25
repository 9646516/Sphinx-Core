extern crate sphinx;

#[cfg(test)]
mod test {
    use sphinx::proto::{ProblemConfig, ProblemConfigOptions};

    #[test]
    fn test_problem_config() {
        let option = ProblemConfigOptions {
            spj_path: "123".to_owned(),
        };

        let cfg = ProblemConfig::read("test/bs.toml", &option).unwrap();

        assert_eq!("oi", cfg.judge_type);
    }
}