use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct UserObj {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub is_super_admin: bool,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
    pub timestamp:String
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl From<web::Json<CreateUser>> for UserObj {
    fn from(new_tutor: web::Json<CreateUser>) -> Self {
        UserObj {
            user_id: Uuid::new_v4().to_string(),
            first_name: new_tutor.first_name.clone(),
            last_name: new_tutor.last_name.clone(),
            email: new_tutor.email.clone(),
            password: new_tutor.password.clone(),
            is_admin:false,
            is_super_admin:false
        }
    }
}
