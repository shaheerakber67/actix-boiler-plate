
use serde::{ Deserialize, Serialize };
use postgres_types::{ FromSql, ToSql, Type, IsNull, to_sql_checked };
use std::error::Error;
use actix_web::web::BytesMut;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text, Timestamp};
use chrono::NaiveDateTime; // For timestamp fields
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldSectorStatus  {
    ACTIVE,
    INACTIVE
}

impl ToSql for FieldSectorStatus  {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        match *self {
            FieldSectorStatus ::ACTIVE => out.extend_from_slice(b"ACTIVE"),
            FieldSectorStatus ::INACTIVE => out.extend_from_slice(b"INACTIVE"),
        }
        Ok(IsNull::No)

    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "status"
    }
    to_sql_checked!();
}

impl FromSql<'_> for FieldSectorStatus  {
    fn from_sql(_sql_type: &Type, value: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match value {
            b"ACTIVE" => Ok(FieldSectorStatus ::ACTIVE),
            b"INACTIVE" => Ok(FieldSectorStatus ::INACTIVE),
            _ => Ok(FieldSectorStatus ::ACTIVE),
        }
    }
    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "status"
    }
}

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CreateFieldSector {
    pub status: FieldSectorStatus,
}

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct CreateFieldSectorTranslation {
    pub lang_code: String,
    pub name: String,
}

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CreateFieldSectorrWithTranslation {
    pub status: String,
    pub field_sector_translation: Vec<CreateFieldSectorTranslation>, // Change to Vec to handle multiple user types
}


#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: i32,
    pub name: String,
}

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Meeting {
    pub id: i32,
    pub room_id: i32,
    pub lang_code: String,
    pub name: String,
}


#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct RoomWithMeeting {
    pub id: i32,
    pub room_id: i32,
    pub lang_code: String,
    pub name: String,
    pub meeting: Vec<Meeting>
}

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FieldSector {
    pub id: i32,
    pub created_by: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub created_date: Option<chrono::NaiveDateTime>,
    pub last_modified_date: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
}


#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FieldSectorTranslation {
    pub id: i32,
    pub field_sector_id: i32,
    pub created_by: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub created_date: Option<chrono::NaiveDateTime>,
    pub last_modified_date: Option<chrono::NaiveDateTime>,
    pub lang_code: String,
    pub name: String,
}


#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FieldSectorWithTranslation {
    pub id: i32,
    pub created_by: Option<i32>,
    pub last_modified_by: Option<i32>,
    pub created_date: Option<chrono::NaiveDateTime>,
    pub last_modified_date: Option<chrono::NaiveDateTime>,
    pub status: Option<String>,
    pub field_sector_translation: Vec<FieldSectorTranslation>
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}