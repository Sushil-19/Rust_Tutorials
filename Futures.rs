use std::time::Duration;
use tokio::time::sleep;
async fn async_task() -> String {
    println!("Starting async task...");
    sleep(Duration::from_secs(2)).await;
    println!("Async task completed!");
    "Result from async task".to_string()
}
#[tokio::main]
async fn main() {
    println!("Main thread: Starting...");
    let result = async_task().await;
    println!("Main thread: Received result: {}", result);
    println!("Main thread: Exiting...");
}

// use futures::future::FutureExt;
// async fn produce_tuple() -> (String) {
//     (String::from("Hello"))
// }
// fn main() {
//     let future = produce_tuple();
//     let result = futures::executor::block_on(future);
//     println!("Result: {}", result);
// }


// use std::sync::mpsc;

// macro_rules! create_channel {
//     ($message:expr) => {{
//         let (tx, rx) = mpsc::channel::<String>();
//         tx.send($message.to_string()).unwrap();
//         rx
//     }};
// }

// fn main() {
//     let received = create_channel!("Hello from thread!");
//     println!("Received: {}", received.recv().unwrap());
// }
