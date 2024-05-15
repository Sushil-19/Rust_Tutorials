// #![feature(async_await)]
// use std::time::Duration;
// use tokio::time::sleep;

// async fn async_task() -> String {
//     println!("Starting async task...");
//     sleep(Duration::from_secs(2)).await;
//     println!("Async task completed!");
//     "Result from async task".to_string()
// }


// #[tokio::main]
// async fn main() {
//     println!("Main thread: Starting...");
//     let result = async_task().await;
//     println!("Main thread: Received result: {}", result);
//     println!("Main thread: Exiting...");
// }

async fn printing(){
    println!("I am async function");
}

fn main(){
    let x: impl Future<Output = ()> = printing();
}