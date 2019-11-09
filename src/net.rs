use threadpool::ThreadPool;

use mpi;
use mpi::traits::*;
use mpi::request::WaitGuard;


pub fn main_net(args : &::Args, pool : &ThreadPool) {
  if args.cmd_server {
    run_server(args, pool);
  } else {
    run_client(args, pool);
  }
}

fn run_client(args : &::Args, pool : &ThreadPool) {
  println!("client: {:?}", args);
  let port = get_default_port(&args);
  let servername = &args.arg_servername;
}

fn run_server(args : &::Args, pool : &ThreadPool) {
  let port = get_default_port(args);
  println!("server mode: port({})", port);

  let universe = mpi::initialize().unwrap();
  let world = universe.world();
  let size = world.size();
  let rank = world.rank();
  println!("size: {}; rank: {}", size, rank);

  let next_rank = if rank + 1 < size { rank + 1 } else { 0 };
  let previous_rank = if rank - 1 >= 0 { rank - 1 } else { size - 1 };

  let msg = vec![rank , 2 * rank, 4 * rank];
  let _sreq = WaitGuard::from(world.process_at_rank(next_rank).immediate_send(&msg[..]));

  let (msg, status) = world.any_process().receive_vec();

  println!("Process {} got message {:?}.\nStatus is: {:?}", rank, msg, status);
  let x = status.source_rank();
  assert_eq!(x, previous_rank);
  assert_eq!(vec![x, 2 * x, 4 * x], msg);

  let root_rank = 0;
  let root_process = world.process_at_rank(root_rank);

  let mut a;
  if world.rank() == root_rank {
      a = vec![2, 4, 8, 16];
      println!("Root broadcasting value: {:?}.", &a[..]);
  } else {
      a = vec![0; 4];
  }
  root_process.broadcast_into(&mut a[..]);
  println!("Rank {} received value: {:?}.", world.rank(), &a[..]);
  assert_eq!(&a[..], &[2, 4, 8, 16]);
}


fn get_default_port(args : &::Args) -> u16 {
  if args.arg_port == 0 { 5278 } else { args.arg_port }
}
