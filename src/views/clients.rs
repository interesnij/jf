use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::errors::Error;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use crate::utils::{
    get_request_user, UserSmallData, SpecialitiesData, AuthResponseData,
};


pub fn clients_routes(config: &mut web::ServiceConfig) {
    config.route("/client/overview", web::get().to(client_overview_page));
    config.route("/client/overview/matters", web::get().to(client_overview_matters_page));
    config.route("/client/overview/invoices", web::get().to(client_overview_invoices_page));
    config.route("/client/chats", web::get().to(client_chats_page));
    config.route("/client/chats/{id}", web::get().to(client_chat_page));
    config.route("/client/find", web::get().to(client_find_page));
    config.route("/client/find/search", web::get().to(client_search_page));
    config.route("/client/find/attorneys/{id}", web::get().to(client_find_attorney_page));
    config.route("/client/find/favorites", web::get().to(client_favorites_page));
    config.route("/forums", web::get().to(client_forums_page));
    config.route("/forums/my-posts", web::get().to(client_my_posts_page));
    config.route("/forums/following", web::get().to(client_following_posts_page));
    config.route("/news", web::get().to(client_news_page));
    config.route("/news/{id}", web::get().to(client_new_page));

    config.route("/client/settings/account", web::get().to(settings_page));
    config.route("/client/settings/notify", web::get().to(settings_notify_page));
    config.route("/client/settings/quard", web::get().to(settings_quard_page));
    config.route("/client/settings/pay", web::get().to(settings_pay_page));

    //config.route("/clients/", web::get().to(clients_page));
    //config.route("/users/clients/{id}/", web::get().to(client_page));
    //config.route("/users/clients/current/favorite/", web::get().to(client_favorite_page));
}


pub async fn client_overview_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/overview".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Overview".to_string();
            description = "https://app.juslaw.com: Overview".to_string();
        }
        else { 
            title = "Overview".to_string();
            description = "https://app.juslaw.com: Overview".to_string();
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
            #[template(path = "desctop/client/overview.stpl")]
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
            #[template(path = "desctop/client/overview.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_overview_matters_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/overview/matters".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Matters".to_string();
            description = "https://app.juslaw.com: Matters".to_string();
        }
        else { 
            title = "Matters".to_string();
            description = "https://app.juslaw.com: Matters".to_string();
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
            #[template(path = "desctop/client/matters.stpl")]
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
            #[template(path = "desctop/client/matters.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_overview_invoices_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/overview/invoices".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Invoices".to_string();
            description = "https://app.juslaw.com: Invoices".to_string();
        }
        else { 
            title = "Invoices".to_string();
            description = "https://app.juslaw.com: Invoices".to_string();
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
            #[template(path = "desctop/client/invoices.stpl")]
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
            #[template(path = "desctop/client/invoices.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_chats_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/chats".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Chats".to_string();
            description = "https://app.juslaw.com: Chats".to_string();
        }
        else { 
            title = "Chats".to_string();
            description = "https://app.juslaw.com: Chats".to_string();
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
            #[template(path = "desctop/client/chats.stpl")]
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
            #[template(path = "desctop/client/chats.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_chat_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let chat_id = *_id;
        let title: String; 
        let description: String;
        let link = "/client/chats/".to_string() + &_id.to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Chat".to_string();
            description = "https://app.juslaw.com: Chat".to_string();
        }
        else { 
            title = "Chat".to_string();
            description = "https://app.juslaw.com: Chat".to_string();
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
            #[template(path = "desctop/client/chat.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                chat_id:      i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                chat_id:      chat_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/client/chat.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                chat_id:      i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                chat_id:      chat_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_find_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/find".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Find an Attorney".to_string();
            description = "https://app.juslaw.com: Find an Attorney".to_string();
        }
        else { 
            title = "Find an Attorney".to_string();
            description = "https://app.juslaw.com: Find an Attorney".to_string();
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
            #[template(path = "desctop/client/find.stpl")]
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
            #[template(path = "desctop/client/find.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_forums_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/forums".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Forums".to_string();
            description = "https://app.juslaw.com: Forums".to_string();
        }
        else { 
            title = "Forums".to_string();
            description = "https://app.juslaw.com: Forums".to_string();
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
            #[template(path = "desctop/client/forums.stpl")]
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
            #[template(path = "desctop/client/forums.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_my_posts_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/forums/my-posts".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "My posts".to_string();
            description = "https://app.juslaw.com: My posts".to_string();
        }
        else { 
            title = "My posts".to_string();
            description = "https://app.juslaw.com: My posts".to_string();
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
            #[template(path = "desctop/client/my_forums.stpl")]
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
            #[template(path = "desctop/client/my_forums.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_following_posts_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/forums/following".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Forums".to_string();
            description = "https://app.juslaw.com: Forums".to_string();
        }
        else { 
            title = "Forums".to_string();
            description = "https://app.juslaw.com: Forums".to_string();
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
            #[template(path = "desctop/client/following_forums.stpl")]
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
            #[template(path = "desctop/client/following_forums.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_news_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/news".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "News".to_string();
            description = "https://app.juslaw.com: News".to_string();
        }
        else { 
            title = "News".to_string();
            description = "https://app.juslaw.com: News".to_string();
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
            #[template(path = "desctop/client/news.stpl")]
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
            #[template(path = "desctop/client/news.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_new_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/news/".to_string() + &_id.to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "News".to_string();
            description = "https://app.juslaw.com: News".to_string();
        }
        else { 
            title = "News".to_string();
            description = "https://app.juslaw.com: News".to_string();
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
            #[template(path = "desctop/client/new.stpl")]
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
            #[template(path = "desctop/client/new.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}


pub async fn client_search_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/find/search".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Search".to_string();
            description = "https://app.juslaw.com: Search".to_string();
        }
        else { 
            title = "Search".to_string();
            description = "https://app.juslaw.com: Search".to_string();
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
            #[template(path = "desctop/client/search.stpl")]
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
            #[template(path = "desctop/client/search.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_find_attorney_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/find/attorneys/".to_string() + &_id.to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Attorney".to_string();
            description = "https://app.juslaw.com: Attorney".to_string();
        }
        else { 
            title = "Attorney".to_string();
            description = "https://app.juslaw.com: Attorney".to_string();
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
            #[template(path = "desctop/attorney/attorney.stpl")]
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
            #[template(path = "desctop/attorney/attorney.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn settings_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/client/settings".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
        }
        else { 
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
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
            #[template(path = "desctop/client/settings_general.stpl")]
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
            #[template(path = "desctop/client/settings_general.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn settings_notify_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/client/settings/notify".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
        }
        else { 
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
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
            #[template(path = "desctop/client/settings_notify.stpl")]
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
            #[template(path = "desctop/client/settings_notify.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn settings_quard_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/client/settings/quard".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
        }
        else { 
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
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
            #[template(path = "desctop/client/settings_quard.stpl")]
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
            #[template(path = "desctop/client/settings_quard.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn settings_pay_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/client/settings/pay".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
        }
        else { 
            title = "Settings".to_string();
            description = "https://app.juslaw.com: Settings".to_string();
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
            #[template(path = "desctop/client/settings_pay.stpl")]
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
            #[template(path = "desctop/client/settings_pay.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
}

pub async fn client_favorites_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/client/find/favorites".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Favorites".to_string();
            description = "https://app.juslaw.com: Favorites".to_string();
        }
        else { 
            title = "Favorites".to_string();
            description = "https://app.juslaw.com: Favorites".to_string();
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
            #[template(path = "desctop/client/favorites.stpl")]
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
            #[template(path = "desctop/client/favorites.stpl")]
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
    else {
        crate::views::login_page(req).await
    }
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
    pub city_data:         Option<crate::utils::CityData>, 
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
pub struct ClientssData { 
    pub favorite_attorneys:      Vec<i32>,
    pub favorite_attorneys_data: Vec<crate::utils::RequestAttorney>,
}

////////////////////////////////
