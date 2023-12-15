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
};
use futures::StreamExt;
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use sailfish::TemplateOnce;
use std::borrow::BorrowMut;
use actix_web::http::header::Header;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/login/")
        .route(web::get().to(login_page))
        .route(web::post().to(login))
    );
    config.service(web::resource("/signup/")
        .route(web::get().to(signup_page))
        .route(web::post().to(process_signup))
    );
    config.route("/logout/", web::get().to(logout_page));
}


pub async fn signup_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/signup/".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Signup".to_string();
            description = "https://app.juslaw.com: Signup".to_string();
        }
        else { 
            title = "Signup".to_string();
            description = "https://app.juslaw.com: Signup".to_string();
        }

        if is_ajax == 0 {
            crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/signup.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/signup.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn login_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;  
        let title: String;
        let description: String;
        let link = "/login/".to_string();
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
            crate::utils::get_first_load_page (
                &req,
                is_desctop,
                &title,
                &description,
                &link,
                &image,
            ).await
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/login.stpl")]
            struct Template {
                is_ajax: i32,
            }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/auth/login.stpl")]
            struct Template {
                is_ajax: i32,
                }
            let body = Template {
                is_ajax: is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn logout_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        crate::utils::remove_token(&req);
        crate::views::login_page(req).await
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub email:    String,
    pub password: String,
}
pub async fn login_form(payload: &mut Multipart) -> LoginUser2 {
    let mut form: LoginUser2 = LoginUser2 {
        email:    "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "email" {
                    form.email = data_string
                } else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}

pub async fn login(mut payload: Multipart, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("0"))
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        println!("{:?}", form.username.clone());
        println!("{:?}", form.password.clone());
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserForm {
    pub username: String,
    pub email:    String,
    pub password: String,
} 
pub async fn signup_form(payload: &mut Multipart) -> NewUserForm {
    let mut form: NewUserForm = NewUserForm {
        username: "".to_string(),
        email:    "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                }
                else if field.name() == "email" {
                    form.email = data_string
                }
                else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}
pub async fn process_signup(req: HttpRequest, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    // Если пользователь не аноним, то отправляем его на страницу новостей
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("-300"))
    } 
    else { 
        let form = signup_form(payload.borrow_mut()).await;
        println!("{:?}", form.username.clone());
        println!("{:?}", form.email.clone());
        println!("{:?}", form.password.clone());

        //set_current_user(&_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body())
    }
}
