use actix_web::{
    web,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use sailfish::TemplateOnce;
use serde::{
    Deserialize, 
};
use crate::utils::{
    get_string_with_string, get_limit, get_string_withi64,
    get_string_withbool, get_request_user, AuthResponseData, request_get, API,
    get_id_withi32, get_string_withdate, gett_string_withi32,
    SmallCountryData, StateData, SmallCityData, SpecialitiesData, AppointmentTypeData,
    FeeTypesData, PaymentTypeData, LanguageData,
};

pub fn load_routes(config: &mut web::ServiceConfig) {
    config.route("/load/countries/", web::get().to(countries_load));
    config.route("/load/states/{id}", web::get().to(states_load));
    config.route("/load/cities/{id}", web::get().to(cities_load));

    config.route("/load/specialities/", web::get().to(specialities_load));
    config.route("/load/appointment_types", web::get().to(appointment_types_load));
    config.route("/load/fee_types", web::get().to(fee_types_load));
    config.route("/load/payment_methods", web::get().to(payment_methods_load));
    config.route("/load/languages", web::get().to(languages_load));
}


#[derive(Debug, Deserialize)]
pub struct CountriesData { 
    pub results: Vec<SmallCountryData>,
} 

pub async fn countries_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let countries_list: Vec<SmallCountryData>;

    let url = concat_string!(
        API.to_owned(),
        "locations/countries/"
    );
    let resp = request_get::<CountriesData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        countries_list = data.results;
    }
    else {
        countries_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/countries_form.stpl")]
    pub struct Template {
        countries_list: Vec<SmallCountryData>,
    }
    let body = Template {
        countries_list: countries_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

/////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct StatesData { 
    pub results: Vec<StateData>,
}

pub async fn states_load(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let states_list: Vec<StateData>;

    let url = concat_string!(
        API.to_owned(),
        "locations/states/",
        "?country=", _id.to_string()
    );
    let resp = request_get::<StatesData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        states_list = data.results;
    }
    else {
        states_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/states_form.stpl")]
    pub struct Template {
        states_list: Vec<StateData>,
    }
    let body = Template {
        states_list: states_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}


//////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct CitiesData { 
    pub results: Vec<SmallCityData>,
}

pub async fn cities_load(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let cities_list: Vec<SmallCityData>;

    let url = concat_string!(
        API.to_owned(),
        "locations/cities/",
        "?state=", _id.to_string()
    ); 
    let resp = request_get::<CitiesData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        cities_list = data.results;
    }
    else {
        cities_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/cities_form.stpl")]
    pub struct Template {
        cities_list: Vec<SmallCityData>,
    }
    let body = Template {
        cities_list: cities_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

//////////////////////

#[derive(Debug, Deserialize)]
pub struct SpecialitiessData { 
    pub results: Vec<SpecialitiesData>,
}

pub async fn specialities_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let specialities_list: Vec<SpecialitiesData>;

    let url = concat_string!(
        API.to_owned(),
        "users/specialities/"
        //"?state=", _id.to_string(),
    ); 
    let resp = request_get::<SpecialitiessData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        specialities_list = data.results;
    }
    else {
        specialities_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/specialities_form.stpl")]
    pub struct Template {
        specialities_list: Vec<SpecialitiesData>,
    }
    let body = Template {
        specialities_list: specialities_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

//////////////////////

#[derive(Debug, Deserialize)]
pub struct AppointmentTypesData { 
    pub results: Vec<AppointmentTypeData>,
}

pub async fn appointment_types_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let appointment_types_list: Vec<AppointmentTypeData>;

    let url = concat_string!(
        API.to_owned(),
        "users/appointment-types/"
        //"?state=", _id.to_string(),
    ); 
    let resp = request_get::<AppointmentTypesData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        appointment_types_list = data.results;
    }
    else {
        appointment_types_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/appointment_type_form.stpl")]
    pub struct Template {
        appointment_types_list: Vec<AppointmentTypeData>,
    }
    let body = Template {
        appointment_types_list: appointment_types_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

//////////////////////

#[derive(Debug, Deserialize)]
pub struct FeeTypessData { 
    pub results: Vec<FeeTypesData>,
}

pub async fn fee_types_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let object_list: Vec<FeeTypesData>;

    let url = concat_string!(
        API.to_owned(),
        "users/appointment-types/"
        //"?state=", _id.to_string(),
    ); 
    let resp = request_get::<FeeTypessData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        object_list = data.results;
    }
    else {
        object_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/fee_types_form.stpl")]
    pub struct Template {
        object_list: Vec<FeeTypesData>,
    }
    let body = Template {
        object_list: object_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}


//////////////////////

#[derive(Debug, Deserialize)]
pub struct PaymentTypesData { 
    pub results: Vec<PaymentTypeData>,
}

pub async fn payment_methods_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let payment_methods_list: Vec<PaymentTypeData>;

    let url = concat_string!(
        API.to_owned(),
        "users/payment-types/"
        //"?state=", _id.to_string(),
    ); 
    let resp = request_get::<PaymentTypesData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        payment_methods_list = data.results;
    }
    else {
        payment_methods_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/fee_types_form.stpl")]
    pub struct Template {
        payment_methods_list: Vec<PaymentTypeData>,
    }
    let body = Template {
        payment_methods_list: payment_methods_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

//////////////////////

#[derive(Debug, Deserialize)]
pub struct LanguagesData { 
    pub results: Vec<LanguageData>,
}

pub async fn languages_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let languages_list: Vec<LanguageData>;

    let url = concat_string!(
        API.to_owned(),
        "users/languages/"
        //"?state=", _id.to_string(),
    ); 
    let resp = request_get::<LanguagesData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        languages_list = data.results;
    }
    else {
        languages_list = Vec::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/generic/items/languages_form.stpl")]
    pub struct Template {
        languages_list: Vec<LanguageData>,
    }
    let body = Template {
        languages_list: languages_list,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

