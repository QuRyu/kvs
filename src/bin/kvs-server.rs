use std::net::{SocketAddr};
use std::path::{Path};
use std::process::exit; 

use clap::arg_enum;
use structopt::StructOpt;
use slog::{info, debug, error, warn, Logger};
use sloggers::Build; 
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity; 

use kvs::{KvsEngine, KvStore, Result, KvsServer, SledKvsEngine};

const DEFAULT_ENGINE: Engine = Engine::kvs;

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
    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    #[allow(non_camel_case_types)]
    enum Engine {
        kvs, sled
    }
}

fn main() -> Result<()> {
    let mut cmd = Command::from_args();
    let dir = std::env::current_dir()?;


    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build()?;
    let logger_out = logger.clone();

    let state = detect_engine(&dir, logger.clone()).and_then(move |eng| {
        if cmd.engine.is_none() { 
            cmd.engine = eng;
        } 
        
        if eng.is_some() && cmd.engine != eng {
            error!(logger, "Wrong engine!");
            exit(1);
        } 

        run(cmd, logger.clone())
    });


    if let Err(e) = state {
        error!(logger_out, "{}", e);
        exit(1);
    }

    Ok(())
}

fn run(cmd: Command, logger: Logger) -> Result<()> { 
    let dir = std::env::current_dir()?;
    let engine = cmd.engine.unwrap_or(DEFAULT_ENGINE);

    info!(logger, "Server version number {}", env!("CARGO_PKG_VERSION"));
    info!(logger, "Config: IP address {}, storage engine {:?}", cmd.addr, cmd.engine);

    std::fs::write(dir.join("engine"), format!("{}", engine))?;

    match engine { 
        Engine::kvs => run_with_engine(KvStore::open(dir)?, &cmd.addr, logger),
        Engine::sled => run_with_engine(SledKvsEngine::new(dir)?, &cmd.addr, logger),
    } 
}

fn run_with_engine<E: KvsEngine>(engine: E, addr: &SocketAddr, logger: Logger) -> Result<()> {
    let mut server = KvsServer::new(engine, logger);
    server.run(addr)
}


fn detect_engine(path: &Path, logger: Logger) -> Result<Option<Engine>> {
    let engine_path = path.join("engine");

    if !engine_path.exists() {
        debug!(logger, "No engine exists");
        return Ok(None);
    }

    match std::fs::read_to_string(engine_path)?.parse() {
        Ok(v) => return Ok(Some(v)), 
        Err(e) => {
            warn!(logger, "Content of engine file invalid: {}", e);
            Ok(None)
        }
    }

}
