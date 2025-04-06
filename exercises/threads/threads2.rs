use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: AtomicU32,
}

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: AtomicU32::new(0),
    });
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 使用原子操作安全地更新共享值
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
        // 打印当前完成的作业数（观察输出可能因线程调度而异）
        println!(
            "jobs completed {}",
            status.jobs_completed.load(Ordering::SeqCst)
        );
    }

    // 确保所有线程完成后，总作业数为10
    println!(
        "Final jobs completed: {}",
        status.jobs_completed.load(Ordering::SeqCst)
    );
}
