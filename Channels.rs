/*
Channels and message passing
*/

use std::thread;
use thread::JoinHandle;
use std::sync::mpsc::{self,Sender, Receiver};

fn main(){
    let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();
    let t1:JoinHandle<()> = thread::spawn(move || {
        let data:String = String::from("This is a secret message to main thread");
        println!("Sending the message to main thread");
        tx.send(data).unwrap();
    });

    // let recv_data:String = rx.recv().unwrap();
    // println!("{}", recv_data);

    let mut recv_flag:bool = false;
    t1.join();
    while recv_flag!=true{
        match rx.try_recv(){
            Ok(msg) => {
                println!("{}", msg);
                recv_flag=true;
            },
            Err(_) => println!("Doing something else"),
        }
    }

}