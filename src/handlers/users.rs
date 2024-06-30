use crate::dbaccess::users::*;
use crate::errors::BlogAppError;
use crate::models::users::CreateUser;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn post_new_user(
    new_user: web::Json<CreateUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, BlogAppError> {
    post_new_user_db(&app_state.db, new_user.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn get_all_users(app_state: web::Data<AppState>) -> Result<HttpResponse, BlogAppError> {
    get_all_user_db(&app_state.db)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}


#[derive(Deserialize,Serialize, Debug, Clone)]
struct AdminStatus {
    status: bool,
}


pub async fn set_admin(
    app_state: web::Data<AppState>,
    admin: web::Query<AdminStatus>,
    path: web::Path<String>,
) -> Result<HttpResponse, BlogAppError> {
    let user_id = path.into_inner();
     set_admin_db(&app_state.db,user_id.clone(), admin.status).await?;
    Ok(HttpResponse::Ok().json(format!("{:?} is now an admin",user_id)))
}


pub async fn set_super_admin(
    app_state: web::Data<AppState>,
    admin: web::Query<AdminStatus>,
    path: web::Path<String>,
) -> Result<HttpResponse, BlogAppError> {
    let user_id = path.into_inner();
    set_super_admin_db(&app_state.db,user_id.clone(), admin.status).await?;
    Ok(HttpResponse::Ok().json(format!("{:?} is now an admin",user_id)))
}
