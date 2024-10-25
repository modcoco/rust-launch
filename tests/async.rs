use std::sync::{Arc, Mutex};

use tokio::{
    task,
    time::{sleep, Duration},
};

#[tokio::test]
async fn test_async() {
    println!("start");
    test_concurrent_tasks().await;
    println!("end");
}

async fn test_concurrent_tasks() {
    // 共享的计数器，用于记录所有任务的执行情况
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let counter = Arc::clone(&counter);

            task::spawn(async move {
                // 模拟任务执行时间
                sleep(Duration::from_millis(10000)).await;
                println!("Task {} is running on a different thread!", i);

                let mut num = counter.lock().unwrap();
                *num += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    // 检查是否所有任务都执行过
    let result = *counter.lock().unwrap();
    assert_eq!(result, 5, "Not all tasks were executed!");

    println!("All tasks executed successfully. Total count: {}", result);
}
