use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use actix_web_validator::Path;
use diesel::prelude::*;
use validator::Validate;
use crate::app::error::AppError;
use crate::app::lib::helpers::get_connection;
use crate::app::lib::validators::user_id_exists;
use crate::app::models::*;
use crate::schema::{users, user_type};
use crate::app::models::field_sectors::{FieldSector, FieldSectorTranslation, Meeting, Room};
use crate::schema::{field_sector, field_sector_translation, room, meeting};



#[get("/users")]
pub async fn list() -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    let query_result = users::table
        .load::<user::User>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load users"))?;

    Ok(HttpResponse::Ok().json(query_result))
}

// #[post("/users")]
// pub async fn create(user: web::Json<user::CreateUser>) -> Result<impl Responder, AppError> {
//     let mut con = get_connection()?;

//     let query_result = diesel::insert_into(users::table)
//         .values((
//             users::username.eq(&user.username),
//             users::password.eq(&user.password),
//         ))
//         .execute(&mut *con)
//         .map_err(|e| AppError::new(500).cause(e).message("Failed to create user"))?;

//     Ok(HttpResponse::Ok().json(query_result))
// }
#[post("/users")]
pub async fn create(user: web::Json<user::CreateUserWithType>) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    // Insert the user into the users table
    let user_id = diesel::insert_into(users::table)
        .values((
            users::username.eq(&user.username),
            users::password.eq(&user.password),
        ))
        .returning(users::id) // Get the user ID of the inserted row
        .get_result::<i32>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to create user"))?;

    // Prepare user_type entries for batch insertion
    let user_type_entries: Vec<_> = user
        .user_type
        .iter()
        .map(|user_type| (
            user_type::user_id.eq(user_id),
            user_type::name.eq(&user_type.name),
        ))
        .collect();

    // Insert all user_type entries in a single query
    diesel::insert_into(user_type::table)
        .values(&user_type_entries)
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to create user types"))?;

    // Return a success response
    Ok(HttpResponse::Ok().json("User and user types created successfully"))
}

#[derive(Deserialize, Validate)]
pub struct DeleteRequest {
    #[validate(custom = "user_id_exists")]
    id: i32,
}

#[delete("/users/{id}")]
pub async fn delete(path: Path<DeleteRequest>) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    let query_result = diesel::delete(users::table.filter(users::id.eq(path.id)))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to delete user"))?;

    Ok(HttpResponse::Ok().json(query_result))
}
