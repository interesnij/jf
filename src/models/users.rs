use serde::{Serialize, Deserialize};
use crate::errors::Error;


#[derive(Debug, Serialize, Deserialize)]
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
            return "<img src=".to_string() + &self.image.as_deref().unwrap().to_string() + &">".to_string();
        }
        else {
            return "<svg stroke='currentColor' fill='currentColor' stroke-width='0' viewBox='0 0 448 512' height='1em' width='1em'><path d='M224 256c70.7 0 128-57.3 128-128S294.7 0 224 0 96 57.3 96 128s57.3 128 128 128zm89.6 32h-16.7c-22.2 10.2-46.9 16-72.9 16s-50.6-5.8-72.9-16h-16.7C60.2 288 0 348.2 0 422.4V464c0 26.5 21.5 48 48 48h352c26.5 0 48-21.5 48-48v-41.6c0-74.2-60.2-134.4-134.4-134.4z'></path></svg>".to_string();
        }
    }
    pub fn is_admin(&self) -> bool {
        return self.perm > 10;
    }
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
}

#[derive(Debug, Deserialize)]
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