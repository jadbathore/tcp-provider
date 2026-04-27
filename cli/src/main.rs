use std::{ffi::OsString, path::{Path, PathBuf}};

use clap::Parser;
use commun_utils_handler::fs_strategies::FileReader;


fn parse_file_reader(s:&str)-> Result<FileReader, String> 
{
    FileReader::try_from(Path::new(s)).map_err(|_|String::from(s))
}

#[derive(Parser,Debug)]
struct Cli{
    pattern: String,

    #[arg(value_parser = parse_file_reader)]
    path:FileReader,
}


fn main() {
    let args = Cli::parse();
    dbg!(args);
    println!("Hello, world!");
}


fn a(){

}