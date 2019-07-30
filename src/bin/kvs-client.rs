use std::net::SocketAddr;

use structopt::StructOpt;

use kvs::{Result, KvsClient};

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


    match cmd { 
        Command::GET { key, addr } => { 
            let mut client = KvsClient::connect(&addr)?;
            client.get(key)?;
        }
        
        Command::SET { key, value, addr } => {
            let mut client = KvsClient::connect(&addr)?;
            client.set(key, value)?;
        }

        Command::REMOVE { key, addr } => { 
            let mut client = KvsClient::connect(&addr)?;
            client.remove(key)?;
        }
    }

    Ok(())

}
