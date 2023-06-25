use std::env;

use dotenv::dotenv;

pub struct Config {
    pub port: u16,
    pub workers: usize,
    pub database_url: String,
}

impl Config {
    pub fn read() -> Self {
        dotenv().ok();
        let port = Config::port();
        let workers = Config::workers();
        let database_url = Config::database_url();

        let config = Config {
            port,
            workers,
            database_url,
        };
        config.dump();
        config
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
                println!("Cannot parse WORKERS variable. Will use all available CPUs ()");
                cpus
            }
        }
    }

    fn database_url() -> String {
        Config::get_env_var("DATABASE_URL").expect("DATABASE_URL not set")
    }

    fn get_env_var(key: &str) -> Option<String> {
        match env::vars().find(|x| x.0 == key) {
            Some(pair) => Some(pair.1),
            None => None,
        }
    }

    fn dump(&self) {
        println!("Current config:");
        println!("  PORT:    {}", self.port);
        println!("  WORKERS: {}", self.workers);
    }
}
