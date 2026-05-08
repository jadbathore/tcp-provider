use std::{error::Error, io::{Read, Write}, path::Path};

use clap::Parser;
use commun_utils_handler::{errors::GlobalError, fs_strategies::FileReader};
use derive_utils::IterableStringifyEnum;
use commun_utils_handler::IterableStringifyEnum;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};
use futures::{SinkExt, StreamExt};

fn parse_file_reader(s:&str)-> Result<FileReader, String> 
{
    FileReader::try_from(Path::new(s)).map_err(|_|String::from(s))
}

#[derive(IterableStringifyEnum,Debug,Clone)]
enum Command {
    AddFile,
    AddProgram,
    Type
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
                let (stream,_) = connect_async("ws://localhost:8080").await?;

                // let stream = TcpStream::connect("localhost:8080").await?;
                // let mut ws = accept_async(stream).await?;
                let (mut write,_) = stream.split();
                let _ = write.send(Message::Text(file.to_string_lossy().to_string())).await;
                let mut buffers = Vec::new(); 
                file.flush_data(&mut buffers)?;
                for buffer in buffers { 
                    let _ = write.send(Message::Binary(buffer.to_vec())).await;
                } 
                // let listener = TcpListener::bind("localhost:8080").await.unwrap();
                // let mut buffers = Vec::new();
            
                // stream.write_all(file.to_string_lossy().as_bytes())?;
                
                // stream.flush()?;  
                // dbg!()
            } else {
                // return Err(Box::new(GlobalError::ParseError("missing file".to_string())));
            }
        },
        Command::AddProgram => {

        },
        Command::Type => {
            if let Some(file) = args.path {
                dbg!(file.get_strategy());
            }
        }
    }
    Ok(())
}
