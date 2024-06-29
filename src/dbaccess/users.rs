use crate::{
    errors::BlogAppError,
    models::users::{ User, UserObj},
};
use sqlx::{self, PgPool};



pub async fn post_new_user_db(pool: &PgPool, new_user:UserObj) -> Result<User, BlogAppError> {
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
        is_admin:user_row.isadmin.expect("admin should be set"),
        timestamp:user_row.time_stamp.unwrap().to_string()
    })
}

pub async  fn get_all_user_db(pool: &PgPool)-> Result<Vec<User>, BlogAppError>{
    let user_row = sqlx::query!("Select user_id,first_name,last_name,email,time_stamp from users").fetch_all(pool).await?;
    user_row.iter().map(|u|{
    User{

    }
    })
}
