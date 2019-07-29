use std::net::SocketAddr;

use clap::arg_enum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Command {
    #[structopt(
        long,
        default_value = "127.0.0.1:4000",
        value_name = "IP-PORT",
        help = "Specify socket address to bound to",
        parse(try_from_str)
    )]
    addr: SocketAddr,

    #[structopt(
        long,
        value_name = "ENGINE-NAME",
        help = "Specify which engine to use",
        raw(possible_values = "&Engine::variants()")
    )]
    engine: Option<Engine>,
}

arg_enum! {
    #[derive(Eq, PartialEq, Debug)]
    enum Engine {
        Kvs, Sled
    }
}

fn main() {
    let _cmd = Command::from_args();
}
