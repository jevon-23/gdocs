use std::net::TcpStream;
use std::io::Write;

pub fn send_error_response(mut stream : &TcpStream, response : String) {
    let mut status = "HTTP/1.1 400 OK\r\n\r\n".to_string();
    status.push_str(&response);
    stream.write(status.as_bytes()).unwrap();
    stream.flush().unwrap();
}
/* Send response back to the client */
pub fn send_response(mut stream : &TcpStream, response : String) {
    let mut status = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    status.push_str(&response);
    stream.write(status.as_bytes()).unwrap();
    stream.flush().unwrap();
}
