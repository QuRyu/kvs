use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;

use serde_json::Deserializer;
use slog::{debug, error, info, Logger};

use crate::{KvsEngine, Request, Response, Result, ThreadPool};

/// Kvs Server.
pub struct KvsServer<E: KvsEngine, P: ThreadPool> {
    engine: E,
    logger: Arc<Logger>,
    pool: P,
}

impl<E: KvsEngine, P: ThreadPool> KvsServer<E, P> {
    /// Create a new KvsServer.
    pub fn new(engine: E, logger: Logger, pool: P) -> Self {
        KvsServer {
            engine,
            logger: Arc::new(logger),
            pool,
        }
    }

    /// Start KvsServer to serve incoming requests.
    pub fn run(&mut self, addr: &SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        info!(self.logger, "Bind to address {:?}", addr);

        for stream in listener.incoming() {
            let logger = self.logger.clone();
            let logger_copy = self.logger.clone();
            let engine = self.engine.clone();

            self.pool.spawn(move || match stream {
                Ok(s) => {
                    if let Err(e) = serve(s, engine, logger) {
                        error!(logger_copy, "Error processing incoming request: {}", e);
                    }
                }
                Err(e) => error!(logger, "Connection failed: {}", e),
            });
        }

        Ok(())
    }
}

fn serve<E: KvsEngine>(stream: TcpStream, engine: E, logger: Arc<Logger>) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    let reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);

    let reader = Deserializer::from_reader(reader).into_iter::<Request>();

    macro_rules! send_resp {
        ( $op: expr, $resp: expr ) => {
            let rep = $resp;
            serde_json::to_writer(&mut writer, &rep)?;
            writer.flush()?;
            debug!(logger, "{} response sent to {}: {:?}", $op, peer_addr, rep);
        };
    }

    for req in reader {
        match req? {
            Request::Set { key, value } => {
                send_resp!("Set", {
                    match engine.set(key, value) {
                        Ok(_) => Response::Ok(None),
                        Err(e) => Response::Err(e.to_string()),
                    }
                });
            }

            Request::Get { key } => {
                send_resp!("Get", {
                    match engine.get(key) {
                        Ok(v) => Response::Ok(v),
                        Err(e) => Response::Err(e.to_string()),
                    }
                });
            }

            Request::Remove { key } => {
                send_resp!("Remove", {
                    match engine.remove(key) {
                        Ok(_) => Response::Ok(None),
                        Err(e) => Response::Err(e.to_string()),
                    }
                });
            }
        }
    }

    Ok(())
}
