use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener,TcpStream},
    thread,
    time::Duration, 
};
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Portfolio_Website.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "Portfolio_Website.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND","Error.html")
    };

/*let (status_line, file_name) = if request == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK","Portfolio_Website.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","Error.html")
    };  */

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);  

    //Use listener.incoming().take(2) to shutdown down after 2 requests
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down...")
}
