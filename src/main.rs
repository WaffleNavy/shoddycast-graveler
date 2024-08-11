use rand::{distributions::Standard, thread_rng, Rng};
use std::time::SystemTime;
use std::sync::mpsc;
use std::thread;

const TOTAL_ITERATIONS: u32 = 1_000_000_000;
const THREAD_COUNT: u8 = 10;


fn main() {
    let mut max: u8 = 0;
    let (send, recv) = mpsc::channel();
    let runs_per_thread = TOTAL_ITERATIONS / THREAD_COUNT as u32;
    
    let start = SystemTime::now();
    // See if there is any remainder and if so, run that number of iterations
    let remainder = TOTAL_ITERATIONS - (runs_per_thread.checked_mul(THREAD_COUNT as u32).unwrap());
    for _ in 0..remainder {
        let count = get_paralysis_count();
        if (count as u8) > max {
            max = count as u8;
        }
    }
    
    // Spawn & run threads
    for _ in 0..THREAD_COUNT {
        let rpt = runs_per_thread.clone();
        let sender = send.clone();
        thread::spawn(move || {
            for _ in 0..rpt {
                let count = get_paralysis_count();
                sender.send(count).unwrap();
            }
        });
    }
    
    // Explicitly drop our own reference to the send channel, so that only the threads have access to it,
    // this prevents an indefinite block in the following loop
    drop(send);

    // Receive results from threads, continues running until all results from all threads have been processed
    loop {
        let outcome = recv.recv();
        if let Err(_) = outcome {
            break;
        }
        let value = outcome.unwrap();
        if (value as u8) > max {
            max = value as u8;
        }
    }

    let duration = SystemTime::now()
                                    .duration_since(start)
                                    .expect("Error getting the duration!");

    println!("Max Value: {}", max);
    println!("Seconds Taken: {}", duration.as_secs_f32());
}


fn get_paralysis_count() -> usize {
    // Get 231 random 8 bit values
    let rand_vals: Vec<u8> = thread_rng().sample_iter(Standard).take(231).collect();
    // Count the values divisible by 4 and return that count
    rand_vals.iter().filter(|x| *x % 4 == 0).count()
}