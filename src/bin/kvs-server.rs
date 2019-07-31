use std::net::{SocketAddr};

use clap::arg_enum;
use structopt::StructOpt;
use slog::{info};
use sloggers::Build; 
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity; 

use kvs::{KvStore, Result, KvsServer};

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
    #[allow(non_camel_case_types)]
    enum Engine {
        kvs, sled
    }
}

fn main() -> Result<()> {
    let cmd = Command::from_args();

    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);

    let logger = builder.build()?;
    info!(logger, "Server version number {}", env!("CARGO_PKG_VERSION"));
    info!(logger, "Config: IP address {}, storage engine {:?}", cmd.addr, cmd.engine);

    let engine = KvStore::open(std::env::current_dir()?)?;

    let mut server = KvsServer::new(engine, logger.clone());
    server.run(cmd.addr)?;
    

    Ok(())
}
