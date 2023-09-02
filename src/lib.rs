use std::error::Error;
use std::fs;
pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build<'b>(args: &'b [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query: &str = &args[1];
        let file_path: &str = &args[2];

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}
