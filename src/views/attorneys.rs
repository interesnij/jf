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
use serde::{
    Deserialize, 
    //Serialize
};
use crate::utils::{
    get_request_user, UserSmallData, SpecialitiesData, 
    AuthResponseData, request_get, API, 
    get_plans_page, get_onboarding_page,
};


pub fn attorneys_routes(config: &mut web::ServiceConfig) {
    config.route("/attorney/overview", web::get().to(attorney_overview_page));
    config.route("/attorney/leads", web::get().to(attorney_leads_page));

    config.route("/attorney/matters", web::get().to(attorney_matters_page));
    config.route("/attorney/matters/{id}", web::get().to(attorney_matter_page));
    config.route("/attorney/matters/{id}/documents", web::get().to(attorney_matter_documents_page));
    config.route("/attorney/matters/{id}/messages", web::get().to(attorney_matter_messages_page));
    config.route("/attorney/matters/{matter_id}/messages/{message_id}", web::get().to(attorney_matter_post_comments_page));
    config.route("/attorney/matters/{id}/notes", web::get().to(attorney_matter_notes_page));
    config.route("/attorney/matters/{id}/billing", web::get().to(attorney_matter_billing_page));
    config.route("/attorney/matters/{id}/invoices", web::get().to(attorney_matter_invoices_page));

    config.route("/attorney/invoices", web::get().to(attorney_invoices_page));
    config.route("/attorney/billing", web::get().to(attorney_billing_page));
    config.route("/attorney/documents/templates", web::get().to(attorney_templates_page));
    config.route("/documents", web::get().to(documents_page)); 

    config.route("/attorney/contacts", web::get().to(attorney_contacts_page));

    config.route("/attorney/chats/clients", web::get().to(attorney_chats_clients_page));
    config.route("/attorney/chats/opportunities", web::get().to(attorney_chats_opportunities_page));
    config.route("/attorney/chats/leads", web::get().to(attorney_chats_leads_page));
    config.route("/attorney/chats/network", web::get().to(attorney_chats_network_page));
    config.route("/attorney/chats/{id}", web::get().to(attorney_chat_page));

    config.route("/attorney/bank", web::get().to(attorney_bank_page));

    config.route("/attorney/engagement", web::get().to(attorney_engagement_page));
    config.route("/attorney/engagement/topic/{id}", web::get().to(attorney_engagement_topic_page));
    config.route("/attorney/engagement/post/{id}", web::get().to(attorney_engagement_post_page));
    config.route("/attorney/engagement/submitted", web::get().to(attorney_submitted_engagement_page));

    config.route("/forums", web::get().to(attorney_forums_page));
    config.route("/forums/my-posts", web::get().to(attorney_my_posts_page));
    config.route("/forums/following", web::get().to(attorney_following_posts_page));

    config.route("/attorney/news", web::get().to(attorney_news_page));
    config.route("/attorney/news/{id}", web::get().to(attorney_new_page));

    config.route("/attorney/settings/profile", web::get().to(settings_profile_page));
    config.route("/attorney/settings/events", web::get().to(settings_events_page));
    config.route("/attorney/settings/account", web::get().to(settings_account_page)); 
    config.route("/attorney/settings/subscription", web::get().to(settings_subscriptions_page));
    config.route("/attorney/settings/stages", web::get().to(settings_stages_page));
    config.route("/attorney/settings/notifies", web::get().to(settings_notifies_page));
    config.route("/attorney/settings/integrations", web::get().to(settings_integrations_page));
} 


pub async fn attorney_overview_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    if user_some.is_some() {
        println!("user is auth!");
        let request_user = user_some.unwrap();
        
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/overview".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }

        let activities: Vec<crate::views::ActivitiesData>;
        let billing: crate::views::BillingData;
        let chats: Vec<crate::views::ChatsData>;
        let open_matters: Vec<crate::views::OpenMattersData>;
        let open_matters_count: i32;  

        let resp = request_get::<crate::views::AttorneyOnboardingData> (
            API.to_owned()
            + &"users/attorneys/".to_string() + &request_user.user_id + &"/overview/".to_string(),
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            activities = data.activities;
            billing = data.billing;
            chats = data.chats;
            open_matters = data.open_matters;
            open_matters_count = data.open_matters_count;
        }
        else {
            activities = Vec::new();
            billing = crate::views::BillingData {
                overdue:   0,
                paid:      0,
                un_billed: 0.0,
                unpaid:    0,
            };
            chats = Vec::new();
            open_matters = Vec::new();
            open_matters_count = 0;
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/overview.stpl")]
            pub struct Template {
                request_user:       AuthResponseData,
                is_ajax:            i32,
                title:              String,
                description:        String,
                link:               String,
                image:              String,
                activities:         Vec<crate::views::ActivitiesData>,
                billing:            crate::views::BillingData,
                chats:              Vec<crate::views::ChatsData>,
                open_matters:       Vec<crate::views::OpenMattersData>,
                open_matters_count: i32,
            }
            let body = Template {
                request_user:       request_user,
                is_ajax:            is_ajax,
                title:              title,
                description:        description,
                link:               link,
                image:              image,
                activities:         activities,
                billing:            billing,
                chats:              chats,
                open_matters:       open_matters,
                open_matters_count: open_matters_count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/overview.stpl")]
            pub struct Template {
                request_user:       AuthResponseData,
                is_ajax:            i32,
                title:              String,
                description:        String,
                link:               String,
                image:              String,
                activities:         Vec<crate::views::ActivitiesData>,
                billing:            crate::views::BillingData,
                chats:              Vec<crate::views::ChatsData>,
                open_matters:       Vec<crate::views::OpenMattersData>,
                open_matters_count: i32,
            }
            let body = Template {
                request_user:       request_user,
                is_ajax:            is_ajax,
                title:              title,
                description:        description,
                link:               link,
                image:              image,
                activities:         activities,
                billing:            billing,
                chats:              chats,
                open_matters:       open_matters,
                open_matters_count: open_matters_count,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        println!("user is anon!");
        crate::views::login_page(req).await
    }
}

pub async fn attorney_leads_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2; 
        let title: String; 
        let description: String;
        let link = "/attorney/leads".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Leads & Clients".to_string();
            description = "https://app.juslaw.com: Leads & Clients".to_string();
        }
        else { 
            title = "Leads & Clients".to_string();
            description = "https://app.juslaw.com: Leads & Clients".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/leads_clients.stpl")]
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
            #[template(path = "desctop/attorney/leads_clients.stpl")]
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

pub async fn attorney_matters_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/matters".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matters.stpl")]
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
            #[template(path = "desctop/attorney/matters.stpl")]
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

pub async fn attorney_matter_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let matter_id = *_id;
        let link = "/attorney/matters/".to_string() + &matter_id.to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_overview.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_overview.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
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

pub async fn attorney_matter_documents_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let matter_id = *_id;
        let link = "/attorney/matters/".to_string() + &matter_id.to_string() + &"/documents".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_documents.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_documents.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
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

pub async fn attorney_matter_notes_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let matter_id = *_id;
        let link = "/attorney/matters/".to_string() + &matter_id.to_string() + &"/notes".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_notes.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_notes.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
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

pub async fn attorney_matter_billing_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let matter_id = *_id;
        let link = "/attorney/matters/".to_string() + &matter_id.to_string() + &"/billing".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_billing.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_billing.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
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

pub async fn attorney_matter_invoices_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let matter_id = *_id;
        let link = "/attorney/matters/".to_string() + &matter_id.to_string() + &"/invoices".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_invoices.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_invoices.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
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

pub async fn attorney_matter_messages_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let matter_id = *_id;
        let link = "/attorney/matters/".to_string() + &matter_id.to_string() + &"/messages".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_messages.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_messages.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
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

pub async fn attorney_matter_post_comments_page(req: HttpRequest, params: web::Path<(i32, i32)>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let (matter_id, post_id) = (params.0, params.1);
        let link = "/attorney/matters/".to_string() 
            + &matter_id.to_string() 
            + &"/messages/".to_string() 
            + &post_id.to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_comments.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
                post_id:      i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
                post_id:      post_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/matter_comments.stpl")]
            struct Template {
                request_user: AuthResponseData,
                is_ajax:      i32,
                title:        String,
                description:  String,
                link:         String,
                image:        String,
                matter_id:    i32,
                post_id:      i32,
            }
            let body = Template {
                request_user: request_user,
                is_ajax:      is_ajax,
                title:        title,
                description:  description,
                link:         link,
                image:        image,
                matter_id:    matter_id,
                post_id:      post_id,
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

pub async fn attorney_invoices_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/invoices".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/invoices.stpl")]
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
            #[template(path = "desctop/attorney/invoices.stpl")]
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

pub async fn attorney_billing_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/billing".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Billing Items".to_string();
            description = "https://app.juslaw.com: Billing Items".to_string();
        }
        else {  
            title = "Billing Items".to_string();
            description = "https://app.juslaw.com: Billing Items".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/billings.stpl")]
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
            #[template(path = "desctop/attorney/billings.stpl")]
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

pub async fn attorney_templates_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/documents/templates".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Templates".to_string();
            description = "https://app.juslaw.com: Templates".to_string();
        }
        else {  
            title = "Templates".to_string();
            description = "https://app.juslaw.com: Templates".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/templates.stpl")]
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
            #[template(path = "desctop/attorney/templates.stpl")]
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

pub async fn documents_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/documents".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Documents".to_string();
            description = "https://app.juslaw.com: Documents".to_string();
        }
        else {  
            title = "Documents".to_string();
            description = "https://app.juslaw.com: Documents".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/documents.stpl")]
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
            #[template(path = "desctop/attorney/documents.stpl")]
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

pub async fn attorney_contacts_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/contacts".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Industry Contacts".to_string();
            description = "https://app.juslaw.com: Industry Contacts".to_string();
        }
        else {  
            title = "Industry Contacts".to_string();
            description = "https://app.juslaw.com: Industry Contacts".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/contacts.stpl")]
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
            #[template(path = "desctop/attorney/contacts.stpl")]
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

pub async fn attorney_chats_clients_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/chats/clients".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/chat_clients.stpl")]
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
            #[template(path = "desctop/attorney/chat_clients.stpl")]
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

pub async fn attorney_chat_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let chat_id = *_id;
        let title: String; 
        let description: String;
        let link = "/attorney/chats/".to_string() + &_id.to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/chat.stpl")]
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
            #[template(path = "desctop/attorney/chat.stpl")]
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

pub async fn attorney_chats_opportunities_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/chats/opportunities".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/chat_opportunities.stpl")]
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
            #[template(path = "desctop/attorney/chat_opportunities.stpl")]
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

pub async fn attorney_chats_leads_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/chats/leads".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/chat_leads.stpl")]
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
            #[template(path = "desctop/attorney/chat_leads.stpl")]
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

pub async fn attorney_chats_network_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/chats/network".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/chat_network.stpl")]
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
            #[template(path = "desctop/attorney/chat_network.stpl")]
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

pub async fn attorney_bank_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/bank".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Bank Accounts".to_string();
            description = "https://app.juslaw.com: Bank Accounts".to_string();
        }
        else {  
            title = "Bank Accounts".to_string();
            description = "https://app.juslaw.com: Bank Accounts".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/bank.stpl")]
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
            #[template(path = "desctop/attorney/bank.stpl")]
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

pub async fn attorney_engagement_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/engagement".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
        }
        else {  
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/potential.stpl")]
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
            #[template(path = "desctop/attorney/potential.stpl")]
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

pub async fn attorney_engagement_topic_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/engagement/topic/".to_string() + &_id.to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
        }
        else {  
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/engagement_topic.stpl")]
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
            #[template(path = "desctop/attorney/engagement_topic.stpl")]
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

pub async fn attorney_engagement_post_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/engagement/post/".to_string() + &_id.to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
        }
        else {  
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/engagement_post.stpl")]
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
            #[template(path = "desctop/attorney/engagement_post.stpl")]
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

pub async fn attorney_submitted_engagement_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/engagement/submitted".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
        }
        else {  
            title = "Potential Engagement".to_string();
            description = "https://app.juslaw.com: Potential Engagement".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/submitted_potential.stpl")]
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
            #[template(path = "desctop/attorney/submitted_potential.stpl")]
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

pub async fn attorney_forums_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/forums".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
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

pub async fn attorney_my_posts_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/forums/my-posts".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Posts by me".to_string();
            description = "https://app.juslaw.com: Posts by me".to_string();
        }
        else {  
            title = "Posts by me".to_string();
            description = "https://app.juslaw.com: Posts by me".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
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

pub async fn attorney_following_posts_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String;
        let description: String;
        let link = "/attorney/forums/following".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 { 
            title = "Posts".to_string();
            description = "https://app.juslaw.com: Posts".to_string();
        }
        else {  
            title = "Posts".to_string();
            description = "https://app.juslaw.com: Posts".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
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

pub async fn attorney_news_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/news".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
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

pub async fn attorney_new_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/news/".to_string() + &_id.to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
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


pub async fn settings_profile_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/profile".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Edit Profile".to_string();
            description = "https://app.juslaw.com: Edit Profile".to_string();
        }
        else { 
            title = "Edit Profile".to_string();
            description = "https://app.juslaw.com: Edit Profile".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_profile.stpl")]
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
            #[template(path = "desctop/attorney/settings_profile.stpl")]
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

pub async fn settings_events_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/events".to_string();
        let image = crate::utils::get_default_image();
        if l == 2 {
            title = "Edit Profile".to_string();
            description = "https://app.juslaw.com: Edit Profile".to_string();
        }
        else { 
            title = "Edit Profile".to_string();
            description = "https://app.juslaw.com: Edit Profile".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_events.stpl")]
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
            #[template(path = "desctop/attorney/settings_events.stpl")]
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

pub async fn settings_account_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/account".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_account.stpl")]
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
            #[template(path = "desctop/attorney/settings_account.stpl")]
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

pub async fn settings_subscriptions_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/subscription".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_subscription.stpl")]
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
            #[template(path = "desctop/attorney/settings_subscription.stpl")]
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

pub async fn settings_stages_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/stages".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_stages.stpl")]
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
            #[template(path = "desctop/attorney/settings_stages.stpl")]
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

pub async fn settings_notifies_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/notifies".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_notifies.stpl")]
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
            #[template(path = "desctop/attorney/settings_notifies.stpl")]
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

pub async fn settings_integrations_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let l = 2;
        let title: String; 
        let description: String;
        let link = "/attorney/settings/integrations".to_string();
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
        else if !request_user.onboarding {
            return crate::utils::get_onboarding_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        else if !request_user.is_free_subscription && request_user.plan_id.is_none() {
            return crate::utils::get_plans_page (
                request_user,
                is_desctop,
                is_ajax,
                &title,
                &description,
                &link,
                &image,
            ).await;
        }
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/attorney/settings_integrations.stpl")]
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
            #[template(path = "desctop/attorney/settings_integrations.stpl")]
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