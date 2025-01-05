use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;  
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// use crate::app::models::{FieldSector, FieldSectorTranslation}; // Your models
// use crate::app::schema::{field_sector, field_sector_translation}; // Your schema

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// pub fn get_test_connection() -> SqliteConnection {
//     let conn = SqliteConnection::establish(":memory:")
//         .expect("Failed to create an in-memory SQLite connection");

//     // Run migrations to set up the schema
//     conn.run_pending_migrations(MIGRATIONS)
//         .expect("Failed to run migrations");

//     conn
// }

// pub fn insert_test_data(conn: &mut SqliteConnection) {
//     use field_sector::dsl::*;
//     use field_sector_translation::dsl::*;

//     // Insert mock `field_sector` data
//     diesel::insert_into(field_sector)
//         .values((
//             id.eq(1), // Mock ID
//             created_by.eq("test_user"),
//             last_modified_by.eq("test_user"),
//             created_date.eq(chrono::Utc::now().naive_utc()), // Use `chrono` for timestamps
//             last_modified_date.eq(chrono::Utc::now().naive_utc()),
//             status.eq("active"),
//         ))
//         .execute(conn)
//         .expect("Failed to insert test field sector");

//     // Insert mock `field_sector_translation` data
//     diesel::insert_into(field_sector_translation)
//         .values(vec![
//             (
//                 id.eq(1), // Mock ID
//                 field_sector_id.eq(1), // Link to `field_sector`
//                 language.eq("en"),
//                 name.eq("Test Field Sector"),
//             ),
//             (
//                 id.eq(2),
//                 field_sector_id.eq(1),
//                 language.eq("es"),
//                 name.eq("Sector de Campo de Prueba"),
//             ),
//         ])
//         .execute(conn)
//         .expect("Failed to insert test translations");
// }