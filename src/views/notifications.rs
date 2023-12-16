use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use crate::utils::{
    get_request_user, UserSmallData, SpecialitiesData, AuthResponseData
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    //config.route("/notifications/", web::get().to(notifications_page));
    //config.route("/notifications/settings/", web::get().to(notifications_settings_page));
}

//////////////  NOTIFICATIONS  //////
//////////////  NOTIFICATIONS  //////

#[derive(Debug, Deserialize)]
pub struct NotificationsParams {
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub ordering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NotifyData { 
    pub id:          i32,
    pub title:       String,
    pub object_id:   i32,  
    pub content:     String,
    pub r#type:      i32,
    pub runtime_tag: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationData { 
    pub id:           i32,
    pub notification: NotifyData,
    pub sender:       i32,  
    pub sender_data:  crate::utils::UserCardData,
    pub status:       String,
    pub created:      String,
    pub modified:     String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationsData { 
    pub count:    i32,
    pub next:     Option<String>,
    pub previous: Option<String>,
    pub results:  Vec<NotificationData>,
}
///////////////////////////////////////

//////////////  NOTIFICATIONS SETTINGS  //////
//////////////  NOTIFICATIONS SETTINGS  //////

#[derive(Debug, Deserialize)]
pub struct NotificationsSettingsData { 
    pub chats:    bool,
    pub contacts: bool,
    pub email:    bool,  
    pub forums:   bool,
    pub matters:  bool,
    pub push:     bool,
}
///////////////////////////////////////