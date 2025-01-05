// mod utils; // Import the utilities from `tests/utils.rs`
use actix_web::{test, web, App};
// use crate::src::app::api::field_sector::get_field_sector_translations_by_id; // Import the endpoint you want to test

#[actix_web::test]
async fn test_example() {
    assert_eq!(2 + 2, 4); // Simple test to ensure it's being executed
}

// #[actix_web::test]
// async fn test_get_field_sector_translations_by_id() {
//     let mut conn = utils::get_test_connection(); // Get test database connection
//     utils::insert_test_data(&mut conn); // Insert mock data

//     // Create an Actix web app
//     let app = test::init_service(
//         App::new()
//             .app_data(web::Data::new(conn)) // Provide the database connection
//             .route(
//                 "/field-sectors/translations/{id}",
//                 web::get().to(get_field_sector_translations_by_id),
//             ),
//     )
//     .await;

//     // Perform a test request
//     let req = test::TestRequest::get()
//         .uri("/field-sectors/translations/1")
//         .to_request();

//     let resp = test::call_service(&app, req).await;

//     // Check the response status
//     assert!(resp.status().is_success());

//     // Validate the response body
//     let result: FieldSectorWithTranslation = test::read_body_json(resp).await;
//     assert_eq!(result.id, 1);
//     assert_eq!(result.field_sector_translation.len(), 2);
// }
