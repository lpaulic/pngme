use std::env;
use std::process;

use pngme::Config;
use pngme::ConfigError;

fn main() {
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(ConfigError::ArgumentParsing(err)) => err.exit(),
        Err(err) => {
            eprintln!("Non parsing error: {}", err);
            process::exit(1)
        }
    };

    if let Err(err) = pngme::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
