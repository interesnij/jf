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
    FeeTypesData, PaymentTypeData, LanguageData, StageData,
}; 


pub fn create_routes(config: &mut web::ServiceConfig) {
    config.route("/create/contact", web::get().to(create_contact));
    config.route("/create/document", web::get().to(create_document));
    config.route("/create/expense_entry", web::get().to(create_expense_entry));
    config.route("/create/time_entry", web::get().to(create_time_entry));
    config.route("/create/flat_fee", web::get().to(create_flat_fee));
    config.route("/create/invoice", web::get().to(create_invoice));
    config.route("/create/matter", web::get().to(create_matter));
    config.route("/create/template", web::get().to(create_template));
    config.route("/create/post", web::get().to(create_post));
}



