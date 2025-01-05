#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]

pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct UserType {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUserWithType {
    pub username: String,
    pub password: String,
    pub user_type: Vec<CreateUserType>, // Change to Vec to handle multiple user types
}

#[derive(Deserialize)]
pub struct CreateUserType {
    pub name: String,
}
