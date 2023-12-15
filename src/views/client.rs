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
    //config.route("/clients/", web::get().to(clients_page));
    //config.route("/users/clients/{id}/", web::get().to(client_page));
    //config.route("/users/clients/current/favorite/", web::get().to(client_favorite_page));
}


//////////////  CLIENTS  //////
//////////////  CLIENTS  //////

#[derive(Debug, Deserialize)]
pub struct ClientsParams {
    pub attorney: Option<i32>,
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub search:   Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClientsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<ClientData>,
}

////////////////////////////////

//////////////  CLIENT DETAIL  //////
//////////////  CLIENT DETAIL  //////

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
////////////////////////////////

//////////////  CLIENT FAVOURITES  //////
//////////////  CLIENT FAVOURITES  //////

#[derive(Debug, Deserialize)]
pub struct ClientsData { 
    pub favorite_attorneys:      Vec<i32>,
    pub favorite_attorneys_data: Vec<crate::utils::RequestAttorney>,
}

////////////////////////////////
