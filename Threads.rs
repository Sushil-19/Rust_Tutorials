/*
Threading
*/

use std::thread;
use thread::JoinHandle;
use std::time::Duration;

fn main(){
    println!("This is print1");
    println!("This is print2");
    println!("This is print3");
    println!("This is print4");
    
    let t:JoinHandle<()> = thread::spawn(|| {
        println!("This is print1 in thread");
        println!("This is print2 in thread");
        println!("This is print3 in thread");
        println!("This is print4 in thread");
        println!("This is print5 in thread");
        thread::sleep(Duration::from_millis(1));
        println!("This is print6 in thread");
        println!("This is print7 in thread");
        println!("This is print8 in thread");
        println!("This is print9 in thread");
        println!("This is print10 in thread");
    });

    thread::sleep(Duration::from_millis(1));
    println!("This is print1 out of thread");
    println!("This is print2 out of thread");
    println!("This is print3 out of thread");
    println!("This is print4 out of thread");
    t.join();
}