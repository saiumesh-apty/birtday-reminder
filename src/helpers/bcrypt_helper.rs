use bcrypt::{ hash, verify };

const DEFAULT_PASSWORD: &'static str = "apty@123";
const PASSWORD_COST: u32 = 4;

pub fn get_default_password() -> String {
    hash(DEFAULT_PASSWORD, PASSWORD_COST).unwrap()
}

pub fn hash_password(password: &String) -> String {
    hash(password, PASSWORD_COST).unwrap()
}

pub fn verify_password(plain_password: &String, hashed_password: &String) -> bool {
    verify(plain_password, hashed_password).unwrap()
}