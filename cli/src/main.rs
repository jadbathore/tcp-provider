use std::{error::Error, ffi::OsString, path::{Path, PathBuf}};

use clap::Parser;
use commun_utils_handler::{errors::GlobalError, fs_strategies::FileReader};
use derive_utils::IterableStringifyEnum;
use commun_utils_handler::IterableStringifyEnum;

fn parse_file_reader(s:&str)-> Result<FileReader, String> 
{
    FileReader::try_from(Path::new(s)).map_err(|_|String::from(s))
}

#[derive(IterableStringifyEnum,Debug,Clone)]
enum Command {
    AddFile,
    AddProgram
}


#[derive(Parser,Debug)]
struct Cli{

    pattern: Command,

    #[arg(value_parser = parse_file_reader)]
    path:Option<FileReader>,
}


fn main()->Result<(), Box<dyn Error>>
{
    let args = Cli::parse();
    match args.pattern {
        Command::AddFile => {
            if let Some(file) = args.path {
                let mut buffers = Vec::new();
                file.flush_data(&mut buffers)?;

                dbg!(file.to_str());
                for buffer in buffers {
                    
                }   
                dbg!()
            } else {
                return Err(Box::new(GlobalError::ParseError("missing file".to_string())));
            }
        },
        Command::AddProgram => {

        }
    }
    Ok(())
}
