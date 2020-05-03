use MultithreadedServer::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    let request_line = b"GET / HTTP/1.1\r\n";
    let sleep_line = b"GET /sleep HTTP/1.1\r\n";
    let (status,filename) = if buffer.starts_with(request_line) {
        ("HTTP/1.1  OK\r\n\r\n", "hello.html")
    }
    else if buffer.starts_with(sleep_line){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1  OK\r\n\r\n", "hello.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","hello_error.html")
    };
    let html_response = fs::read_to_string(filename).unwrap();
    //println!("Htnml resposne is:{}",html_response);
    let response = format!("{}{}",status,html_response); //200 is the status code here.
    //println!("Whole response sent from server is:{}",html_response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}