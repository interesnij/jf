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
    //config.route("/social/chats/", web::get().to(chats_page));
    //config.route("/social/chats/{id}/", web::get().to(chat_page));
    //config.route("/social/chats/{id}/messages/", web::get().to(messages_page));
}

//////////////  CHATS  //////
//////////////  CHATS  //////

#[derive(Debug, Deserialize)]
pub struct ChatsParams {
    pub ordering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddressData { 
    pub country:  String,
    pub state:    String,
    pub city:     String,
    pub address1: String,
    pub address2: String,
    pub zip_code: String,
} 

#[derive(Debug, Deserialize)]
pub struct UserChatCardData { 
    pub id:             i32,
    pub first_name:     String,
    pub middle_name:    String,
    pub last_name:      String,
    pub email:          String,
    pub phone:          String,
    pub avatar:         Option<String>,
    pub address:        AddressData,
    pub user_type:      String,
    pub opportunity_id: Option<i32>, 
    pub lead_id:        Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatData { 
    pub id:                i32,
    pub title:             String,
    pub participants:      Vec<i32>,
    pub participants_data: Vec<UserChatCardData>,
    pub last_message:      crate::views::LastMessageData,
    pub is_favorite:       bool,
    pub is_archived:       bool,
    pub is_group:          bool,
    pub chat_type:         String,
    pub created:           String,
} 

#[derive(Debug, Deserialize)]
pub struct ChatsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<ChatData>,
}

//////////////  CHAT DETAIL  //////
//////////////  CHAT DETAIL  //////

/// это ChatData

//////////////  CHAT MESSAGES  //////
//////////////  CHAT MESSAGES  //////

#[derive(Debug, Deserialize)]
pub struct FilesData { 
    pub message: i32,
    pub title:   String,
    pub size:    String,
    pub file:    String,
} 

#[derive(Debug, Deserialize)]
pub struct MessageData { 
    pub id:          i32,
    pub author:      i32,
    pub author_data: crate::utils::UserCardData,
    pub r#type:      String,
    pub chat:        i32,
    pub text:        String,
    pub files:       Vec<FilesData>,
    pub timestamp1:  Option<String>,
    pub created:     String,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessagesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<MessageData>,
}