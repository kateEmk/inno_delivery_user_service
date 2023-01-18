use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginResponse {
    pub user_logged_in: UserLoggedIn
}

#[derive(Debug, Serialize)]
pub struct UserLoginError {
    pub message: String,
    pub error: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLoggedIn {
    pub first_name: String,
    pub email: String,
    pub jwt: String,
    pub refresh_token: Option<String>
}
