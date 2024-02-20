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
use serde::{Deserialize, Serialize};
use crate::utils::{
    get_request_user, UserSmallData, SpecialitiesData, AuthResponseData,
};


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/terms-of-use", web::get().to(terms_page));
    config.route("/privacy-policy", web::get().to(policy_page));
}


pub async fn terms_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let l = 2;
    let title: String; 
    let description: String;
    let link = "/terms-of-use".to_string();
    let image = crate::utils::get_default_image();
    if l == 2 {
        title = "Terms of Use".to_string();
        description = "https://app.juslaw.com: Terms of Use".to_string();
    }
    else { 
        title = "Terms of Use".to_string();
        description = "https://app.juslaw.com: Terms of Use".to_string();
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

    else if user_some.is_some() {
        let request_user = user_some.unwrap();

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/terms.stpl")]
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
            #[template(path = "desctop/pages/terms.stpl")]
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

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_terms.stpl")]
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
            #[template(path = "desctop/pages/anon_terms.stpl")]
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

pub async fn policy_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let l = 2;
    let title: String; 
    let description: String;
    let link = "/privacy-policy".to_string();
    let image = crate::utils::get_default_image();
    if l == 2 {
        title = "JusLaw Privacy Policy".to_string();
        description = "https://app.juslaw.com: JusLaw Privacy Policy".to_string();
    }
    else { 
        title = "JusLaw Privacy Policy".to_string();
        description = "https://app.juslaw.com: JusLaw Privacy Policy".to_string();
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

    else if user_some.is_some() {
        let request_user = user_some.unwrap();

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/policy.stpl")]
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
            #[template(path = "desctop/pages/policy.stpl")]
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

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_policy.stpl")]
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
            #[template(path = "desctop/pages/anon_policy.stpl")]
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
pub struct ActivitiesData {
    pub avatar:   Option<String>,
    pub created:  String,
    pub id:       i32,
    pub matter:   i32,
    pub modified: String,
    pub title:    String,
    pub r#type:   String,
}

///////////////////////////// Attorney board /////////
#[derive(Debug, Deserialize)]
pub struct OpenMattersData {
    pub client:           i32,
    pub client_avatar:    Option<String>,
    pub client_name:      String,
    pub created:          String,
    pub fee_type:         String,
    pub id:               i32,
    pub practice_area:    String,
    pub principle_avatar: Option<String>,
    pub principle_name:   String,
    pub title:            String,
}

#[derive(Debug, Deserialize)]
pub struct BillingData {
    pub overdue:   i32,
    pub paid:      i32,
    pub un_billed: f32,
    pub unpaid:    i32,
} 

#[derive(Debug, Deserialize)]
pub struct ClientData {
    pub id:          i32,
    pub avatar:      Option<String>,
    pub email:       String,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
}
#[derive(Debug, Deserialize)]
pub struct LastMessageData { 
    pub chat:       i32,
    pub created:    String,
    pub id:         i32,
    pub files:      Vec<String>,
    pub text:       String,
    pub timestamp1: Option<String>,
    pub r#type:     String,
}

#[derive(Debug, Deserialize)]
pub struct ChatsData {
    pub chat_type:    String,
    pub client_data:  ClientData,
    pub created:      String,
    pub id:           i32,
    pub is_archived:  bool,
    pub is_favorite:  bool,
    pub is_group:     bool,
    pub last_message: LastMessageData,
    pub title:        Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AttorneyOnboardingData {
    pub activities:         Vec<ActivitiesData>,
    pub billing:            BillingData,
    pub chats:              Vec<ChatsData>,
    pub open_matters:       Vec<OpenMattersData>,
    pub open_matters_count: i32,
}


///////////////////////////// Client board /////////

#[derive(Debug, Deserialize)]
pub struct RecentMattersData {
    pub attorney_avatar:       Option<String>,
    pub attorney_email:        String,
    pub attorney_id:           i32,
    pub attorney_name:         String,
    pub created:               String,
    pub description:           String,
    pub due_amount:            i32,
    pub id:                    i32,
    pub rate_type:             Vec<crate::utils::RateTypeData>,
    pub shared_with_data:      Vec<crate::utils::UserSharedData>,
    pub speciality_data:       Vec<SpecialitiesData>,
    pub stage:                 Option<String>,
    pub start_date:            String,
    pub status:                String,
    pub title:                 String,
    pub unread_document_count: i32,
    pub unread_message_count:  i32,
}

#[derive(Debug, Deserialize)] 
pub struct UpcomingBillsData {
    pub id:           i32,
    pub invoice_id:   String,
    pub title:        String,
    pub invoice_from: String,
    pub status:       String,
    pub due_date:     String,
    pub amount:       String,
} 

#[derive(Debug, Deserialize)] 
pub struct RecentDocumentsData {
    pub created:          String,
    pub file_size:        i32,
    pub shared_with_size: i32,
    pub title:            String,
    pub uploaded_by:      String,
}

#[derive(Debug, Deserialize)]
pub struct ClientOnboardingData { 
    pub recent_activities: Vec<ActivitiesData>,
    pub recent_documents:  Vec<RecentDocumentsData>,
    pub recent_matters:    Vec<RecentMattersData>,
    pub upcoming_bills:    Vec<UpcomingBillsData>,
}
////////////////////////////////////////////
