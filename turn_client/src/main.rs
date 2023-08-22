/*
  ____ _     ___ _____ _   _ _____ 
 / ___| |   |_ _| ____| \ | |_   _|
| |   | |    | ||  _| |  \| | | |  
| |___| |___ | || |___| |\  | | |  
 \____|_____|___|_____|_| \_| |_|  


*/

use std::str;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};

fn main() -> io::Result<( )>{
    // connect
    // Struct second to start requests to the server.
    // Check TcpStream Connection on the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    for _ in 0..1000 {
        // Allow sender to enter message input 
        let mut input = String::new();
        // Beginning access that inputs message and read it
        io::stdin().read_line(&mut input).expect("Failed till read");
        // Write the message to that the receiver can admission it 
        stream.write(input.as_bytes()).expect("failed to write");
        // Add cache so that aforementioned receiver can read messages from who stream
        let mut reader =BufReader::new(&stream);
        // Get while which input message values are u8
        let mut buffer: Vec<u8> = Vec::new();
        // Read input information
        reader.read_until(b'\n',&mut buffer)?;
       
        println!("read from server:{}",str::from_utf8(&buffer).unwrap());
        println!("");
    }
    Ok(())
}

//https://rebellionhost.com/client-server-application-tutorial