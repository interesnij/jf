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
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};


pub fn user_routes(config: &mut web::ServiceConfig) {
    //config.route("/users/clients/{id}/", web::get().to(user_clients_page));
    //config.route("/promotion/events/", web::get().to(events_page));
}


#[derive(Debug, Deserialize)]
pub struct ClientData {  
    pub id:                i32,
    pub first_name:        String,
    pub middle_name:       String,
    pub last_name:         String,
    pub email:             String,
    pub phone:             String,
    pub r#type:            String,
    pub twofa:             bool,
    pub job:               String,
    pub avatar:            Option<String>,
    pub client_type:       String,
    pub organization_name: Option<String>,
    pub note:              Option<String>,
    pub country:           i32,
    pub country_data:      crate::utils::CountryData,
    pub state:             Option<i32>,
    pub state_data:        Option<crate::utils::StateData>,
    pub city:              i32,
    pub city_data:         crate::utils::CityData,
    pub timezone:          i32,
    pub timezone_data:     crate::utils::TimezoneData,
    pub help_description:  Option<String>,
    pub specialities:      Vec<i32>,
    pub zip_code:          String,
    pub address1:          String,
    pub address2:          String,
    pub matters_count:     i32,
}

//////////////  EVENTS   //////
//////////////  EVENTS //////

#[derive(Debug, Deserialize)]
pub struct EventsParams {
    pub ordering:  Option<String>,
    pub attorney:  Option<i32>,
    pub client:    Option<i32>,
    pub paralegal: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EventData {  
    pub id:            i32,
    pub attorney:      Option<i32>,
    pub paralegal:     Option<i32>, 
    pub title:         String,
    pub description:   Option<String>,
    pub is_all_day:    bool,
    pub start:         String,
    pub end:           String,
    pub duration:      String,
    pub location:      String,
    pub timezone:      i32,
    pub timezone_data: crate::utils::TimezoneData,
}

#[derive(Debug, Deserialize)]
pub struct EventsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<EventData>,
}
/////////////////////////////


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
    pub r#type:          String,
    pub phone:         String,
    pub pending:       bool,
    pub email:         String,
    pub avatar:        Option<String>,
    pub jurisdictions: Vec<crate::utils::PracticeJurisdictionsData>,
    pub education:     Vec<crate::utils::EducationData>,
    pub biography:     String,
}

#[derive(Debug, Deserialize)]
pub struct ContactssData { 
    pub personal_details: PersonalDetailsData,
    pub about:            PersonalAboutData,
    pub payment_methods:  crate::utils::PaymentMethodData,
    pub fee_types:        crate::utils::FeeTypesData,
    pub firm_locations:   Vec<crate::utils::FirmLocationData>,
    pub address:          Vec<i32>,
}