use std::{io::{self, BufRead}, thread};
use clap::Parser;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::process::Command;

// maybe add an text mode and ansi text
// but the idea is to only send html

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(default_value = "127.0.0.1")]
    host: String,

    port: Option<u32>,
    
    #[clap(default_value = "xdg-open")]
    browser: String
}

fn read_stdin() -> io::Result<String> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let buf = handle.lines().map(|i| i.unwrap()).collect();
    Ok(buf)
}

fn handle_connection(mut stream: TcpStream, pipein: &String) {
    // output the html and http

    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", http_request);
}

fn start_webserver(host: &String, pipein: &String) -> io::Result<()> {
    let listener = TcpListener::bind(host)?;

    if let Some(stream) = listener.incoming().next() {
        let stream = stream?;
        handle_connection(stream, pipein);
    }

    Ok(())
}

fn main() -> io::Result<()>{
    let args = Cli::parse();
    let port = args.port.unwrap_or_else(|| 7878);
    let host = format!("{}:{}", args.host, port);
    let url = format!("http://{}", &host);
    let mm = read_stdin()?;
    let ret = thread::spawn(move ||{
        match start_webserver(&host, &mm) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error in webserver thread: {e}");
            }
        }
    });
    Command::new(&args.browser)
        .arg(url)
        .output()?;
    ret.join().expect("Couldn't wait on thread");
    Ok(())
}
