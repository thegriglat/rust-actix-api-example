use std::env;

use dotenv::dotenv;

pub struct Config {
    pub port: u16,
    pub workers: usize,
}

impl Config {
    pub fn read() -> Self {
        dotenv().ok();
        let port = Config::port();
        let workers = Config::workers();

        Config { port, workers }
    }

    fn port() -> u16 {
        Config::get_env_var("PORT")
            .expect("PORT variable not found")
            .parse::<u16>()
            .expect("Cannot parse PORT")
    }

    fn workers() -> usize {
        match Config::get_env_var("WORKERS") {
            Some(value) => value.parse::<usize>().expect("Cannot parse WORKERS"),
            None => {
                let cpus = num_cpus::get_physical();
                println!("Cannot parse WORKERS variable. Use number of CPU: {}", cpus);
                cpus
            }
        }
    }

    fn get_env_var(key: &str) -> Option<String> {
        match env::vars().find(|x| x.0 == key) {
            Some(pair) => Some(pair.1),
            None => None,
        }
    }
}
