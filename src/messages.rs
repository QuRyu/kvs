
use serde::{Serialize, Deserialize}; 

#[derive(Debug, Serialize, Deserialize)]
/// Request sent by client to server.
pub enum Request { 
    /// Set a given string key to a string value. 
    Set { 
        /// A string key. 
        key: String, 
        /// A string value.
        value: String 
    }, 

    /// Get a string value given a string key. 
    Get { 
        /// A string key.
        key: String 
    }, 

    /// Remove a string value given a string key. 
    Remove { 
        /// A string key. 
        key: String 
    },
}


#[derive(Debug, Serialize, Deserialize)]
/// Response from the kvs server given a request.
pub enum Response { 
    /// Request is processed successfully. 
    /// If the request is `Request::Get`, the requested string value is returned as `Some(value)`.
    /// Otherwise, `None` is returned. 
    Ok(Option<String>),

    /// Request is not processed successfully and the cause is returned. 
    Err(String),
}


 
