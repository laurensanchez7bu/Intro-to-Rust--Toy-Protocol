// This is in a separate file so that I can automatically test student code against my own
// unit tests (by replacing this file).

use super::*;

#[cfg(test)]
mod test {
    use crate::User;

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

    /*
    TODO:
    Add more tests here, and document what each one does. You should cover all cases where you are
    trying to serialize or deserialize invalid data.
     */
}
