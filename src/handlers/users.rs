use crate::{dbaccess::users::*, types::users::AdminStatus};
use crate::errors::BlogAppError;
use crate::models::users::CreateUser;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

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




pub async fn set_admin(
    app_state: web::Data<AppState>,
    admin: web::Query<AdminStatus>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, BlogAppError> {
    let user_id = path.into_inner();
     set_admin_db(&app_state.db,user_id, admin.status).await?;
    Ok(HttpResponse::Ok().json(format!("{:?} is now an admin",user_id)))
}


pub async fn set_super_admin(
    app_state: web::Data<AppState>,
    admin: web::Query<AdminStatus>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, BlogAppError> {
    let user_id = path.into_inner();
    set_super_admin_db(&app_state.db,user_id, admin.status).await?;
    Ok(HttpResponse::Ok().json(format!("{:?} is now an admin",user_id)))
}
