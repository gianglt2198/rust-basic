// use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let logs: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(
        (0..100_000_000)
            .map(|i| {
                if i % 100 == 0 {
                    "ERROR: Connection failed".to_string()
                } else {
                    "INFO: All systems go".to_string()
                }
            })
            .collect(),
    ));

    // let start = std::time::Instant::now();
    // let count: usize = logs.iter().filter(|log| log.contains("ERROR")).count();

    // println!(
    //     "Sequential: Found {} errors in {:?}",
    //     count,
    //     start.elapsed()
    // );

    let start = std::time::Instant::now();

    // thread::scope(|s| {
    //     let writer_logs = Arc::clone(&logs);
    //     s.spawn(move || {
    //         let mut lock = writer_logs.lock().unwrap();
    //         for _ in 0..1000 {
    //             lock.push("ERROR: Connection failed".to_string());
    //         }
    //     });
    // });

    let final_logs = logs.lock().unwrap();
    let num_threads = 4;
    let chunk_size = final_logs.len() / num_threads;

    let total_errors = thread::scope(|s| {
        let mut handles = Vec::new();

        for chunk in final_logs.chunks(chunk_size) {
            let handle = s.spawn(move || chunk.iter().filter(|log| log.contains("ERROR")).count());
            handles.push(handle);
        }
        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<usize>()
    });

    println!(
        "Parallel: Found {} errors in {:?}",
        total_errors,
        start.elapsed()
    );

    // let (tx, rx) = mpsc::channel();

    // thread::scope(|s| {
    //     // let writer_logs = Arc::clone(&logs);
    //     // s.spawn(move || {
    //     //     let mut lock = writer_logs.lock().unwrap();
    //     //     for _ in 0..1000 {
    //     //         lock.push("ERROR: Connection failed".to_string());
    //     //     }
    //     // });

    //     let num_threads = 4;

    //     for i in 0..num_threads {
    //         let thread_tx = tx.clone();
    //         let reader_logs = Arc::clone(&logs);
    //         s.spawn(move || {
    //             let lock = reader_logs.lock().unwrap();
    //             let len = lock.len();
    //             let chunk_size = len / num_threads;

    //             let start_idx = i * chunk_size;
    //             let end_idx = if i == num_threads - 1 {
    //                 len
    //             } else {
    //                 (i + 1) * chunk_size
    //             };

    //             let count = lock[start_idx..end_idx]
    //                 .iter()
    //                 .filter(|log| log.contains("ERROR"))
    //                 .count();

    //             thread_tx.send(count).unwrap();
    //         });
    //     }
    // });

    // drop(tx);

    // let total_errors: usize = rx.iter().sum();

    // println!("Found {} errors in {:?}", total_errors, start.elapsed());
}
