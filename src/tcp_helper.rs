use crate::request_pool::RequestPool;
use std::
    net::TcpListener
;



pub fn start(listen_port: u16) {
    assert!(listen_port > 8000u16);
    assert!(listen_port < 65535u16);
    let listenaddr = format!("127.0.0.1:{}", listen_port);
    let listener = TcpListener::bind(listenaddr).expect("端口监听失败");
    println!("starting listen on port:{}", listen_port);

    let requestpool= RequestPool::new(6);
    for tcpstream in listener.incoming() {
        requestpool.execute(tcpstream.unwrap());
    }
}
