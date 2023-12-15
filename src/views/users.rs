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
