use rand::{distributions::Standard, thread_rng, Rng};
use std::time::SystemTime;
use std::thread;

const TOTAL_ITERATIONS: u32 = 1_000_000_000;
const THREAD_COUNT: u8 = 10;


fn main() {
    let mut max: u8 = 0;
    let runs_per_thread = TOTAL_ITERATIONS / THREAD_COUNT as u32;
    
    let start = SystemTime::now();
    // See if there is any remainder and if so, run that number of iterations
    let remainder = TOTAL_ITERATIONS - (runs_per_thread * THREAD_COUNT as u32);
    for _ in 0..remainder {
        let count = get_paralysis_count();
        if (count as u8) > max {
            max = count as u8;
        }
    }
    
    // Spawn & run threads
    let mut threads = vec![];
    for _ in 0..THREAD_COUNT {
        let rpt = runs_per_thread.clone();
        let t_tmp = thread::spawn(move || {
            let mut max = 0;
            for _ in 0..rpt {
                let count = get_paralysis_count();
                if count > max {
                    max = count;
                }
            }
            return max
        });
        threads.push(t_tmp);
    }
    
    let mut max = 0;
    for thread in threads {
        let t_max = thread.join().unwrap_or(0);
        if t_max > max {
            max = t_max;
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