use serde::Serialize;
#[derive(Serialize)]
pub struct User {
   pub user_name: String,
   pub age: i32,
   pub deleted: bool,
   pub user_id: i32,
}
