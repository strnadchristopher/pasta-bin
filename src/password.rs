use rand;

#[derive(Debug)]
struct SecurePassword {
    hash_key: i32,
    hashed_password: String,
}
impl SecurePassword{
    fn new(unhashed_password: &str) -> SecurePassword {
        let hash_key = rand::random::<i32>();
        let hashed_password = SecurePassword::hash_password(unhashed_password, hash_key);
        SecurePassword {
            hash_key,
            hashed_password
        }
    }
    fn display(&self) {
        println!("Hash key: {}", self.hash_key);
        println!("Hashed password: {}", self.hashed_password);
    }
    fn hash_password(unhashed_password: &str, hash_key: i32) -> String {
        let mut hashed_password = String::new();
        for c in unhashed_password.chars() {
            let hashed_char = c as i32 + hash_key;
            hashed_password.push(hashed_char as u8 as char);
        }
        hashed_password
    }
    fn unhash_password(secure_password: SecurePassword) -> String{
        let mut unhashed_password = String::new();
        for c in secure_password.hashed_password.chars() {
            let unhashed_char = c as i32 - secure_password.hash_key;
            unhashed_password.push(unhashed_char as u8 as char);
        }
        unhashed_password
    }

}

fn main() {
    let secure_password = SecurePassword::new("password");
    secure_password.display();
    let unhashed_password = SecurePassword::unhash_password(secure_password);
    println!("Unhashed password: {}", unhashed_password);
}

