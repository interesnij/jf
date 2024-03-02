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
    FeeTypesData, PaymentTypeData, LanguageData, StageData, RequestClient, RequestAttorney,
};

pub fn load_routes(config: &mut web::ServiceConfig) {
    config.route("/load/countries", web::get().to(countries_load));
    config.route("/load/states/{id}", web::get().to(states_load));
    config.route("/load/cities/{id}", web::get().to(cities_load));

    config.route("/load/specialities", web::get().to(specialities_load));
    config.route("/load/stages", web::get().to(stages_load));
    config.route("/load/appointment_types", web::get().to(appointment_types_load));
    config.route("/load/fee_types", web::get().to(fee_types_load));
    config.route("/load/payment_methods", web::get().to(payment_methods_load));
    config.route("/load/languages", web::get().to(languages_load)); 
    //config.route("/load/topics", web::get().to(topics_load));
    config.route("/load/clients", web::get().to(clients_load));
    config.route("/load/attorneys", web::get().to(attorneys_load));
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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
pub struct SpecialitiesParams {
    pub types: Option<String>,
} 

#[derive(Debug, Deserialize)]
pub struct SpecialitiessData { 
    pub results: Vec<SpecialitiesData>,
}

pub async fn specialities_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let specialities_list: Vec<SpecialitiesData>;
    let types: String;
    let params_some = web::Query::<SpecialitiesParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        types = crate::utils::get_string_with_string(params.types.clone());
    }
    else {
        types =  String::new();
    }

    let url = concat_string!(
        API.to_owned(),
        "users/specialities/"
        //"?state=", _id.to_string(),
    ); 
    let resp = request_get::<SpecialitiessData> (
        url,
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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
        types:             String,
    }
    let body = Template {
        specialities_list: specialities_list,
        types:             types,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

//////////////////////

#[derive(Debug, Deserialize)]
pub struct StagesParams {
    pub attorney: Option<String>,
    pub types:    Option<String>,
} 

#[derive(Debug, Deserialize)]
pub struct StagessData { 
    pub results: Vec<StageData>,
} 

pub async fn stages_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let stages_list: Vec<StageData>;
    let attorney: String;
    let types: String;
    let params_some = web::Query::<StagesParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        attorney = crate::utils::get_string_with_string(params.attorney.clone());
        types = crate::utils::get_string_with_string(params.types.clone());
    }
    else {
        attorney = String::new();
        types = String::new();
    }

    let url = concat_string!(
        API.to_owned(),
        "business/stages/",
        "?attorney=", attorney
    ); 
    let resp = request_get::<StagessData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        stages_list = data.results;
    }
    else {
        stages_list = Vec::new();
    }
    if types == "dropdown".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/stages_form.stpl")]
        pub struct Template {
            stages_list: Vec<StageData>,
        } 
        let body = Template {
            stages_list: stages_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else if types == "settings".to_string() {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/stages.stpl")]
        pub struct Template {
            stages_list: Vec<StageData>,
        } 
        let body = Template {
            stages_list: stages_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("types needed"))
    }
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        payment_methods_list = data.results;
    }
    else {
        payment_methods_list = Vec::new();
    }

    #[derive(TemplateOnce)] 
    #[template(path = "desctop/generic/items/payment_methods_form.stpl")]
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
        &"f844491b0ae26140bd3078902a3c1ff4c6b97bf3".to_string()
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

//////////////////////

#[derive(Debug, Deserialize)]
pub struct ClientsParams {
    pub search: Option<String>,
    pub types:  Option<String>,
} 
#[derive(Debug, Deserialize)]
pub struct ClientsData { 
    pub results: Vec<RequestClient>,
}

pub async fn clients_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let users_list: Vec<RequestClient>;

    let search: String;
    let types: String;
    let params_some = web::Query::<ClientsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            search = get_string_with_string(params.search.clone());
            types = get_string_with_string(params.types.clone());
        }
        else {
            search = String::new();
            types = String::new();
        }

    let url = concat_string!(
        API.to_owned(),
        "users/clients/",
        "?search=", search
    );
    let resp = request_get::<ClientsData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        users_list = data.results;
    }
    else {
        users_list = Vec::new();
    }

    if types == "single_form".to_string() { 
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/users_single_form.stpl")]
        pub struct Template {
            users_list: Vec<RequestClient>,
        }
        let body = Template {
            users_list: users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else if types == "many_form".to_string() { 
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/users_many_form.stpl")]
        pub struct Template {
            users_list: Vec<RequestClient>,
        }
        let body = Template {
            users_list: users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/users_single_form.stpl")]
        pub struct Template {
            users_list:   Vec<RequestAttorney>,
        }
        let body = Template {
            users_list:   users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

//////////////////////

#[derive(Debug, Deserialize)]
pub struct AttorneysParams {
    pub search: Option<String>,
    pub types:  Option<String>,
} 
#[derive(Debug, Deserialize)]
pub struct AttorneysData { 
    pub results: Vec<RequestAttorney>,
}

pub async fn attorneys_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let l = 2;
    let users_list: Vec<RequestAttorney>;

    let search: String;
    let types: String;
    let params_some = web::Query::<AttorneysParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        search = get_string_with_string(params.search.clone());
        types = get_string_with_string(params.types.clone());
    }
    else {
        search = String::new();
        types = String::new();
    }

    let url = concat_string!(
        API.to_owned(),
        "users/attorneys/",
        "?search=", search
    );
    let resp = request_get::<AttorneysData> (
        url,
        &"".to_string()
    ).await;
    if resp.is_ok() {
        let data = resp.expect("E.");
        users_list = data.results;
    }
    else {
        users_list = Vec::new();
    }

    if types == "single_form".to_string() { 
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/users_single_form.stpl")]
        pub struct Template {
            users_list: Vec<RequestAttorney>,
        }
        let body = Template {
            users_list: users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else if types == "many_form".to_string() { 
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/items/users_many_form.stpl")]
        pub struct Template {
            users_list: Vec<RequestAttorney>,
        }
        let body = Template {
            users_list: users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/users_single_form.stpl")]
        pub struct Template {
            users_list:   Vec<RequestAttorney>,
        }
        let body = Template {
            users_list:   users_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}