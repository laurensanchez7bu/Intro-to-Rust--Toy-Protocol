// This is in a separate file so that I can automatically test student code against my own
// unit tests (by replacing this file).

use super::*;

#[cfg(test)]
mod test {
    use crate::User;
    use crate::UserError;

    fn create_valid_user() -> User {
        let user = User {
            id: 1,
            access_level: 12,
            active: true,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        user
    }

    /// Serialize a valid User struct
    #[test]
    fn test_serialize_valid() {
        let user = create_valid_user();
        let serialized_user = user.serialize().unwrap();
        assert_eq!(serialized_user.len(), 27);
    }

    /// Deserialize a valid user packet
    #[test]
    fn test_deserialize_valid() {
        let user = create_valid_user();
        let serialized_user = user.serialize().unwrap();
        let deserialized_user = super::User::deserialize(&serialized_user).unwrap();
        assert_eq!(user, deserialized_user);
    }

    /// Serialize a user with an invalid access level
    #[test]
    fn test_serialize_user_invalid_access_level_too_large() {
        let mut user = create_valid_user();
        user.access_level = 40;
        let result = user.serialize();
        assert!(matches!(result, Err(UserError::FieldTooLarge)));
    }

    /// Serialize a user with an invalid name length
    #[test]
    fn test_serialize_user_invalid_name_too_large() {
        let mut user = create_valid_user();
        user.name = "A".repeat(300);
        let result = user.serialize();
        assert!(matches!(result, Err(UserError::FieldTooLarge)));
    }

    /// Serialize a user with an invalid email length
    #[test]
    fn test_serialize_user_invalid_email_too_large() {
        let mut user = create_valid_user();
        user.email = "A".repeat(300);
        let result = user.serialize();
        assert!(matches!(result, Err(UserError::FieldTooLarge)));
    }

    /// Serialize a user with an invalid name (empty)
    #[test]
    fn test_serialize_user_invalid_name_field_too_small() {
        let mut user = create_valid_user();
        user.name = "".to_string();
        let result = user.serialize();
        assert!(matches!(result, Err(UserError::FieldTooSmall)));
    }

    /// Serialize a user with an invalid email (empty)
    #[test]
    fn test_serialize_user_invalid_email__field_too_small() {
        let mut user = create_valid_user();
        user.email = "".to_string();
        let result = user.serialize();
        assert!(matches!(result, Err(UserError::FieldTooSmall)));
    }

    /// Deserialize a user with an invalid header (too short)
    #[test]
    fn test_deserialize_user_invalid_header_malformed_body() {
        let buf: Vec<u8> = vec![0x00, 0x01];
        let result = super::User::deserialize(&buf);
        assert!(matches!(result, Err(UserError::MalformedBody)));
    }

    /// Deserialize a user with invalid name (empty)
    #[test]
    fn test_deserialize_user_invalid_name_malformed_body() {
        let user = create_valid_user();
        let mut buf = user.serialize().unwrap();

        buf[3] = 0;
        let result = super::User::deserialize(&buf);
        assert!(matches!(result, Err(UserError::MalformedBody)));
    }

    /// Deserialize a user with invalid email (empty)
    #[test]
    fn test_deserialize_user_invalid_email_malformed_body() {
        let user = create_valid_user();
        let mut buf = user.serialize().unwrap();

        let namelength = buf[3] as usize;
        let emaillengthindex = 4 + namelength;

        buf[emaillengthindex] = 0;
        let result = super::User::deserialize(&buf);
        assert!(matches!(result, Err(UserError::MalformedBody)));
    }

    /// Deserialize a user with invalid length of body (too long)
    #[test]
    fn test_deserialize_user_invalid_body_malformed_body() {
        let user = create_valid_user();
        let mut buf = user.serialize().unwrap();
        buf.push(0xAA);
        let result = super::User::deserialize(&buf);
        assert!(matches!(result, Err(UserError::MalformedBody)));
    }
}
