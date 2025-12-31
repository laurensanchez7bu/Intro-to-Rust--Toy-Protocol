/**************************************
*
* Header: 24 bits
*
* 0-15: id (u16) | 16-20: access_level (5-bit uint) | 21: active (bool) | 22-23: zero padding
*
* ---------------------------
*
* Body: variable length
*
* 8 bits: name_length | name (utf8 byte array) | 8 bits: email_length | email (utf8 byte array)
***************************************/

/// The fundamental User struct
#[derive(PartialEq, Debug)]
struct User {
    id: u16,
    access_level: u8,
    active: bool,
    name: String,
    email: String,
}

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Cursor, Read};
use thiserror::Error;

/// Custom error type for serializing and deserializing User structs
#[derive(Error, Debug)]
pub enum UserError {
    #[error("The packet field is too long to be serialized.")]
    FieldTooLarge,

    #[error("The body of the packet has invalid data.")]
    MalformedBody,

    #[error(transparent)]
    IoError(#[from] io::Error),
}

impl User {
    // Serialize the User struct to a Vec<u8> in big endian format
    fn serialize(&self) -> Result<Vec<u8>, UserError> {
        /*
        TODO:
        Your code here
         */
    }

    /// Deserialize a Vec<u8> (in big endian format) back to a User struct. Returns an error
    /// if the packet is invalid.
    fn deserialize(buf: &[u8]) -> Result<User, UserError> {
        /*
        TODO:
        Your code here
         */
    }
}

fn main() -> Result<(), UserError> {
    // Create a User instance
    let user = User {
        id: 1,
        access_level: 12,
        active: true,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // Serialize the User instance to a String
    let serialized_user = user.serialize()?;
    println!("Serialized User: {:?}", serialized_user);

    // // Deserialize the String back to a User instance
    let deserialized_user = User::deserialize(&serialized_user)?;
    println!(
        "Deserialized User: id={}, name={}, email={}",
        deserialized_user.id, deserialized_user.name, deserialized_user.email
    );

    assert_eq!(
        user, deserialized_user,
        "Deserialized User is not equal to the original User"
    );

    Ok(())
}

/* Do not change anything below this line */
#[cfg(test)]
mod test;
