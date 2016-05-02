extern crate threadpool;
extern crate rand;
extern crate random;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::Instant;

const TEST_TASKS : usize = 3;
const NUM_THREADS : usize = 3;

fn main() {
  println!("Hello, lasttest!\n\nNUM_THREADS: {}\nTEST_TASKS: {}", NUM_THREADS, TEST_TASKS);
  
  let pool = ThreadPool::new(NUM_THREADS);
  
  println!("\nStatic started...");
  let start = Instant::now();
  run_static(&pool);
  let duration = start.elapsed();
  println!("Static finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  
  println!("\nCommunicating started...");
  let start = Instant::now();
  run_communicating(&pool);
  let duration = start.elapsed();
  println!("Communicating finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
}

fn run_static(pool : &ThreadPool) {
  let versuche : usize = 10_000_000;
  let (tx, rx) = channel();
  
  for i in 0..TEST_TASKS {
    let j = i +1;
    let tx = tx.clone();
    pool.execute(move|| {
      let s = match j {
        1 => {
          int_rant();
          42
        },
        2 => {
          float_steps();
          23
        },
        3 => {
          let start = Instant::now();
          pi_approx_random(versuche as u64, rand::random::<f64>);
          let duration = start.elapsed();
          println!("static pi_approx_random finished <=> duration = {:?}\n",   duration);
          13
        },
        _ => {
          let mut v = vec![];
          for i in 0..versuche {
              v.push(i % j);
          }
          v.len()
        },
      };
      tx.send(s).unwrap();
    });
  }
  
  println!("Result {}", rx.iter().take(TEST_TASKS).fold(0, |a, b| a + b));
}

fn run_communicating(pool : &ThreadPool) {
  let versuche : u64 = 42_000_000;
  let (sender, receiver) = channel::<(u64, u64)>();
  
  for _ in 0..TEST_TASKS {
    let sender = sender.clone();
    pool.execute(move|| {
      sender.send(pi_approx_random(versuche, rand::random::<f64>)).unwrap()
    });
  }
  
  println!("Result run_communicating: {}", receiver.iter().take(TEST_TASKS)
            .map(|(inside, _)| 4.0 * inside as f64)
            .fold(0.0, |a : f64, b : f64| a + (b / (versuche as f64) ) )
            / TEST_TASKS as f64);
}

fn int_rant() {
  let v : Vec<usize> = vec![23, 42, 666];
  
  for b in v {
    let h = b.wrapping_mul(0x9E3779B97F4A7C15);
    
    if h == 0 {
      println!("Hash of {} is 0", b);
    } else {
      for f in 2..10000000 {
        let r = b.wrapping_mul(f);
        if r == 0 { // should never be true, but can not be known at compile time
          println!("{} times {} is 0", b, f);
        }
      }
    }
  }
}

fn float_steps() {
  let pi_approx : f64;
  let step_size : f64 = 0.00001;
  let mut current_x : f64 = 0.0;
  let mut current_y : f64;
  
  let mut inside : u64 = 0;
  let mut outside : u64 = 0;
  
  while current_x <= 1.0 {
    current_y = 0.0;
    while current_y <= 1.0 {
      if current_x * current_x + current_y * current_y < 1.0 {
        inside += 1;
      } else {
        outside += 1;
      }
      
      current_y += step_size;
    }
    current_x += step_size;
  }
  
  pi_approx = 4 as f64 * inside as f64 / (inside + outside) as f64;
  
  println!("Pi: {}", pi_approx);
}

/// (inside : u64, outside : u64)
fn pi_approx_random(tries : u64, rand : fn() -> f64) -> (u64, u64) {
  let mut inside = 0;
  
  for _ in 0..tries {
    let rx = rand();
    let ry = rand();
    
    if rx*rx + ry*ry < 1.0 {
      inside += 1;
    }
  }
  
  (inside, tries)
}
