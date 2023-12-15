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


pub fn page_routes(config: &mut web::ServiceConfig) {
    //config.route("/locations/countries/", web::get().to(countries_load));
    //config.route("/locations/states/", web::get().to(states_load));
    //config.route("/locations/cities/", web::get().to(cities_load));
    //config.route("/users/specialities/", web::get().to(specialities_load));
    //config.route("/users/clients/search_attorneys_and_paralegals/", web::get().to(search_attorneys_and_paralegals));
    //config.route("/attorneys/stages/", web::get().to(stages_page));
}

//////////////  COUNTRIES  //////
//////////////  COUNTRIES //////

#[derive(Debug, Deserialize)]
pub struct CountryData {
    pub id:        i32,
    pub name:      String,
    pub code2:     String,
    pub phone:     String,
    pub has_state: bool,
}

#[derive(Debug, Deserialize)]
pub struct CountriesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<CountryData>,
}


//////////////  STATES  //////
//////////////  STATES //////

#[derive(Debug, Deserialize)]
pub struct StatesParams {
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
    pub country: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StatesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::StateData>,
}


//////////////  CITIES  //////
//////////////  CITIES //////

#[derive(Debug, Deserialize)]
pub struct CityParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub region: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CitiesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::CityData>,
}

//////////////  SPECIALITIES  //////
//////////////  SPECIALITIES //////

#[derive(Debug, Deserialize)]
pub struct SpecialityParams {
    pub ordering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpecialitiesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::SpecialitiesData>,
}

//////////////  SEARCH ATTORNEYS AND PARALEGALS  //////
//////////////  SEARCH ATTORNEYS AND PARALEGALS //////

#[derive(Debug, Deserialize)]
pub struct SearchAtttorneysParams {
    pub offset:   Option<i64>,
    pub sharable: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchAttorneysData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::UserSmallData>,
}
//////////////////////////////////////////////


//////////////  STAGES  //////
//////////////  STAGES //////

#[derive(Debug, Deserialize)]
pub struct StagesParams {
    pub attorney: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StagesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::StageData>,
}
//////////////////////////////////////////////