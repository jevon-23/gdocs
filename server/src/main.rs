use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod login;
mod users;
mod file_system;
mod utils;

pub struct Input<'inp> {
    path : Vec<String>,
    url : String,
    action : String,
    user : String,
    params : &'inp [String],
    output_stream : &'inp TcpStream,
}

impl<'inp> Input<'inp> {
    fn new(path : &'inp Vec<String>, stream : &'inp TcpStream) -> Option<Self> {
        if path.len() < 3 {
            println!("No user passed in to the request");
            return None;
        }
        return Some(Self {
            path : path.to_owned(),
            url : path[0].to_owned(),
            action : path[1].to_owned(),
            user : path[2].to_owned(),
            params : &path[3..],
            output_stream : stream,
        })
    }

    #[allow(dead_code)]
    fn print(inp : &Self) {
        println!("path: {}", inp.path.join(" "));
        println!("url: {}", inp.url);
        println!("action: {}", inp.action);
        println!("user: {}", inp.user);
        println!("params: {}", inp.params.join(" "));
    }
}

// static LOGIN: &str = "http://localhost:8477/login/user";
fn process_input(input_strings : Vec<&str>, mut stream : &TcpStream) {
    let req : &str = input_strings[0];
    let req_split : Vec<&str> = req.split(" ").collect();

    /* Get the string correlating to the path */
    let req_path : &str = req_split[1];

    let path : Vec<&str> = req_path.split("/").collect();
    let path_list : Vec<String> = path.into_iter().map(String::from).collect();

    // let inp : Input = Input::new(&path_list);
    let mut inp : Input = match Input::new(&path_list, stream) {
        Some(inp) => inp,
        None => return,
    };
    Input::print(&inp);

    match inp.action.as_str() {
        "login" => login::handle_login(inp),
        "logout" => login::handle_logout(inp),
        "new" => file_system::new_file(inp),
        _ => println!("Invalid input was passed into program"),
    };
}

fn handle_connection(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let read_string = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", read_string);

    let _split : Vec<&str>= read_string.split("\n").collect();

    process_input(_split, stream);

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
/* Credit:
 * https://medium.com/@rameshovyas/a-step-by-step-guide-to-build-a-custom-http-server-of-own-in-rust-7308cead63a2
 */

fn main() {
    /* Simple HTTP Server */
    /* Author : Ramesh Vyas */
    /* Creating a Local TcpListener at Port 8477 */
    const HOST : &str ="127.0.0.1";
    const PORT : &str ="8477";
    /* Concating Host address and Port to Create Final Endpoint */
    let end_point : String = HOST.to_owned() + ":" +  PORT;
    /*Creating TCP Listener at our end point */
    let listener = TcpListener::bind(end_point).unwrap();
    println!("Web server is listening at port {}",PORT);
    /* Conneting to any incoming connections */
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
        // Call function to process any incomming connections
        handle_connection(&_stream);
    }
}
