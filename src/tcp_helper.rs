use crate::http_method::ToHttpMethod;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
    ops::Index,
};

use super::http_method::HttpMethod;

pub fn start(listen_port: u16) {
    assert!(listen_port > 8000u16);
    assert!(listen_port < 65535u16);
    let listenaddr = format!("127.0.0.1:{}", listen_port);
    let listener = TcpListener::bind(listenaddr).expect("端口监听失败");
    println!("starting listen on port:{}", listen_port);

    for tcpstream in listener.incoming() {
        if let Ok(tcpstream) = tcpstream {
            handle_stream(tcpstream);
        } else {
            //ignore
            continue;
        }
    }
}
fn handle_stream(mut tcpstream: TcpStream) {
    // let httprequest = HttpRequest::from_tcp(&mut tcpstream);
    // println!("{:?}", httprequest);
    let reader = BufReader::new(&mut tcpstream);
    for lres in reader.lines() {
        match lres {
            Ok(line_content) => println!("{line_content}"),
            Err(e) => println!("****************err,{e}****************"),
        }
    }
}
#[derive(Debug, Default)]
struct HttpRequest {
    method: HttpMethod,
    hash: String,
    query_strs: HashMap<String, String>,
    content_type: String,
    accept: String,
    portol: String,
    host: String,
    user_agent: String,
    headers: HashMap<String, String>,
    content: Option<String>,
}
impl HttpRequest {
    fn from_tcp(stream: &mut TcpStream) -> HttpRequest {
        let mut default_request = HttpRequest::default();
        let reader = BufReader::new(stream);
        let lines: Vec<_> = reader
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| !l.is_empty())
            .collect();
        for (index, mut line) in lines.into_iter().enumerate() {
            if index == 0 {
                let mut first_contents = line.split_whitespace();
                let http_method = first_contents.next().expect("not found httpmethod");
                default_request.method = http_method.to_http_method();

                let path = first_contents.next().expect("not found path");
                if let Some(querystr_index) = path.find("?") {
                    let (path, querystr) = path.split_at(querystr_index);
                    default_request.hash.push_str(path);
                    querystr
                        .trim_start_matches("?")
                        .split("&")
                        .for_each(|item| {
                            if let Some(i) = item.find("=") {
                                let (key, value) = item.split_at(i);
                                default_request
                                    .query_strs
                                    .insert(key.to_string(), value.to_string());
                            }
                        });
                } else {
                    default_request.hash.push_str(path);
                }

                let protol = first_contents.next().expect("not found protol");
                default_request.portol.push_str(protol);
            } else if line.starts_with("User-Agent") {
                let ua = line.split(':').last().unwrap();
                default_request.user_agent.push_str(ua);
            } else if line.starts_with("Host") {
                let (_key, value) = line.split_at(4);
                let host: &str = value.trim().trim_start_matches(':').trim();
                default_request.host.push_str(host);
            } else {
                if let Some(index) = line.find(':') {
                    let value = line.split_off(index);
                    default_request.headers.insert(line, value);
                }
            }
        }
        default_request
    }
}
