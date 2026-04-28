use std::{error::Error, io::{Read, Write}, path::Path};

use clap::Parser;
use commun_utils_handler::{errors::GlobalError, fs_strategies::FileReader};
use derive_utils::IterableStringifyEnum;
use commun_utils_handler::IterableStringifyEnum;
use tokio::{net::TcpListener};


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


#[tokio::main]
async fn main()->Result<(), Box<dyn Error>>
{
    let args = Cli::parse();
    match args.pattern {
        Command::AddFile => {
            if let Some(file) = args.path {
                let listener = TcpListener::bind("localhost:8080").await.unwrap();
                // let mut buffers = Vec::new();
            
                // stream.write_all(file.to_string_lossy().as_bytes())?;
                // file.flush_data(&mut buffers)?;
                // for buffer in buffers {
                //     stream.write_all(&buffer)?;
                // } 
                // stream.flush()?;  
                // dbg!()
            } else {
                // return Err(Box::new(GlobalError::ParseError("missing file".to_string())));
            }
        },
        Command::AddProgram => {

        }
    }
    Ok(())
}
