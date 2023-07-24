use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream}, time::Duration,
    thread,
};
use webserver::ThreadPool;

fn main() {
    //TCP服务绑定7878端口
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 新建大小为4的线程池
    let thread_pool = ThreadPool::new(4);
    // 对监听器读取的流数据进行遍历
    for stream in tcp_listener.incoming() {
        //拿出数据流
        let stream = stream.unwrap();
        // 传入func作为参数，执行线程池execute
        thread_pool.execute(|| handle_tcpstream(stream));
    }
}

fn handle_tcpstream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();
    let (http_status, file_path) = match &http_request[..] {
        "GET / HTTP/1.1" =>  ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ =>  ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
   

    let contents = fs::read_to_string(file_path).unwrap();
    let content_length = contents.len();
    let response = format!("{http_status}\r\nContent-Length: {content_length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
//封装reponse
