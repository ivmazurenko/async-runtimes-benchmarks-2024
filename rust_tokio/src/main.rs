use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let num_tasks = args[1].parse::<i32>().unwrap();
    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        tasks.push(sleep(Duration::from_secs(10)));
    }
    futures::future::join_all(tasks).await;
}