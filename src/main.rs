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
mod net;

use local::main_local;
use net::main_net;

use std::time::{Instant};
use threadpool::ThreadPool;
use docopt::Docopt;


const USAGE: &'static str = "
lasttest is a load generator written in rust optimized for NUMA systems.

Usage:
  lasttest [options] local all
  lasttest [options] local [static] [communicating] [chain] [flood] [mesh]
  lasttest [options] net server [<port>]
  lasttest [options] net client <servername> [<port>]
  lasttest (-h | --help)

Options:
  -h --help     Show this screen.
  --logical     Use all logical CPUs not just the pyisical

The tests are split into two groups:
  local     running on one machine
  net       running on two machines and transfer load betweend them
";

#[derive(Debug,PartialEq,RustcDecodable)]
pub struct Args {
  // global options
  flag_logical: bool,

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
  //println!("{:?}", a);
  let mut empty = Args {
    flag_logical: a.flag_logical,

    cmd_local: false,
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

  if a.cmd_local {
    empty.cmd_local = true;
  } else {
    empty.cmd_net = true;
  }
  
  if a == empty {
    return Err("you must pick at least one target".into());
  }

  
  Ok(if a.cmd_local && a.cmd_all {
      Args {
        flag_logical: a.flag_logical,

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
  let num_threads = if args.flag_logical {
    num_cpus::get()
  } else {
    num_cpus::get_physical()
  };
  
  println!("Hello, lasttest!\n\nnum_threads: {}\nTEST_TASKS: {}\nVERSUCHE: {}", num_threads, TEST_TASKS, VERSUCHE);
  
  let pool = ThreadPool::new_with_name("lasttest.local".into(), num_threads);
  
  let start = Instant::now();


  if args.cmd_local {
    main_local(&args, &pool);
  }
  
  if args.cmd_net {
    main_net(&args, &pool);
  }

  println!("\n Was running for {:?}\n", start.elapsed());
}

