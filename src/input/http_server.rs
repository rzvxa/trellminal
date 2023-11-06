use super::{Event, EventSender};
use std::{
    fs,
    io::{prelude::*, BufReader, Error as IoError, ErrorKind},
    net::{TcpListener, TcpStream},
    ops::Drop,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub type Port = &'static str;
pub type Validator = fn(Request) -> Option<Request>;

pub struct Request {
    url: String,
    stream: TcpStream,
}

impl Request {
    pub fn url(&self) -> &String {
        return &self.url;
    }

    pub fn url_str(&self) -> &str {
        return &self.url().as_str();
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
        let full_path = format!("res/www/{view_path}");
        let view = fs::read_to_string(full_path)?;
        self.respond(view)
    }
}

enum ThreadMessage {
    Terminate,
}

pub struct HttpServer {
    thread_message_broker: Sender<ThreadMessage>,
}

fn handle_connection(mut stream: TcpStream) -> Option<Request> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let first_line = match http_request.first() {
        Some(params) => params,
        None => "BAD REQUEST",
    };

    let mut params = first_line.split(' ');

    // check if first token is GET as we only process get requests
    if params.next().unwrap_or("") != "GET" {
        return None;
    }

    let url = params.next().unwrap_or("");

    Some(Request {
        url: url.to_string(),
        stream,
    })
}

fn background_worker(
    socket: TcpListener,
    event_sender: EventSender,
    event_receiver: Receiver<ThreadMessage>,
    validator: Validator,
) {
    let tick_rate = Duration::from_millis(200);
    for stream in socket.incoming() {
        match stream {
            Ok(s) => match handle_connection(s) {
                Some(req) => {
                    if let Some(req) = validator(req) {
                        event_sender.send(Event::Request(req)).unwrap();
                    }
                }
                None => {}
            },
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => match event_receiver.try_recv().ok()
            {
                Some(msg) => match msg {
                    ThreadMessage::Terminate => {
                        break;
                    }
                },
                None => {}
            },
            Err(e) => panic!("encountered IO error: {}", e),
        }
        thread::sleep(tick_rate);
    }
}

impl HttpServer {
    pub fn new(event_sender: EventSender, port: Port, validator: Validator) -> Self {
        let (tx, rx) = mpsc::channel();
        let socket = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
        socket.set_nonblocking(true).unwrap();
        thread::spawn(move || background_worker(socket, event_sender, rx, validator));
        Self {
            thread_message_broker: tx,
        }
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.thread_message_broker
            .send(ThreadMessage::Terminate)
            .unwrap_or(());
    }
}
