use std::{sync::Arc, collections::HashSet, time::{Instant, Duration}};

use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use tokio_util::time::DelayQueue;

#[tokio::main]
async fn main() {
    async_main_tasks().await;
}

async fn async_main_tasks() {
    let set = Arc::new(RwLock::new(HashSet::<u32>::new()));

    tokio::spawn(async move {
        let mut time = Instant::now();
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("{:?} seconds elapsed", time.elapsed());
            time = Instant::now();
        }
    });

    const NUM_TASKS: u32 = 100_000;
    for i in 0..NUM_TASKS {
        let set = set.clone();
        set.write().await.insert(i);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(10)).await;
            set.write().await.remove(&i);
        });
    }

    loop {
        println!("{} tasks remaining", set.read().await.len());
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
