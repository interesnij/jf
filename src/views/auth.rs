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
    get_request_user, get_device_and_ajax,
    AuthResponseData, SmallCountryData, StateData, SmallCityData, 
};
use futures::StreamExt;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use sailfish::TemplateOnce;
use std::borrow::BorrowMut;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(login_page)); 
    config.route("/auth/register", web::get().to(register_page));
    config.route("/auth/email-verified", web::get().to(email_verified_page));
    config.route("/auth/forgot-password", web::get().to(forgot_password_page));

    config.route("/auth/register/attorney", web::get().to(register_attorney_1_page));
    config.route("/auth/register_attorney_2", web::get().to(register_attorney_2_page));
    config.route("/auth/register/client", web::get().to(register_client_1_page));
    config.route("/auth/register_client_2", web::get().to(register_client_2_page));
    config.route("/auth/register/paralegal", web::get().to(register_paralegal_1_page));
    config.route("/auth/register_paralegal_2", web::get().to(register_paralegal_2_page));
    config.route("/auth/register_paralegal_3", web::get().to(register_paralegal_3_page));
    config.route("/auth/register_other_3", web::get().to(register_other_3_page));
    config.route("/auth/register/enterprise", web::get().to(register_enterprise_1_page));
    config.route("/auth/register_enterprise_2", web::get().to(register_enterprise_2_page));

    config.route("/auth/onboard_attorney_1", web::get().to(onboard_attorney_1_page));
    config.route("/auth/onboard_attorney_2", web::get().to(onboard_attorney_2_page));
    config.route("/auth/onboard_attorney_3", web::get().to(onboard_attorney_3_page));
    config.route("/auth/onboard_attorney_4", web::get().to(onboard_attorney_4_page));
}

pub async fn forgot_password_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/forgot-password".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Recover account".to_string();
            description = "Juslaw: Recover account".to_string();
        }
        else { 
            title = "Recover account".to_string();
            description = "Juslaw: Recover account".to_string();
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
            #[template(path = "desctop/auth/recover_account.stpl")]
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
            #[template(path = "desctop/auth/recover_account.stpl")]
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

pub async fn email_verified_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/email-verified".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Email verified".to_string();
            description = "Juslaw: Email verified".to_string();
        }
        else { 
            title = "Email verified".to_string();
            description = "Juslaw: Email verified".to_string();
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
            #[template(path = "desctop/auth/email_verified.stpl")]
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
            #[template(path = "desctop/auth/email_verified.stpl")]
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


pub async fn register_attorney_1_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_attorney_1".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
        }
        else { 
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
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
            #[template(path = "desctop/auth/1_register_attorney.stpl")]
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
            #[template(path = "desctop/auth/1_register_attorney.stpl")]
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

#[derive(Debug, Deserialize)]
pub struct CountriesData { 
    pub results: Vec<SmallCountryData>,
}
pub async fn register_attorney_2_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_attorney_2".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
        }
        else { 
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
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
            #[template(path = "desctop/auth/2_register_attorney.stpl")]
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
            #[template(path = "desctop/auth/2_register_attorney.stpl")]
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

pub async fn register_client_1_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_client_1".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
        }
        else { 
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
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
            #[template(path = "desctop/auth/1_register_client.stpl")]
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
            #[template(path = "desctop/auth/1_register_client.stpl")]
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

pub async fn register_client_2_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_client_2".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
        }
        else { 
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
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
            #[template(path = "desctop/auth/2_register_client.stpl")]
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
            #[template(path = "desctop/auth/2_register_client.stpl")]
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

pub async fn register_paralegal_1_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_paralegal_1".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
        }
        else { 
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
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
            #[template(path = "desctop/auth/1_register_paralegal.stpl")]
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
            #[template(path = "desctop/auth/1_register_paralegal.stpl")]
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

pub async fn register_paralegal_2_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_paralegal_2".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
        }
        else { 
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
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
            #[template(path = "desctop/auth/2_register_paralegal.stpl")]
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
            #[template(path = "desctop/auth/2_register_paralegal.stpl")]
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

pub async fn register_paralegal_3_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_paralegal_3".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 3".to_string();
            description = "Juslaw: Create Account - Step 3".to_string();
        }
        else { 
            title = "Create Account - Step 3".to_string();
            description = "Juslaw: Create Account - Step 3".to_string();
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
            #[template(path = "desctop/auth/3_register_paralegal.stpl")]
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
            #[template(path = "desctop/auth/3_register_paralegal.stpl")]
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

pub async fn register_other_3_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_other_3".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 3".to_string();
            description = "Juslaw: Create Account - Step 3".to_string();
        }
        else { 
            title = "Create Account - Step 3".to_string();
            description = "Juslaw: Create Account - Step 3".to_string();
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
            #[template(path = "desctop/auth/3_register_other.stpl")]
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
            #[template(path = "desctop/auth/3_register_other.stpl")]
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

pub async fn register_enterprise_1_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_enterprise_1".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
        }
        else { 
            title = "Create Account - Step 1".to_string();
            description = "Juslaw: Create Account - Step 1".to_string();
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
            #[template(path = "desctop/auth/1_register_enterprise.stpl")]
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
            #[template(path = "desctop/auth/1_register_enterprise.stpl")]
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

pub async fn register_enterprise_2_page(req: HttpRequest) -> impl Responder {
    if get_request_user(&req).is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/register_enterprise_2".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
        }
        else { 
            title = "Create Account - Step 2".to_string();
            description = "Juslaw: Create Account - Step 2".to_string();
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
            #[template(path = "desctop/auth/2_register_enterprise.stpl")]
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
            #[template(path = "desctop/auth/2_register_enterprise.stpl")]
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

pub async fn onboard_attorney_1_page(req: HttpRequest) -> impl Responder {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/onboard_attorney_1".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Onboard - Step 1".to_string();
            description = "Juslaw: Onboard - Step 1".to_string();
        }
        else { 
            title = "Onboard - Step 1".to_string();
            description = "Juslaw: Onboard - Step 1".to_string();
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
            #[template(path = "desctop/auth/1_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/1_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn onboard_attorney_2_page(req: HttpRequest) -> impl Responder {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/onboard_attorney_2".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Onboard - Step 2".to_string();
            description = "Juslaw: Onboard - Step 2".to_string();
        }
        else { 
            title = "Onboard - Step 2".to_string();
            description = "Juslaw: Onboard - Step 2".to_string();
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
            #[template(path = "desctop/auth/2_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/2_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn onboard_attorney_3_page(req: HttpRequest) -> impl Responder {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/onboard_attorney_3".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Onboard - Step 3".to_string();
            description = "Juslaw: Onboard - Step 3".to_string();
        }
        else { 
            title = "Onboard - Step 3".to_string();
            description = "Juslaw: Onboard - Step 3".to_string();
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
            #[template(path = "desctop/auth/3_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/3_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn onboard_attorney_4_page(req: HttpRequest) -> impl Responder {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/auth/onboard_attorney_4".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Onboard - Step 4".to_string();
            description = "Juslaw: Onboard - Step 4".to_string();
        }
        else { 
            title = "Onboard - Step 4".to_string();
            description = "Juslaw: Onboard - Step 4".to_string();
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
            #[template(path = "desctop/auth/4_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/4_onboard_attorney.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}