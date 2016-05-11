const TEST_TASKS  : usize = 128_00;
const VERSUCHE    : u64   = 42_000;


extern crate threadpool;
extern crate rand;
//extern crate random;  
extern crate crossbeam;
extern crate num_cpus;
extern crate docopt;
extern crate rustc_serialize;

mod local;

use local::{run_chain, run_communicating, run_flood, run_mesh, run_static};
use std::time::{Instant};
use threadpool::ThreadPool;
use docopt::Docopt;


const USAGE: &'static str = "
lasttest is a load generator written in rust optimized for NUMA systems.

Usage:
  lasttest <module> <command>
  lasttest local all
  lasttest local [static] [communicating] [chain] [flood] [mesh]
  lasttest net server [<port>]
  lasttest net client <servername> [<port>]
  lasttest (-h | --help)

Module:
  -h --help     Show this screen.

The tests are split into two groups:
  local     running on one machine
  net       running on two machines and transfer load betweend them
";

#[derive(Debug,PartialEq,RustcDecodable)]
struct Args {
  // local options
  cmd_local: bool,
  cmd_all: bool,
  cmd_static: bool,
  cmd_communicating: bool,
  cmd_chain: bool,
  cmd_flood: bool,
  cmd_mesh: bool,
  
  // net options
  cmd_net: bool,
  cmd_server: bool,
  cmd_client: bool,
  arg_port: u16,
  arg_servername: String,
}

fn validate_args(a : Args) -> Result<Args,String>  {
  println!("{:?}", a);
  let empty = Args {
    cmd_local: true,
    cmd_all: false,
    cmd_static: false,
    cmd_communicating: false,
    cmd_chain: false,
    cmd_flood: false,
    cmd_mesh: false,
    
    cmd_net: false,
    cmd_server: false,
    cmd_client: false,
    arg_port: 0,
    arg_servername: "".into(),
  };
  
  if a == empty {
    return Err("you must pick at least one target".into());
  }
  
  Ok(if a.cmd_all {
      Args {
        cmd_local: true,
        cmd_all: true,
        cmd_static: true,
        cmd_communicating: true,
        cmd_chain: true,
        cmd_flood: true,
        cmd_mesh: true,
        
        cmd_net: false,
        cmd_server: false,
        cmd_client: false,
        arg_port: 0,
        arg_servername: "".into(),
      }
    } else {
      a
    })
}

fn main() {
  let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
  let args: Args = validate_args(args).unwrap();
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

