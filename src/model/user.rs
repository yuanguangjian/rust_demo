use serde::Serialize;
#[derive(Serialize)]
pub struct User {
   pub userName: String,
   pub age: i32,
   pub deleted: bool,
   pub userId: i32,
}
