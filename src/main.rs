extern crate threadpool;
extern crate rand;
extern crate random;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

const TEST_TASKS : usize = 8;
const NUM_THREADS : usize = 2;

fn main() {
  println!("Hello, lasttest!");
  
  let pool = ThreadPool::new(NUM_THREADS);
  
  println!("Static started...");
  run_static(&pool);
  println!("Static finished");
  
  println!("Communicating started...");
  run_communicating(&pool);
  println!("Communicating finished");
}

fn run_static(pool : &ThreadPool) {
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
          float_rand();
          23
        },
        _ => {
          let mut v = vec![];
          for i in 0..10000000 {
              v.push(i % j);
          }
          v.len()
        },
      };
      tx.send(s).unwrap();
    });
  }
  
  println!("Result b{}", rx.iter().take(TEST_TASKS).fold(0, |a, b| a + b));
}

fn run_communicating(pool : &ThreadPool) {
  let (sender, receiver) = channel();
  
  for i in 0..TEST_TASKS {
    let sender = sender.clone();
    pool.execute(move|| {
      sender.send(pi_approx_random(42000000, rand::random::<f64>)).unwrap()
    });
  }
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
        if r == 0 {
          println!("{} times {} is 0", b, f);
        }
      }
    }
  }
}

fn float_rand() {
  let mut pi_approx : f64 = 0.0;
  let step_size : f64 = 0.00001;
  let mut current_x : f64 = 0.0;
  let mut current_y : f64 = 0.0;
  
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
