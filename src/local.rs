use ::TEST_TASKS;
use ::VERSUCHE;


use threadpool::ThreadPool;
use crossbeam::sync::chase_lev;
use crossbeam::sync::chase_lev::Steal::{Empty, Abort, Data};
use std::sync::mpsc::channel;
use std::time::{Instant, Duration};
use rand;
use std;

pub fn main_local(args : &::Args, pool : &ThreadPool) {
  if args.cmd_static {
    println!("\nStatic started...");
    let start = Instant::now();
    run_static(pool);
    let duration = start.elapsed();
    println!("Static finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_communicating {
    println!("\nCommunicating started...");
    let start = Instant::now();
    run_communicating(pool);
    let duration = start.elapsed();
    println!("Communicating finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_chain {
    println!("\nChain started...");
    let start = Instant::now();
    run_chain(pool);
    let duration = start.elapsed();
    println!("Chain finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_flood {
    println!("\nFlood started...");
    let start = Instant::now();
    run_flood(pool);
    let duration = start.elapsed();
    println!("Flood finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_mesh {
    println!("\nMesh started...");
    let start = Instant::now();
    run_mesh(pool);
    let duration = start.elapsed();
    println!("Mesh finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
}

pub fn run_static(pool : &ThreadPool) {
  let versuche : usize = 1_000_000;
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
          let r = pi_approx_random(versuche as u64, rand::random::<f64>);
          let duration = start.elapsed();
          println!("static pi_approx_random finished <=> duration = {:?} // {}\n", duration, format_pi_approx(r));
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

pub fn run_communicating(pool : &ThreadPool) {
  let versuche = VERSUCHE;
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
  
  println!("float_steps.pi: {}", format_pi_approx((inside, inside+outside)));
}

/// (inside : u64, tries : u64)
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


pub fn run_chain(pool : &ThreadPool) {
let versuche = VERSUCHE;
  let (tx, mut rx) = channel();

  tx.send( (0, 0) ).is_ok();
  
  
  // Chain
  for _ in 0..TEST_TASKS {
    let rx_pre = rx;
    let (tx_chain, rx_chain) = channel();
    
    rx = rx_chain;
    
    pool.execute(move || {
      let r = pi_approx_random(versuche as u64, rand::random::<f64>);
      let b = rx_pre.recv().unwrap();
      tx_chain.send( (b.0 + r.0, b.1 + r.1) ).is_ok();
    });
  }
  
  println!("chain.pi: {}", format_pi_approx(rx.recv().unwrap()));
}


pub fn run_flood(pool: &ThreadPool) {
  let (t_inside, r_inside) = channel();
  let (t_outside, r_outside) = channel();
  let (t, r) = channel();
  
  // Collect all inside
  pool.execute(move || {
    let mut inside = 0;
    for _ in 0..TEST_TASKS {
      while let Some(n) = r_inside.recv().unwrap() {
        inside += n
      }
    }
    
    println!("flood.collected all inside: {}", inside);
    t.send(inside).is_ok();
  });
  
  for _ in 0..TEST_TASKS {
    let inside = t_inside.clone();
    let outside = t_outside.clone();
    
    pool.execute(move || {
      let r = rand::random::<f64>;
      for _ in 0..VERSUCHE {
        let x = r();
        let y = r();
        
        if x*x + y*y > 1.0 {
          outside.send(Some(1)).is_ok();
        } else {
          inside.send(Some(1)).is_ok();
        }
      }
      
      outside.send(None).is_ok();
      inside.send(None).is_ok();
    });
  }
  
  // Collect all outside
  let mut outside = 0;
  for _ in 0..TEST_TASKS {
    while let Some(n) = r_outside.recv().unwrap() {
      outside += n
    }
  }
  println!("flood.collected all outside: {}", outside);
  let inside = r.recv().unwrap();
  
  println!("flood.pi: {}", format_pi_approx( (inside, inside+outside) ));
}


pub fn run_mesh(pool : &ThreadPool) {
  let versuche = VERSUCHE;
  let v : Vec<usize> = vec![23, 42, 666, 521, 8192,
    0x056b, 0xe660, 0x4c74, 0x8ca5, 0x12f6, 0xb000, 0x0a14, 0x40ee,
    0xa224, 0x2b36, 0x9d11, 0x26ab, 0xc2d2, 0xbd39, 0x5d63, 0x9e93,
    0x1fd3, 0x55b4, 0x168f, 0xff9b, 0x3ee2, 0x0342, 0x1cd3, 0x8782,
    0xb321, 0x14e3, 0x7aeb, 0xea8e, 0xd8d2, 0xe43f, 0xd8ee, 0x7a50,
    0x3990, 0x7137, 0xe668, 0xceae, 0x2bfd, 0xd99e, 0x974d, 0x855d,
    0xf4dd, 0xe8db, 0x31a7, 0x4388, 0x6907, 0x283f, 0x3ae2, 0xf9d6,
    0x6987, 0x235d, 0xdaa9, 0x6e0f, 0x3ca8, 0xccab, 0x732f, 0x29e3,
    0x4daf, 0xd6cd, 0xf463, 0x6786, 0xc835, 0x9e5b, 0xd5e6, 0x97e1,
    0x0675, 0x83eb, 0x3392, 0x5cda, 0x8c4b, 0x72e9, 0xed2f, 0x4042,
    0x8bfc, 0x2f52, 0xae13, 0x7666, 0x65c7, 0x2300, 0x50fc, 0xc6f6,
    0xed82, 0xa637, 0xca83, 0x2b08, 0x95b1, 0x67c0, 0x954c, 0x060c,
  ];
  let n_v = v.len();
  
  let (tx, rx) = channel();
  
  for b in v {
    let tx = tx.clone();
    pool.execute(move || {
      let h = b.wrapping_mul(0x9E3779B97F4A7C15);
      
      if h == 0 {
        tx.send(format!("Hash of {} is 0", b)).is_ok();
      } else {
        for f in 2..10_000_000 {
          let r = b.wrapping_mul(f);
          if r == 0 { // should never be true, but can not be known at compile time
            tx.send(format!("{} times {} is 0", b, f)).is_ok();
          }
        }
      }
      
      tx.send(format!("{}", h)).is_ok();
    });
  }
  println!("Pool size #0: {}", pool.active_count());
  
  
  let (mut worker, stealer) = chase_lev::deque();
  pool.execute(move || {
    let mut i = 0;
    for _ in 0..n_v {
      //println!("step1.retransmit into chase_lev");
      worker.push(rx.recv().unwrap());
      i += 1;
    }
    println!("Rerouted {} of {} numbers into deque", i, n_v);
  });
  println!("Pool size #1: {}", pool.active_count());
  
  
  let (tx3, rx3) = channel();
  for _ in 0..n_v {
    let stealer = stealer.clone();
    let tx3 = tx3.clone();
    
    pool.execute(move|| {
      let seed;
      loop {
        match stealer.steal() {
          Empty => { sleep(1); }
          Data(seed_in) => {
            seed = seed_in;
            //println!("run_communicating.step2 steal: {:?}", seed);
            break;
          }
          Abort => { println!("Aborting..."); return; }
        }
        
      }
      
      //let rng : fn() -> f64 = rand::SeedableRng::from_seed(seed);
      //let r = pi_approx_random(versuche as u64, rng);
      let r = pi_approx_random(versuche as u64, rand::random::<f64>);
      tx3.send(r).unwrap();
    });
  }
  println!("Pool size #2: {}", pool.active_count());
  
  
  // SpinnLock
  let mut i : usize = n_v;
  let mut pi = (0, 0);
  while pool.active_count() > 0 || i > 0 {
    if let Ok(n) = rx3.try_recv() {
      pi = (pi.0+n.0, pi.1+n.1);
      //println!("run_mesh.step3 recv: {:?}", format_pi_approx(pi));
      i -= 1;
    } else {
      sleep(666);
      println!("Waiting in SpinnLock ... remaining numbers: {}, pool: {}", i, pool.active_count());
    }
  }
  
  println!("Pool is empty");
}


fn format_pi_approx((inside, tries) : (u64, u64)) -> f64 {
  (4 as f64) * (inside as f64) / (tries as f64)
}

fn sleep(ms : u64) {
  std::thread::sleep(Duration::from_millis(ms))
}
