use mini_grep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    dbg!("filename:{} q:{}", &config.filename, &config.q);
    if let Err(err) = mini_grep::run(config) {
        eprintln!("Error reading the file: {}", err);
        process::exit(1);
    }
}
