use std::io::{Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

use kvs::{KvStore, KvsError, Result, Request, Response};

use clap::arg_enum;
use structopt::StructOpt;
use slog::{info, debug};
use sloggers::Build; 
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity; 
use serde_json::{Deserializer};

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

        let reader = Deserializer::from_reader(stream).into_iter::<Request>();

        for re in reader { 
            match re? { 
                Request::Set {key, value } => {
                    info!(logger, "Request::Set {{ key {}, value {} }}", key, value);
                }

                Request::Get { key } => { 
                    info!(logger, "Request::Get {{ key {} }}", key);
                }

                Request::Remove { key } => { 
                    info!(logger, "Request::Remove {{ key {} }}", key);

                }
            }
        }

        
        buf.clear();
    }

    Ok(())
}
