use std::{fs, env};

struct Config {
    read_path: String
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let read_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't specify read path"),
        };
        
        Ok(Config {
            read_path
        })
    }
}

fn run(config: Config) {
    let contents: String = fs::read_to_string(config.read_path).expect("Should have been able to read file");
    reaper_to_osu_timing::run(&contents);
}

fn main() {
    let args = env::args();
    let config: Config = Config::new(args).unwrap();
    run(config);
}
