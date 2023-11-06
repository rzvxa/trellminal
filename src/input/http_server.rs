use super::{Event, EventSender};
use std::{
    fs,
    io::{ErrorKind, prelude::*, BufReader, Error as IoError},
    net::{TcpListener, TcpStream},
    ops::Drop,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

pub type Port = &'static str;

pub struct Request {
    url: String,
    stream: TcpStream,
}

impl Request {
    pub fn url(&self) -> &String {
        return &self.url
    }

    pub fn respond(mut self, content: String) -> Result<(), IoError> {
        let status_line = "HTTP/1.1 200 OK";
        let length = content.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

        return self.stream.write_all(response.as_bytes());
    }
}

pub trait RespondWithHtml {
    fn respond_with_html(self, view_path: &str) -> Result<(), IoError>;
}

impl RespondWithHtml for Request {
    fn respond_with_html(self, view_path: &str) -> Result<(), IoError> {
        let view = fs::read_to_string(view_path)?;
        self.respond(view)
    }
}

enum ThreadMessage {
    Terminate,
}

pub struct HttpServer {
    thread_message_broker: Sender<ThreadMessage>,
}

fn handle_connection(mut stream: TcpStream) -> Request {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    Request { url: "URL".to_string(), stream }
}

impl HttpServer {
    pub fn new(event_sender: EventSender, port: Port) -> Self {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(200);
        let server = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
        server.set_nonblocking(true).unwrap();
        thread::spawn(move || {
            for stream in server.incoming() {
                println!("loop|");
                match stream {
                    Ok(s) => {
                        let req = handle_connection(s);
                        event_sender.send(Event::Request(req)).unwrap();
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        match rx.try_recv().ok() {
                            Some(msg) => match msg {
                                ThreadMessage::Terminate => {
                                    break;
                                }
                            },
                            None => {}
                        }
                    }
                    Err(e) => panic!("encountered IO error: {}", e),
                }
                thread::sleep(tick_rate);
            }
            println!("End")
        });
        Self {
            thread_message_broker: tx,
        }
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        println!("MIC DROP");
        self.thread_message_broker
            .send(ThreadMessage::Terminate)
            .unwrap_or(());
        println!("SENTTT");
    }
}
