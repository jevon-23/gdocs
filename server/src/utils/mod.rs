use std::net::TcpStream;
use std::io::Write;
/* Send response back to the client */
pub fn send_response(mut stream : &TcpStream, response : String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
