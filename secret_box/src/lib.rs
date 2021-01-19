use crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct SecretBox {
    master_key: [u8; 32],
    username: String,
    password: String
}

impl SecretBox {

    fn get_master_key(pw: &String, salt: &[u8; 32]) -> [u8; 32] {
        return crypto::get_key_from_pwd(pw, salt);
    }

    pub fn new(username: String, password: String, salt: [u8; 32]) -> SecretBox {
        SecretBox {
            master_key: SecretBox::get_master_key(&password, &salt),
            username: username,
            password: password
        }
    }

    // pub fn protect(buf: Vec<u8>) -> Vec<u8> {

    // }

    // pub fn unprotect(buf: Vec<u8>) -> Vec<u8> {

    // }
}