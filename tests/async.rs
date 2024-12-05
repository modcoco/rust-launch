use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::{
    sync::mpsc,
    task,
    time::{sleep, Duration},
};

#[tokio::test]
async fn test_async() {
    println!("start");
    test_concurrent_tasks().await;
    println!("end");
}

// •	单向消息传递：使用 mpsc 或 oneshot。
// •	多生产者多消费者：使用 broadcast。
// •	共享数据：使用 Mutex 或 RwLock。
// •	状态更新：使用 watch。
// •	同步：使用 Barrier。
#[tokio::test]
async fn test_tokio_msg_with_hashmap() {
    // 创建两个异步通道：主线程 -> 辅助线程 和 辅助线程 -> 主线程
    let (main_to_worker_tx, mut main_to_worker_rx) = mpsc::channel::<HashMap<String, String>>(10);
    let (worker_to_main_tx, mut worker_to_main_rx) = mpsc::channel::<HashMap<String, String>>(10);

    // 启动辅助线程任务
    tokio::spawn(async move {
        while let Some(mut map) = main_to_worker_rx.recv().await {
            println!("辅助线程收到消息: {:?}", map);

            // 处理消息并返回结果
            map.insert("response".to_string(), "收到主线程的HashMap".to_string());
            worker_to_main_tx.send(map).await.unwrap();
        }
    });

    // 主线程构造 HashMap 并发送到辅助线程
    let mut main_map = HashMap::new();
    main_map.insert("key1".to_string(), "value1".to_string());
    main_map.insert("key2".to_string(), "value2".to_string());

    main_to_worker_tx.send(main_map).await.unwrap();

    // 主线程接收来自辅助线程的 HashMap 回复
    if let Some(response_map) = worker_to_main_rx.recv().await {
        println!("主线程收到回复: {:?}", response_map);
    }
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
