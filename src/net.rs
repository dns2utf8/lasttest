use threadpool::ThreadPool;


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
  let ref servername = args.arg_servername;
}

fn run_server(args : &::Args, pool : &ThreadPool) {
  let port = get_default_port(&args);
  println!("server mode: port({})", port);
}


fn get_default_port(args : &::Args) -> u16 {
  if args.arg_port == 0 { 5278 } else { args.arg_port.clone() }
}
