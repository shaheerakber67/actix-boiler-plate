use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use diesel::prelude::*;
use crate::app::error::AppError;
use crate::app::lib::helpers::get_connection;
use crate::app::models::field_sectors::{FieldSector, FieldSectorTranslation, Meeting, Room, RoomWithMeeting, PaginationParams, CreateFieldSectorrWithTranslation, FieldSectorWithTranslation};
use crate::schema::{field_sector, field_sector_translation, room, meeting};
use crate::app::models::*;
use chrono::NaiveDateTime; // For timestamp fields
use std::option::Option;

#[get("/field-sector/all")]
pub async fn list_all() -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    let query_result = field_sector::table
        .load::<FieldSector>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load users"))?;

    Ok(HttpResponse::Ok().json(query_result))
}
#[get("/room/meeting-all")]
pub async fn list_all_meetings(
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?; // Get the database connection

    // Extract pagination parameters
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    println!("limit ===>> {}", limit);
    // Step 1: Fetch paginated rooms
    let rooms = room::table
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Room>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load rooms"))?;

    // Collect room IDs for fetching related meetings
    let room_ids: Vec<i32> = rooms.iter().map(|room| room.id).collect();

    // Step 2: Fetch meetings for the paginated rooms
    let meetings = meeting::table
        .filter(meeting::room_id.eq_any(&room_ids))
        .load::<Meeting>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load meetings"))?;

    // Step 3: Map rooms with their corresponding meetings
    let rooms_with_meetings: Vec<RoomWithMeeting> = rooms
        .into_iter()
        .map(|room| {
            let room_meetings: Vec<Meeting> = meetings
                .iter()
                .filter(|meeting| meeting.room_id == room.id)
                .cloned()
                .collect();

            RoomWithMeeting {
                id: room.id,
                room_id: room.id,
                lang_code: String::new(), // Adjust as needed
                name: room.name.clone(),
                meeting: room_meetings,
            }
        })
        .collect();

    // Step 4: Return the JSON response
    Ok(HttpResponse::Ok().json(rooms_with_meetings))
}


#[get("/field-sectors/translations")]
pub async fn list_all_translations(
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?; // Get the database connection

    // Extract pagination parameters
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    // Step 1: Fetch paginated rooms
    let field_sectors = field_sector::table
        .order(field_sector::id.desc())
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<FieldSector>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load field_sectors"))?;

    // Collect room IDs for fetching related meetings
    let field_sector_ids: Vec<i32> = field_sectors.iter().map(|field_sector| field_sector.id).collect();

    // Step 2: Fetch meetings for the paginated rooms
    let translations = field_sector_translation::table
        .filter(field_sector_translation::field_sector_id.eq_any(&field_sector_ids))
        .order(field_sector_translation::id.desc())
        .load::<FieldSectorTranslation>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load translations"))?;

    // Step 3: Map field sector with their corresponding translations
    let field_sector_with_translations: Vec<FieldSectorWithTranslation> = field_sectors
        .into_iter()
        .map(|field_sector| {
            let field_sector_translations: Vec<FieldSectorTranslation> = translations
                .iter()
                .filter(|translations| translations.field_sector_id == field_sector.id)
                .cloned()
                .collect();

            FieldSectorWithTranslation {

                id: field_sector.id,
                created_by: field_sector.created_by,
                last_modified_by: field_sector.last_modified_by,
                created_date: field_sector.created_date,
                last_modified_date: field_sector.last_modified_date,
                status: field_sector.status,
                field_sector_translation: field_sector_translations
            }
        })
        .collect();

    // Step 4: Return the JSON response
    Ok(HttpResponse::Ok().json(field_sector_with_translations))
}

#[get("/field-sectors/translations/{id}")]
pub async fn get_field_sector_translations_by_id(
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?; // Get the database connection
    let field_sector_id = path.into_inner(); // Extract the room ID from the path

    // Step 1: Fetch paginated rooms
    let field_sector = field_sector::table
        .filter(field_sector::id.eq(field_sector_id))
        .first::<FieldSector>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load field_sector"))?;

    // Step 2: Fetch meetings for the paginated rooms
    let translations = field_sector_translation::table
        .filter(field_sector_translation::field_sector_id.eq(&field_sector.id))
        .load::<FieldSectorTranslation>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to load translations"))?;

    // Step 3: Map field sector with their corresponding translations
    let field_sector_with_translations =

            FieldSectorWithTranslation {

                id: field_sector.id,
                created_by: field_sector.created_by,
                last_modified_by: field_sector.last_modified_by,
                created_date: field_sector.created_date,
                last_modified_date: field_sector.last_modified_date,
                status: field_sector.status,
                field_sector_translation: translations
            };

    // Step 4: Return the JSON response
    Ok(HttpResponse::Ok().json(field_sector_with_translations))
}


#[post("/field-sector")]
pub async fn create(field_sector: web::Json<CreateFieldSectorrWithTranslation>) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    // Insert the user into the users table
    let field_sector_id = diesel::insert_into(field_sector::table)
        .values((
            field_sector::status.eq(&field_sector.status),
        ))
        .returning(field_sector::id) // Get the user ID of the inserted row
        .get_result::<i32>(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to create field sector"))?;

    // Prepare user_type entries for batch insertion
    let translation_entries: Vec<_> = field_sector
        .field_sector_translation
        .iter()
        .map(|field_sector_translation| (
            field_sector_translation::field_sector_id.eq(field_sector_id),
            field_sector_translation::lang_code.eq(&field_sector_translation.lang_code),
            field_sector_translation::name.eq(&field_sector_translation.name),

        ))
        .collect();

    // Insert all user_type entries in a single query
    diesel::insert_into(field_sector_translation::table)
        .values(&translation_entries)
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to create translations"))?;

    // Return a success response
    Ok(HttpResponse::Ok().json("field sector and translations created successfully"))
}

#[put("/field-sector/{id}")]
pub async fn update(
    id: web::Path<i32>, 
    field_sector: web::Json<CreateFieldSectorrWithTranslation>
) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;

    // Step 1: Extract the `id` from the Path.
    let field_sector_id = id.into_inner(); // This moves the value, but now it's stored in `field_sector_id`.

    // Step 2: Update the field sector
    let updated_field_sector = diesel::update(field_sector::table)
        .filter(field_sector::id.eq(field_sector_id)) // Use the borrowed `field_sector_id`
        .set(field_sector::status.eq(&field_sector.status)) // Update the status field
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to update field sector"))?;

    if updated_field_sector == 0 {
        return Err(AppError::new(404).message("Field sector not found"));
    }

    // Step 3: Delete existing translations (optional if you want to fully replace them)
    diesel::delete(field_sector_translation::table)
        .filter(field_sector_translation::field_sector_id.eq(field_sector_id))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to delete old translations"))?;

    // Step 4: Prepare the new translations for insertion
    let translation_entries: Vec<_> = field_sector
        .field_sector_translation
        .iter()
        .map(|field_sector_translation| (
            field_sector_translation::field_sector_id.eq(field_sector_id),
            field_sector_translation::lang_code.eq(&field_sector_translation.lang_code),
            field_sector_translation::name.eq(&field_sector_translation.name),
        ))
        .collect();

    // Step 5: Insert the new translations
    diesel::insert_into(field_sector_translation::table)
        .values(&translation_entries)
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to update translations"))?;

    // Return a success response
    Ok(HttpResponse::Ok().json("Field sector and translations updated successfully"))
}

#[delete("/field-sector/{id}")]
pub async fn delete(id: web::Path<i32>) -> Result<impl Responder, AppError> {
    let mut con = get_connection()?;
    let field_sector_id = id.into_inner();
    // Step 1: Delete associated translations for the given field sector
    let deleted_translations = diesel::delete(field_sector_translation::table)
        .filter(field_sector_translation::field_sector_id.eq(field_sector_id))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to delete translations"))?;

    // Check if any translations were deleted
    if deleted_translations == 0 {
        return Err(AppError::new(404).message("No translations found for this field sector"));
    }

    // Step 2: Delete the field sector itself
    let deleted_field_sector = diesel::delete(field_sector::table)
        .filter(field_sector::id.eq(field_sector_id))
        .execute(&mut *con)
        .map_err(|e| AppError::new(500).cause(e).message("Failed to delete field sector"))?;

    // Check if the field sector was deleted
    if deleted_field_sector == 0 {
        return Err(AppError::new(404).message("Field sector not found"));
    }

    // Return a success response
    Ok(HttpResponse::Ok().json("Field sector and its translations deleted successfully"))
}
