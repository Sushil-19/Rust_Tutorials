/*
Channels with MPSC
*/

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender,Receiver};

fn main(){
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let tx1:Sender<i32> = tx.clone();

    thread::spawn(move || {
        let v1:Vec<i32> = vec![1,2,3,4,5];
        for i in v1{
            thread::sleep(Duration::from_secs(1));
            tx.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        let v1:Vec<i32> = vec![6,7,8,9,0];
        for i in v1{
            thread::sleep(Duration::from_secs(1));
            tx1.send(i).unwrap();
        }
    });

    for rec_val in rx{
        println!("{}",rec_val);
    }
}


// fn timer(d: i32, tx: mpsc::Sender<i32>){
//     thread::spawn(move ||{
//         println!("Sending {}",d);
//         tx.send(d).unwrap();
//     });
// }

// fn main(){
//     let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

//     for i in 1..10{
//         timer(i, tx.clone()); // error here
//     }

//     for rec_val in rx{
//         println!("{}",rec_val);
//     }
// }