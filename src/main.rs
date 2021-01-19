use secret_box::SecretBox;

fn main() {
    let salt = [0u8; 32];
    let sb = SecretBox::new("morpheus".to_string(), "abcdef123456".to_string(), salt);
    println!("Hello, world! {:?}", sb);
}
