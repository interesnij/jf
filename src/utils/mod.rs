mod auths;
mod crypto;
mod reqwest;

pub use self::{
    auths::*, 
    crypto::*,
    reqwest::*,
};

use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::errors::Error;
use serde::{Deserialize, Serialize};
use crate::errors::AuthError;
use sailfish::TemplateOnce;
use crate::models::{
    User, 
};


pub const API: &str = "https://backend.juslaw.com/api/v1/";

fn get_data<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("Request-Data")?.to_str().ok();
} 

pub fn get_request_user(req: &HttpRequest) -> Option<AuthResponseData> {
    let _tokenize = get_data(req);
    if _tokenize.is_some() { 
        let _user_res: Result<AuthResponseData, serde_json::Error> = serde_json::from_str(&_tokenize.unwrap());
        if _user_res.is_ok() {
            let _user: AuthResponseData = _user_res.expect("E.");
            return Some(_user);
        }
        return None;
    }
    return None;
}

#[derive(Debug, Deserialize)]
pub struct ElapsedTimeData {
    pub elapsed_time: i32,
    pub status:       String,
}

#[derive(Debug, Deserialize)]
pub struct RateTypeData {
    pub id:    i32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSmallData { 
    pub id:          i32,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub avatar:      Option<String>,
    pub user_type:   String,
}
#[derive(Debug, Deserialize)]
pub struct UserCardData {  
    pub id:          i32,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub avatar:      Option<String>,
}

////////////////////////////////////////////

//////////////  NOTIFY INFO   //////

#[derive(Debug, Deserialize)]
pub struct NotifyCardData { 
    pub content:     String,
    pub id:          i32,
    pub object_id:   i32,
    pub runtime_tag: String,
    pub title:       String,
    pub r#type:      i32,
}
#[derive(Debug, Deserialize)]
pub struct NotifyData { 
    pub created:      String,
    pub id:           i32,
    pub modified:     String,
    pub notification: NotifyCardData,
    pub sender:       i32,
    pub sender_data:  UserCardData,
    pub status:       String,
}
#[derive(Debug, Deserialize)]
pub struct NotifiesData {  
    pub count:        i32,
    pub next:         Option<String>,
    pub unread_count: i32,
    pub previous:     Option<String>,
    pub results:      Vec<NotifyData>,
}
////////////////////////////////////////////


pub async fn get_first_load_page (
    req:         &HttpRequest,
    is_desctop:  bool,
    title:       &String,
    description: &String,
    uri:         &String,
    image:       &String,
) -> actix_web::Result<HttpResponse> {
    let _request_user_some = get_request_user(&req);
    if _request_user_some.is_some() {
        println!("is auth!");
        let _request_user = _request_user_some.unwrap();
        if is_desctop {
            #[derive(TemplateOnce)] 
            #[template(path = "desctop/generic/first_load.stpl")]
            struct Template<'a> { 
                request_user: AuthResponseData,
                title:        &'a String,
                description:  &'a String,
                image:        &'a String,
                uri:          &'a String,
            }
            let body = Template {
                request_user: _request_user,
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/first_load.stpl")]
            struct Template<'a> {
                request_user: AuthResponseData,
                title:        &'a String,
                description:  &'a String,
                image:        &'a String,
                uri:          &'a String,
            }
            let body = Template {
                request_user: _request_user,
                title:        title,
                description:  description,
                image:        image,
                uri:          uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        println!("is anon!");
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/anon_first_load.stpl")]
            struct Template<'a> {
                title:       &'a String,
                description: &'a String,
                image:       &'a String,
                uri:         &'a String,
            }
            let body = Template {
                title:       title,
                description: description,
                image:       image,
                uri:         uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/anon_first_load.stpl")]
            struct Template<'a> {
                title:       &'a String,
                description: &'a String,
                image:       &'a String,
                uri:         &'a String,
            }
            let body = Template {
                title:       title,
                description: description,
                image:       image,
                uri:         uri,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}

pub fn get_default_image() -> String {
    return "/static/images/hakew.png".to_string();
}

pub fn is_desctop(req: &HttpRequest) -> bool {
    return !get_content_type(req).unwrap().contains("Mobile");
}

pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, i32) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = 0;
    let _type = true;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = params.ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
    }

    (is_desctop(req), is_ajax)
}