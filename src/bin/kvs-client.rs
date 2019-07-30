use std::io::Write;
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use kvs::{KvStore, KvsError, Result};

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-client")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    SET {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,

        #[structopt(name = "VALUE", help = "A string value")]
        value: String,

        #[structopt(
            long,
            default_value = "127.0.0.1:4000",
            value_name = "IP-PORT",
            help = "Specify socket address to bound to",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },

    #[structopt(name = "remove", about = "Remove a given string key")]
    REMOVE {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,

        #[structopt(
            long,
            default_value = "127.0.0.1:4000",
            value_name = "IP-PORT",
            help = "Specify socket address to bound to",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },

    #[structopt(name = "get", about = "Get the string value of a given string key")]
    GET {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,

        #[structopt(
            long,
            default_value = "127.0.0.1:4000",
            value_name = "IP-PORT",
            help = "Specify socket address to bound to",
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}

fn main() -> Result<()> {
    let cmd = Command::from_args();

    //let listender = TcpListener::bind(cmd.addr)?;

    match cmd { 
        Command::GET { addr, .. } => { 
            let mut stream = TcpStream::connect(addr)?;
            stream.write(b"get")?;
            stream.flush()?;
        }
        
        Command::SET { addr, .. } => {
            let mut stream = TcpStream::connect(addr)?;
            stream.write(b"set")?;
            stream.flush()?;
        }

        Command::REMOVE { addr, .. } => { 
            let mut stream = TcpStream::connect(addr)?;
            stream.write(b"remove")?;
            stream.flush()?;

        }
    }

    Ok(())

}
