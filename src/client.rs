use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

use serde::Deserialize;
use serde_json::de::IoRead;
use serde_json::Deserializer;

use crate::{KvsError, Request, Response, Result};

/// Kvs client.
pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to the given socket address.
    ///
    /// # Error
    ///
    /// Return an error if the connection fails.
    pub fn connect(addr: &SocketAddr) -> Result<KvsClient> {
        let writer = TcpStream::connect(addr)?;
        let reader = writer.try_clone()?;

        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(reader)),
            writer: BufWriter::new(writer),
        })
    }

    /// Set the given string value to the given string key by sending
    /// a request to the kvs server.
    ///
    /// # Error
    ///
    /// Return an error if the network fails or if the request is not
    /// processed successfully on the server side.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let req = Request::Set { key, value };
        serde_json::to_writer(&mut self.writer, &req)?;
        self.writer.flush()?;

        let response = Response::deserialize(&mut self.reader)?;
        match response {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(KvsError::ServerError(e)),
        }
    }

    /// Get the string value from the given string key by sending a request
    /// to the kvs server.
    ///
    /// # Error
    ///
    /// Return an error if the network fails or if the request is not
    /// processed successfully on the server side.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;

        let response = Response::deserialize(&mut self.reader)?;
        match response {
            Response::Ok(v) => Ok(v),
            Response::Err(e) => Err(KvsError::ServerError(e)),
        }
    }

    /// Remove the given string key by sending a request to the kvs server.
    ///
    /// # Error
    ///
    /// Return an error if the network fails or if the request is not
    /// processed successfully on the server side.
    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;

        let response = Response::deserialize(&mut self.reader)?;
        match response {
            Response::Ok(_) => Ok(()),
            Response::Err(e) => Err(KvsError::ServerError(e)),
        }
    }
}
