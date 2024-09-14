use std::{net::TcpStream, sync::{mpsc::{self, Receiver, Sender}, Mutex}, thread};
use std::sync::Arc;
use crate::http_processer;


pub struct RequestPool    {
        sender:Sender<TcpStream>,
        workers:Vec<Worker>
}
impl RequestPool{
    pub  fn new(num:u8)->Self{
        assert!(num>0&&num<100);
        let mut workers=Vec::with_capacity(num as usize);
        let (send,recv)=mpsc::channel();
        let receiver=Arc::new(Mutex::new(recv));
        for _ in 0..num {
            let w= Worker::new(receiver.clone());
            w.run();
            workers.push(w);
        }
        Self{
            sender:send,
            workers
        }
    }
   pub fn execute(&self,stream: TcpStream){
       let _= self.sender.send(stream);
    }
}
struct Worker{
    recv:Arc<Mutex<Receiver<TcpStream>>>
}
impl Worker{
    fn new(recv:Arc<Mutex<Receiver<TcpStream>>>)->Self{
        println!("new worker created");
        Self{recv}
    }
    fn run(&self){
        let recv=self.recv.clone();
        thread::spawn(move||{
            loop{
                println!("one worker wait lock");
                let tcpstream= recv.lock().unwrap().recv().unwrap();
                println!("one worker get lock on running..");
                http_processer::handle_stream(tcpstream);
            }
        });
    }
}