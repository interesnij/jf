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
    get_request_user, UserSmallData, SpecialitiesData,
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    config.route("/client/overview", web::get().to(client_overview_page));
    //config.route("/client/overview/matters", web::get().to(client_overview_matters_page));
    //config.route("/client/overview/invoices", web::get().to(client_overview_invoices_page));
    //config.route("/client/chats", web::get().to(client_chats_page));
    //config.route("/client/find", web::get().to(client_find_page));
    //config.route("/client/forums", web::get().to(client_forums_page));
    //config.route("/client/news", web::get().to(client_news_page));

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
            #[template(path = "desctop/client/overview.stpl")]
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
    pub city_data:         crate::utils::CityData, 
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
