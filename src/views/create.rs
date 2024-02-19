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
    FeeTypesData, PaymentTypeData, LanguageData, StageData, UserSharedData
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
    config.route("/create/note", web::get().to(create_note));
    config.route("/create/chat", web::get().to(create_chat));
} 

//////////////////////

#[derive(Debug, Deserialize)]
pub struct AttorneysParams {
    pub types: Option<String>,
} 

pub async fn create_contact(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let types: String;
    let params_some = web::Query::<AttorneysParams>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        types = get_string_with_string(params.types.clone());
    } 
    else {
        types = String::new();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/contact.stpl")]
    pub struct Template {
        types: String,
    }
    let body = Template {
        types: types,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

//////////////////////

pub async fn create_document(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/document.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_time_entry(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/time_entry.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_flat_fee(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/flat_fee.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_expense_entry(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/expense_entry.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_invoice(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/invoice.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_matter(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        #[derive(TemplateOnce)]
        #[template(path = "desctop/create/matter.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
        }
        let body = Template {
            request_user: request_user,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn create_template(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/template.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_post(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/post.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_note(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    #[derive(TemplateOnce)]
    #[template(path = "desctop/create/note.stpl")]
    pub struct Template {}
    let body = Template {}
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

#[derive(Debug, Deserialize)]
pub struct AllContactsData { 
    pub results: Vec<UserSharedData>,
}

pub async fn create_chat(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {  
        let request_user = user_some.unwrap();
        let l = 2;
        let object_list: Vec<UserSharedData>;

        let url = concat_string!(
            API.to_owned(), 
            "users/attorneys/",
            request_user.user_id.clone(),
            "/get_all_contacts/"
        );
        
        let resp = request_get::<AllContactsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            object_list = data.results;
        }
        else {
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/create/chat.stpl")]
        pub struct Template {
            object_list: Vec<UserSharedData>,
        }
        let body = Template {
            object_list: object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}

