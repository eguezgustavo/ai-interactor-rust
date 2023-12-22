mod rest_api;

use std::env;

const PORT_ENV_VAR_NAME: &str = "AI_INTERACTOR_PORT";
const NO_PORT_ERROR: &str = "Port should be set in the AI_INTERACTOR_PORT variable";
const BAD_PORT_ERROR: &str = "Port value should be a number";

fn main() {
    get_api_port();
}

fn get_api_port() -> i32 {
    let port_value = env::var(PORT_ENV_VAR_NAME);
    if port_value.is_err() {
        panic!("{}", NO_PORT_ERROR);
    }

    let parsed_port = port_value.unwrap().parse::<i32>();
    if parsed_port.is_err() {
        panic!("{}", BAD_PORT_ERROR);
    }

    return parsed_port.unwrap();
}

fn start_app(api: impl rest_api::RestAPI, port: i32) {
    api.start(port);
}

#[cfg(test)]
mod test {
    use std::env;
    use rand::Rng;

    use crate::rest_api::restapi::MockRestAPI;

    use super::{start_app, get_api_port};

    struct TestData {
        pub port: i32
    }

    impl TestData {
        fn new() -> Self {
            let mut port_generator = rand::thread_rng();
            let port: i32 = port_generator.gen();
            env::set_var("AI_INTERACTOR_PORT", port.to_string());

            return Self {port};
        }

        fn port(self, port_number: &str) -> Self {
            env::set_var("AI_INTERACTOR_PORT", port_number);
            return  self;
        }
    }

    #[test]
    fn start_app_should_start_the_api_with_the_port_retrieved_from_the_env_variable() {
        let mut api = MockRestAPI::new();
        let test_setup = TestData::new();

        api
            .expect_start()
            .times(1)
            .with(mockall::predicate::eq(test_setup.port))
            .returning(|_| ());

        start_app(api, test_setup.port);
    }

    #[test]
    fn get_api_port_returs_the_port_stored_in_the_env_variable() {
        let test_setup = TestData::new();
        
        let port = get_api_port();
    
        assert_eq!(port, test_setup.port);
    }

    #[test]
    #[should_panic(expected = "Port should be set in the AI_INTERACTOR_PORT variable")]
    fn get_api_port_panics_if_env_variable_with_port_not_set() {
        env::remove_var("AI_INTERACTOR_PORT");
    
        get_api_port();
    }

    #[test]
    #[should_panic(expected = "Port value should be a number")]
    fn get_api_port_panics_if_env_variable_with_port_is_not_a_number() {
        TestData::new().port("BAD_PORT");
    
        get_api_port();
    }
}
