/*
Multiple threads
ownerships
*/

use std::thread;
use thread::JoinHandle;

fn main(){
    // let x:i32 = 10;
    // let y:String = String::from("Hello");

    // let t1:JoinHandle<()> = thread::spawn(move || {
    //     println!("x is {}", x);
    //     println!("y is {}", y);
    // });

    // t1.join();

    let mut vector_thread: Vec<JoinHandle<()>> = vec![];
    for i in 1..10{
        vector_thread.push(thread::spawn(move || {
            println!("thread no {}",i);
        }));
    }
    for i in vector_thread{
        i.join();
    }
}
