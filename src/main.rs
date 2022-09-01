use clap::Parser;
use std::{io::{self, BufRead, Write, Read}, thread};
use std::net::{TcpListener, TcpStream};
use std::process::Command;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(default_value = "127.0.0.1")]
    host: String,

    port: Option<u16>,
    
    #[clap(default_value = "xdg-open")]
    browser: String
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;
    Ok(buf)
}

fn handle_connection(mut stream: TcpStream, pipein: &String) -> io::Result<()> {
    let status_line = "HTTP/1.1 200 OK";
    let length = pipein.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{pipein}");

    stream.write_all(response.as_bytes())
}

fn start_webserver(listener: &TcpListener, pipein: &String) -> io::Result<()> {

    if let Some(stream) = listener.incoming().next() {
        let stream = stream?;
        handle_connection(stream, pipein)?;
    }
    Ok(())
}

fn main() -> io::Result<()>{
    let args = Cli::parse();
    let port = args.port.unwrap_or_else(|| 0);
    let host = format!("{}:{}", args.host, port);
    let mm = read_stdin()?;

    let listener = TcpListener::bind(host)?;
    let local_port = listener.local_addr()?.port();
    let url = format!("http://{}:{}", args.host, local_port);
    let ret = thread::spawn(move ||{
        match start_webserver(&listener, &mm) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error in webserver thread: {e}");
            }
        }
    });
    Command::new(&args.browser)
        .arg(url)
        .output()?;
    ret.join().expect("Couldn't wait on http server");
    Ok(())
}
