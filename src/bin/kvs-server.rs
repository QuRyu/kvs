use std::io::{Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

use kvs::{KvStore, KvsError, Result};

use clap::arg_enum;
use structopt::StructOpt;
use slog::{info, debug};
use sloggers::Build; 
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity; 

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

fn main() -> Result<()> {
    let cmd = Command::from_args();

    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);

    let logger = builder.build()?;
    info!(logger, "Server version number {}", env!("CARGO_PKG_VERSION"));
    info!(logger, "Config: IP address {}, storage engine {:?}", cmd.addr, cmd.engine);

    let listener = TcpListener::bind(cmd.addr)?;

    let mut buf = String::new();
    for stream in listener.incoming() { 
        let mut stream = stream?;
        debug!(logger, "new connection {:?}", stream);

        stream.read_to_string(&mut buf)?;
        debug!(logger, "message: {}", buf);
        
        buf.clear();
    }

    Ok(())
}
