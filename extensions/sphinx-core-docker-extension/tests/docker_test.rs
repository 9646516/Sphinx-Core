extern crate sphinx_core_docker;

#[cfg(test)]
mod test {
    use dockworker::Docker;

    use sphinx_core_docker::utils::{create_judge_container, remove_judge_container};

    const PAN_DIR: &str = "/home/rinne/work/gosrc/src/github.com/Myriad-Dreamin/boj-v6/problem/";

    #[test]
    fn docker_test() {
        let path: String = format!("{}", PAN_DIR);
        let docker = Docker::connect_with_defaults().unwrap();
        let container_id = create_judge_container(&docker, &path).unwrap();

        println!("{}", container_id);

        assert_ne!(container_id, "");

        remove_judge_container(&docker, &container_id).unwrap();
    }
}