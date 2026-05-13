use std::{error::Error, io::{Read, Write}, path::Path,pin::Pin};

use clap::Parser;
use commun_utils_handler::{ errors::GlobalError, fs_strategies::FileReader};
use derive_utils::IterableStringifyEnum;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, accept_async, connect_async, tungstenite::{Message, client::connect_with_config, handshake::server::Callback, protocol::WebSocketConfig}};
use futures::{SinkExt, StreamExt, stream::{self, SplitSink, SplitStream}};

fn parse_file_reader(s:&str)-> Result<FileReader, String> 
{
    FileReader::try_from(Path::new(s)).map_err(|_|String::from(s))
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

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

type WriterSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type ReaderSender = SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>;

async fn connect_with_protocol<F>(command_line_args:Cli,callback:F)->Result<(),Box<dyn Error>> 
    where   
        F: for<'a> FnOnce(&'a mut WriterSender,&'a mut ReaderSender,Option<FileReader>)->BoxFuture<'a,Result<(),Box<dyn Error>>>
{
    let (stream,_) = connect_async("ws://localhost:8080").await?;
    let (mut write,mut read) = stream.split();
    let _ = write.send(Message::Text(command_line_args.pattern.into())).await;
    callback(&mut write,&mut read,command_line_args.path).await?;
    Ok(())
}


async fn handle_add_file(write:&mut WriterSender,read:&mut ReaderSender,file_reader:Option<FileReader>)->Result<(),Box<dyn Error>>
{
    if let Some(file) = file_reader {
        let _ = write.send(Message::Text(file.to_string_lossy().to_string())).await;
        let mut buffers = Vec::new();
        file.flush_data(&mut buffers)?;
        Ok(())
    } else {
        return Err(Box::new(GlobalError::ParseError("missing file".to_string())));
    }
}



#[tokio::main]
async fn main()->Result<(), Box<dyn Error>>
{
    let args = Cli::parse();
    match args.pattern {
        Command::AddFile => {
            connect_with_protocol(args,|write,read,path|
                {
                    Box::pin(handle_add_file(write, read,path))
                }).await?;

            // if let Some(_) = args.path {
            //     // let config = WebSocketConfig::default().

            //     // connect_with_config(request, config, max_redirects)
            //     // connect_with_config("ws://localhost:8080", max_redirects);
            //     // let (stream,_) = connect_async("ws://localhost:8080").await?;
                
            //     // // let stream = TcpStream::connect("localhost:8080").await?;
            //     // // let mut ws = accept_async(stream).await?;
            //     // let (mut write,_) = stream.split();
            //     // // header
            //     // let _ = write.send(Message::Text(args.pattern.into())).await;

            //     // let mut buffers = Vec::new(); 
            //     // file.flush_data(&mut buffers)?;
            //     // for buffer in buffers { 
            //     //     let _ = write.send(Message::Binary(buffer.to_vec())).await;
            //     // } 
            //     // let listener = TcpListener::bind("localhost:8080").await.unwrap();
            //     // let mut buffers = Vec::new();
            
            //     // stream.write_all(file.to_string_lossy().as_bytes())?;
                
            //     // stream.flush()?;  
            //     // dbg!()
            // } else {
            //     // return Err(Box::new(GlobalError::ParseError("missing file".to_string())));
            // }
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
