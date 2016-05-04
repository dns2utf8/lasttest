const TEST_TASKS  : usize = 128_00;
const VERSUCHE    : u64   = 42_000;


extern crate threadpool;
extern crate rand;
extern crate random;
extern crate crossbeam;
extern crate num_cpus;
extern crate docopt;
extern crate rustc_serialize;


use threadpool::ThreadPool;
use crossbeam::sync::chase_lev;
use docopt::Docopt;
use std::sync::mpsc::channel;
use std::time::Instant;

const USAGE: &'static str = "
Naval Fate.

Usage:
  lasttest (all | static | communicating | chain | flood | mesh)
  lasttest (-h | --help)
  lasttest --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --speed=<kn>  Speed in knots [default: 10].
  --moored      Moored (anchored) mine.
  --drifting    Drifting mine.
";

#[derive(Debug, RustcDecodable)]
struct Args {
  cmd_all: bool,
  cmd_static: bool,
  cmd_communicating: bool,
  cmd_chain: bool,
  cmd_flood: bool,
  cmd_mesh: bool,
}

fn validate_args(a : &mut Args) {
  if a.cmd_all {
    a.cmd_static = true;
    a.cmd_communicating = true;
    a.cmd_chain = true;
    a.cmd_flood = true;
    a.cmd_mesh = true;
  }
}

fn main() {
  let mut args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
  validate_args(&mut args);
  println!("{:?}", args);
  let num_threads = num_cpus::get();
  
  println!("Hello, lasttest!\n\nnum_threads: {}\nTEST_TASKS: {}\nVERSUCHE: {}", num_threads, TEST_TASKS, VERSUCHE);
  
  let pool = ThreadPool::new(num_threads);
  
  if args.cmd_static {
    println!("\nStatic started...");
    let start = Instant::now();
    run_static(&pool);
    let duration = start.elapsed();
    println!("Static finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_communicating {
    println!("\nCommunicating started...");
    let start = Instant::now();
    run_communicating(&pool);
    let duration = start.elapsed();
    println!("Communicating finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_chain {
    println!("\nChain started...");
    let start = Instant::now();
    run_chain(&pool);
    let duration = start.elapsed();
    println!("Chain finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_flood {
    println!("\nFlood started...");
    let start = Instant::now();
    run_flood(&pool);
    let duration = start.elapsed();
    println!("Flood finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
  
  if args.cmd_mesh {
    println!("\nMesh started...");
    let start = Instant::now();
    run_mesh(&pool);
    let duration = start.elapsed();
    println!("Mesh finished <=> pool.active_count() = {} // duration = {:?}\n", pool.active_count(), duration);
  }
}

fn run_static(pool : &ThreadPool) {
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

fn run_communicating(pool : &ThreadPool) {
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
  
  println!("Pi: {}", format_pi_approx((inside, outside)));
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


fn run_chain(pool : &ThreadPool) {
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


fn run_flood(pool: &ThreadPool) {
  let (t_inside, r_inside) = channel();
  let (t_outside, r_outside) = channel();
  let (t, r) = channel();
  
  pool.execute(move || {
    let mut inside = 0;
    for _ in 0..TEST_TASKS {
      loop {
        match r_inside.recv().unwrap() {
          Some(n) => inside += n,
          None => break
        }
      }
    }
    
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
  
  let mut outside = 0;
  for _ in 0..TEST_TASKS {
    loop {
      match r_outside.recv().unwrap() {
        Some(n) => outside += n,
        None => break
      }
    }
  }
  let inside = r.recv().unwrap();
  
  println!("flood.pi: {}", format_pi_approx( (inside, inside+outside) ));
}


fn run_mesh(pool : &ThreadPool) {
  let versuche = VERSUCHE;
  let v : Vec<usize> = vec![23, 42, 666, 521, 8192, 0x056b, 0xe660, 0x4c74, 0x8ca5, 0xa224, 0x2b36, 0x9d11, 0x26ab, 0xc2d2, 0xbd39, 0x1fd3, 0x55b4, 0x168f, 0xff9b, 0x3ee2, 0x0342];
  let n_v = v.len();
  
  let (tx, rx) = channel();
  
  for b in v {
    let sender = tx.clone();
    pool.execute(move || {
      let h = b.wrapping_mul(0x9E3779B97F4A7C15);
      
      if h == 0 {
        sender.send(format!("Hash of {} is 0", b)).unwrap();
      } else {
        for f in 2..10000000 {
          let r = b.wrapping_mul(f);
          if r == 0 { // should never be true, but can not be known at compile time
            sender.send(format!("{} times {} is 0", b, f)).is_ok();
          }
        }
      }
    });
  }
  
  let (mut worker, stealer) = chase_lev::deque();
  
  pool.execute(move || {
    for _ in 0..n_v {
      worker.push(rx.recv().unwrap());
    }
  });
  
  let (tx3, rx3) = channel();
  //let q = ms_queue::MsQueue::new();
  for _ in 0..n_v {
    let stealer = stealer.clone();
    let tx3 = tx3.clone();
    
    pool.execute(move|| {
      let seed = stealer.steal();
      println!("run_communicating.step2 steal: {:?}", seed);
      
      /*{
        match stealer.steal() {
          Ok(seed_in) => {
            seed = seed_in;
          },
          Err(e) => {
            println!("run_communicating.step2 error: {:?}", e);
            return;
          }
        }
      }*/
      
      //let rng : rand::StdRng = rand::SeedableRng::from_seed(seed);
      let r = pi_approx_random(versuche as u64, rand::random::<f64>);
      tx3.send(r).unwrap();
    });
  }
  
  pool.execute(move|| {
    match rx3.recv() {
      Ok(r) => { println!("run_communicating.step3 ok: {:?}", r); },
      Err(e) => {
        println!("run_communicating.step3 error: {:?}", e);
      }
    }
  });
  
  // SpinnLock
  let mut i : usize = 0;
  let mut j : usize = 0;
  while pool.active_count() > 0 {
    i += 1;
    if i > 10_000_000 {
      println!("Waiting in SpinnLock ... {}, pool: {}", j, pool.active_count());
      i = 0;
      j += 1;
    }
  }
}


fn format_pi_approx((inside, tries) : (u64, u64)) -> f64 {
  (4 as f64) * (inside as f64) / (tries as f64)
}

