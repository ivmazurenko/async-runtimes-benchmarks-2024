use std::env;
use async_std::task;
use futures::future::join_all;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let num_tasks = args[1].parse::<usize>().unwrap();
    
    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        tasks.push(task::sleep(Duration::from_secs(10)));
    }

    join_all(tasks).await;
}
