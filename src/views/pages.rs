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
    get_request_user, UserSmallData, SpecialitiesData, AuthResponseData,
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    //config.route("/", web::get().to(index_page));
    //config.route("/documents/", web::get().to(documents_page));
    //config.route("/terms/", web::get().to(terms_page));
    //config.route("/policy/", web::get().to(policy_page));
}

#[derive(Debug, Deserialize)]
pub struct ActivitiesData {
    pub avatar:   Option<String>,
    pub created:  String,
    pub id:       i32,
    pub matter:   i32,
    pub modified: String,
    pub title:    String,
    pub r#type:   String,
}

///////////////////////////// Attorney board /////////
#[derive(Debug, Deserialize)]
pub struct OpenMattersData {
    pub client:           i32,
    pub client_avatar:    Option<String>,
    pub client_name:      String,
    pub created:          String,
    pub fee_type:         String,
    pub id:               i32,
    pub practice_area:    i32,
    pub principle_avatar: Option<String>,
    pub principle_name:   String,
    pub title:            String,
}

#[derive(Debug, Deserialize)]
pub struct BillingData {
    pub overdue:   i32,
    pub paid:      i32,
    pub un_billed: i32,
    pub unpaid:    i32,
}

#[derive(Debug, Deserialize)]
pub struct ClientData {
    pub id:          i32,
    pub avatar:      Option<String>,
    pub email:       String,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
}
#[derive(Debug, Deserialize)]
pub struct LastMessageData { 
    pub chat:       i32,
    pub created:    String,
    pub id:         i32,
    pub files:      Vec<String>,
    pub text:       String,
    pub timestamp1: Option<String>,
    pub r#type:     String,
}

#[derive(Debug, Deserialize)]
pub struct ChatsData {
    pub chat_type:    String,
    pub client_data:  ClientData,
    pub created:      String,
    pub id:           i32,
    pub is_archived:  bool,
    pub is_favorite:  bool,
    pub is_group:     bool,
    pub last_message: LastMessageData,
    pub title:        Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AttorneyOnboardingData {
    pub activities:         Vec<ActivitiesData>,
    pub billing:            BillingData,
    pub chats:              Vec<ChatsData>,
    pub open_matters:       Vec<OpenMattersData>,
    pub open_matters_count: i32,
}


///////////////////////////// Client board /////////

#[derive(Debug, Deserialize)]
pub struct RecentMattersData {
    pub attorney_avatar:       Option<String>,
    pub attorney_email:        String,
    pub attorney_id:           i32,
    pub attorney_name:         String,
    pub created:               String,
    pub description:           String,
    pub due_amount:            i32,
    pub id:                    i32,
    pub rate_type:             Vec<crate::utils::RateTypeData>,
    pub shared_with_data:      Vec<UserSmallData>,
    pub speciality_data:       Vec<SpecialitiesData>,
    pub stage:                 Option<String>,
    pub start_date:            String,
    pub status:                String,
    pub title:                 String,
    pub unread_document_count: i32,
    pub unread_message_count:  i32,
}

#[derive(Debug, Deserialize)] 
pub struct UpcomingBillsData {
    pub id:           i32,
    pub invoice_id:   String,
    pub title:        String,
    pub invoice_from: String,
    pub status:       String,
    pub due_date:     String,
    pub amount:       String,
} 

#[derive(Debug, Deserialize)] 
pub struct RecentDocumentsData {
    pub created:          String,
    pub file_size:        i32,
    pub shared_with_size: i32,
    pub title:            String,
    pub uploaded_by:      String,
}

#[derive(Debug, Deserialize)]
pub struct ClientOnboardingData { 
    pub recent_activities: Vec<ActivitiesData>,
    pub recent_documents:  Vec<RecentDocumentsData>,
    pub recent_matters:    Vec<RecentMattersData>,
    pub upcoming_bills:    Vec<UpcomingBillsData>,
}
////////////////////////////////////////////
