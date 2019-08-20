use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUser {
    pub email: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DOB {
    pub user_id: i32,
    pub dob: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePassword {
    pub id: i32,
    pub password: String,
}