use actix_web::{
    HttpRequest,
    HttpResponse, 
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    get_request_user,
    get_device_and_ajax,
    AuthResponseData,
};
use futures::StreamExt;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use sailfish::TemplateOnce;
use std::borrow::BorrowMut;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/")
        .route(web::get().to(login_page))
    ); 
    config.route("/auth/register", web::get().to(register_page));
    config.route("/auth/email-verified", web::get().to(email_verified_page));

    config.route("/auth/register_attorney_1", web::get().to(register_attorney_1_page));
    config.route("/auth/register_attorney_2", web::get().to(register_attorney_2_page));
    config.route("/auth/register_client_1", web::get().to(register_client_1_page));
    config.route("/auth/register_client_2", web::get().to(register_client_2_page));
    config.route("/auth/register_paralegal_1", web::get().to(register_paralegal_1_page));
    config.route("/auth/register_paralegal_2", web::get().to(register_paralegal_2_page));
    config.route("/auth/register_paralegal_3", web::get().to(register_paralegal_3_page));
    config.route("/auth/register_enterprise_1", web::get().to(register_enterprise_1_page));
    config.route("/auth/register_enterprise_2", web::get().to(register_enterprise_2_page));

    config.route("/auth/onboard_attorney_1", web::get().to(onboard_attorney_1_page));
    config.route("/auth/onboard_attorney_2", web::get().to(onboard_attorney_2_page));
    config.route("/auth/onboard_attorney_3", web::get().to(onboard_attorney_3_page));
    config.route("/auth/onboard_attorney_4", web::get().to(onboard_attorney_4_page));
}

pub async fn register_attorney_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/auth/register/attorney".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }
        else { 
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }

        if is_ajax == 0 {
            return crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_attorney.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_attorney.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn register_paralegal_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/auth/register/attorney".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }
        else { 
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }

        if is_ajax == 0 {
            return crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_paralegal.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_paralegal.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn register_enterprise_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/auth/register/attorney".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }
        else { 
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }

        if is_ajax == 0 {
            return crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_enterprise.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_enterprise.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn register_client_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/auth/register/attorney".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }
        else { 
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }

        if is_ajax == 0 {
            return crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_client.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register_client.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn register_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/auth/register".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }
        else { 
            title = "Register".to_string();
            description = "Juslaw: register".to_string();
        }

        if is_ajax == 0 {
            return crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/register.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn login_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).is_some() {
        //let request_user = get_request_user(&req).unwrap();
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/login".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Login".to_string();
            description = "https://app.juslaw.com: Login".to_string();
        }
        else { 
            title = "Login".to_string();
            description = "https://app.juslaw.com: Login".to_string();
        }

        if is_ajax == 0 {
            return crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/login.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/login.stpl")]
            struct Template {
                is_ajax:     i32,
                title:       String,
                description: String,
                link:        String,
                image:       String,
            }
            let body = Template {
                is_ajax:     is_ajax,
                title:       title,
                description: description,
                link:        link,
                image:       image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}