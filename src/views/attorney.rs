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
    get_request_user, UserSmallData, SpecialitiesData,
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    //config.route("/attorneys/", web::get().to(attorneys_page));
    //config.route("/attorney/engagement/", web::get().to(attorney_engagement_page));

    //config.route("/users/attorneys/{attorney_id}/leads_and_clients/", web::get().to(leads_and_clients_page));
    //config.route("/users/attorney/{attorney_id}/industry_contacts/", web::get().to(attorney_contacts_page));
    //config.route("/users/attorneys/{attorney_id}/industry_contact_detail/", web::get().to(attorney_contact_detail_page));
}

//////////////  ATTORNEYS  //////
//////////////  ATTORNEYS  //////

#[derive(Debug, Deserialize)]
pub struct AttorneysParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AttorneysData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::RequestAttorney>,
}
/////////////////////////////////////////////

//////////////  LEADS AND CLIENTS  //////
//////////////  LEADS AND CLIENTS  //////

#[derive(Debug, Deserialize)]
pub struct LeadsAndClientsParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LeadOrClientData { 
    pub id:            i32,
    pub first_name:    String,
    pub middle_name:   String,
    pub last_name:     String,
    pub phone:         String,
    pub avatar:        Option<String>,
    pub job:           String,
    pub company:       Option<String>,
    pub country_data:  crate::utils::CountryData,
    pub state_data:    Option<crate::utils::StateData>,
    pub city_data:     crate::utils::CityData,
    pub address:       String,
    pub zipcode:       String,
    pub note:          Option<String>,
    pub r#type:        String,
    pub email:         String,
    pub matters_count: i32,
    pub is_pending:    bool,
}

#[derive(Debug, Deserialize)]
pub struct LeadsAndClientsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<LeadOrClientData>,
}
///////////////////////////////////////////


//////////////  ATTORNEY CONTACTS  //////
//////////////  ATTORNEY CONTACTS  //////

#[derive(Debug, Deserialize)]
pub struct ContactsParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactData { 
    pub user_id: i32,
    pub name:    String,
    pub firm:    String,
    pub r#type:  String,
    pub phone:   String,
    pub pending: bool,
    pub email:   String,
    pub avatar:  Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<ContactData>,
}
/////////////////////////////


//////////////  ATTORNEY CONTACTS DETAIL  //////
//////////////  ATTORNEY CONTACTS DETAIL //////

#[derive(Debug, Deserialize)]
pub struct ContactsDetailParams {
    pub user_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PersonalDetailsData {
    pub user_id:        i32,
    pub name:           String,
    pub firm:           String,
    pub r#type:         String,
    pub phone:          String,
    pub pending:        bool,
    pub email:          String,
    pub avatar:         Option<String>,
    pub practice_areas: Vec<crate::utils::PracticeAreaData>,
    pub languages:      Vec<crate::utils::LanguageData>,
}

#[derive(Debug, Deserialize)]
pub struct PersonalAboutData {
    pub user_id:       i32,
    pub name:          String, 
    pub firm:          String,
    pub type:          String,
    pub phone:         String,
    pub pending:       bool,
    pub email:         String,
    pub avatar:        Option<String>,
    pub jurisdictions: Vec<crate::utils::PracticeJurisdictionsData>,
    pub education:     Vec<crate::utils::EducationData>,
    pub biography:     String,
}

#[derive(Debug, Deserialize)]
pub struct ContactData { 
    pub personal_details: PersonalDetailsData,
    pub about:            PersonalAboutData,
    pub payment_methods:  crate::utils::PaymentMethodData,
    pub fee_types:        crate::utils::FeeTypesData,
    pub firm_locations:   crate::utils::Vec<FirmLocationData>,
    pub address:          Vec<i32>,
}