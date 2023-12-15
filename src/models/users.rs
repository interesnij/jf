use serde::{Serialize, Deserialize};
use crate::errors::Error;


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub password: String,
    pub description: Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}
impl User {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn is_admin(&self) -> bool {
        return self.perm > 10;
    }
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub description:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub emsil:    String,
    pub password: String,
}