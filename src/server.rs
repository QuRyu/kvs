use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use serde_json::Deserializer;
use slog::{debug, info, Logger};

use crate::{KvsEngine, Request, Response, Result};

/// Kvs Server.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
    logger: Logger,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a new KvsServer.
    pub fn new(engine: E, logger: Logger) -> Self {
        KvsServer { engine, logger }
    }

    /// Start KvsServer to serve incoming requests.
    pub fn run(&mut self, addr: &SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        info!(self.logger, "Bind to address {:?}", addr);

        for stream in listener.incoming() {
            self.serve(stream?)?;
        }

        Ok(())
    }

    fn serve(&mut self, stream: TcpStream) -> Result<()> {
        let peer_addr = stream.peer_addr()?;
        let reader = BufReader::new(stream.try_clone()?);
        let mut writer = BufWriter::new(stream);

        let reader = Deserializer::from_reader(reader).into_iter::<Request>();

        macro_rules! send_resp {
            ( $op: expr, $resp: expr ) => {
                let rep = $resp;
                serde_json::to_writer(&mut writer, &rep)?;
                writer.flush()?;
                debug!(
                    self.logger,
                    "{} response sent to {}: {:?}", $op, peer_addr, rep
                );
            };
        }

        for req in reader {
            match req? {
                Request::Set { key, value } => {
                    let resp = match self.engine.set(key, value) {
                        Ok(_) => Response::Ok(None),
                        Err(e) => Response::Err(e.to_string()),
                    };

                    serde_json::to_writer(&mut writer, &resp)?;
                    writer.flush()?;
                    debug!(
                        self.logger,
                        "Set response sent to {}: {:?}", peer_addr, resp
                    );
                }

                Request::Get { key } => {
                    send_resp!("Get", {
                        match self.engine.get(key) {
                            Ok(v) => Response::Ok(v),
                            Err(e) => Response::Err(e.to_string()),
                        }
                    });
                }

                Request::Remove { key } => {
                    send_resp!("Remove", {
                        match self.engine.remove(key) {
                            Ok(_) => Response::Ok(None),
                            Err(e) => Response::Err(e.to_string()),
                        }
                    });
                }
            }
        }

        Ok(())
    }
}
