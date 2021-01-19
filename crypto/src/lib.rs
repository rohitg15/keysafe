
use openssl::pkcs5::pbkdf2_hmac;
use openssl::hash::MessageDigest;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//
//  constants
//
const MAX_PBKDF2_ITERATIONS: usize        = 10000;
const PBKDF2_MD_DIGEST_SIZE_BYTES: usize  = 32;

pub fn get_key_from_pwd(password: &String, salt: &[u8]) -> [u8; PBKDF2_MD_DIGEST_SIZE_BYTES] {
    let mut key = [0u8; PBKDF2_MD_DIGEST_SIZE_BYTES];
    match pbkdf2_hmac(password.as_bytes(), salt, MAX_PBKDF2_ITERATIONS, MessageDigest::sha256(), &mut key) {
        Err(e) => panic!("pbkdf2_hmac returned error {:?}", e),
        _ => key,
    }
}

pub fn test() {
    println!("hello crypto world!");
}
