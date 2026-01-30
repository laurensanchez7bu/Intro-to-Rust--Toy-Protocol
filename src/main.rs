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

    #[error("The packet field is too small to be serialized.")]
    FieldTooSmall,

    #[error("The body of the packet has invalid data.")]
    MalformedBody,

    #[error(transparent)]
    IoError(#[from] io::Error),
}

impl User {
    // Serialize the User struct to a Vec<u8> in big endian format
    fn serialize(&self) -> Result<Vec<u8>, UserError> {
        if self.access_level > 31 {
            return Err(UserError::FieldTooLarge);
        }

        let namebytes = self.name.as_bytes();
        let emailbytes = self.email.as_bytes();

        if namebytes.is_empty() || emailbytes.is_empty() {
            return Err(UserError::FieldTooSmall);
        }

        if namebytes.len() > u8::MAX as usize || emailbytes.len() > u8::MAX as usize {
            return Err(UserError::FieldTooLarge);
        }

        let mut output = Vec::<u8>::new();
        let activebit: u8 = if self.active { 1 } else { 0 };
        let headerbyte: u8 = (self.access_level << 3) | (activebit << 2);

        output.write_u16::<BigEndian>(self.id)?;
        output.write_u8(headerbyte)?;

        output.write_u8(namebytes.len() as u8)?;
        output.extend_from_slice(namebytes);
        output.write_u8(emailbytes.len() as u8)?;
        output.extend_from_slice(emailbytes);

        Ok(output)
    }

    /// Deserialize a Vec<u8> (in big endian format) back to a User struct. Returns an error
    /// if the packet is invalid.
    fn deserialize(buf: &[u8]) -> Result<User, UserError> {
        if buf.len() < 3 {
            return Err(UserError::MalformedBody);
        }

        let mut cursor = Cursor::new(buf);

        let map_eof = |e: io::Error| {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                UserError::MalformedBody
            } else {
                UserError::IoError(e)
            }
        };

        let id = cursor.read_u16::<BigEndian>().map_err(map_eof)?;

        let b = cursor.read_u8().map_err(map_eof)?;
        let access_level = b >> 3;
        let active = ((b >> 2) & 0b1) == 1;

        let namelength = cursor.read_u8().map_err(map_eof)? as usize;
        if namelength == 0 {
            return Err(UserError::MalformedBody);
        }
        let mut namebuff = vec![0u8; namelength];
        cursor.read_exact(&mut namebuff).map_err(map_eof)?;
        let name = String::from_utf8(namebuff).map_err(|_| UserError::MalformedBody)?;

        let emaillength = cursor.read_u8().map_err(map_eof)? as usize;
        if emaillength == 0 {
            return Err(UserError::MalformedBody);
        }
        let mut emailbuff = vec![0u8; emaillength];
        cursor.read_exact(&mut emailbuff).map_err(map_eof)?;
        let email = String::from_utf8(emailbuff).map_err(|_| UserError::MalformedBody)?;

        if (cursor.position() as usize) != buf.len() {
            return Err(UserError::MalformedBody);
        }

        Ok(User { id, access_level, active, name, email })
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
