use actix_web::{web, HttpResponse};
use crate::dbaccess::users::*;
use crate::errors::BlogAppError;
use crate::state::AppState;
use crate::models::users::CreateUser;

pub async fn post_new_user( new_user:web::Json<CreateUser>,app_state:web::Data<AppState>) -> Result<HttpResponse,BlogAppError> {
    
    post_new_user_db(&app_state.db,new_user.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))

}

pub async fn get_all_users(app_state:web::Data<AppState>)->Result<HttpResponse,BlogAppError> {
    get_all_user_db(&app_state.db)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}