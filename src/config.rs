use std::env;

use dotenv::dotenv;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn read() -> Self {
        dotenv().ok();
        let port = Config::port();

        Config { port }
    }

    fn port() -> u16 {
        env::vars()
            .find(|x| x.0 == "PORT")
            .expect("Specify PORT environment variable")
            .1
            .parse::<u16>()
            .expect("Port is not a number")
    }
}
