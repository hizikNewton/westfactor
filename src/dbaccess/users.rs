use crate::{
    errors::BlogAppError,
    models::users::{User, UserObj},
};
use sqlx::{self, PgPool};
use uuid::Uuid;

pub async fn post_new_user_db(pool: &PgPool, new_user: UserObj) -> Result<User, BlogAppError> {
    let user_row = sqlx::query!(
        "insert into users (user_id, first_name, last_name,email,password,isadmin) values ($1,$2,$3,$4,$5,$6) returning
        user_id, first_name, last_name,email,isadmin,time_stamp",new_user.user_id,new_user.first_name,new_user.last_name,new_user.email,new_user.password,new_user.is_admin)
    .fetch_one(pool)
    .await?;
    Ok(User {
        user_id: user_row.user_id,
        first_name: user_row.first_name,
        last_name: user_row.last_name,
        email: user_row.email,
        is_admin: user_row.isadmin.expect("admin should be set"),
        timestamp: user_row.time_stamp.unwrap().to_string(),
    })
}

pub async fn get_all_user_db(pool: &PgPool) -> Result<Vec<User>, BlogAppError> {
    let user_row =
        sqlx::query!("Select user_id,first_name,last_name,email,isadmin,time_stamp from users")
            .fetch_all(pool)
            .await?;
    let users: Vec<User> = user_row
        .iter()
        .map(|u| User {
            user_id: u.user_id,
            first_name: u.first_name.clone(),
            last_name: u.last_name.clone(),
            email: u.email.clone(),
            timestamp: u.time_stamp.unwrap().to_string(),
            is_admin: u.isadmin.unwrap(),
        })
        .collect();
    match users.len() {
        0 => Err(BlogAppError::NotFound("No users found".into())),
        _ => Ok(users),
    }
}

pub async fn set_admin_db(
    pool: &PgPool,
    user_id: Uuid,
    admin_status: bool,
) -> Result<(), BlogAppError> {
    sqlx::query!(
        "Update users set isadmin = $1 where user_id = $2",
        admin_status,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_super_admin_db(
    pool: &PgPool,
    user_id: Uuid,
    admin_status: bool,
) -> Result<(), BlogAppError> {
    sqlx::query!(
        "Update users set is_super_admin = $1 where user_id = $2",
        admin_status,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
