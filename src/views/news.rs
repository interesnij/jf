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
    get_request_user, UserSmallData, SpecialitiesData,
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    //config.route("/news/", web::get().to(news_page));
    //config.route("/news/{id}/", web::get().to(new_page));
}

//////////////  NEWS  //////
//////////////  NEWS  //////

#[derive(Debug, Deserialize)]
pub struct LeadsAndClientsParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct NewData { 
    pub id:              i32,
    pub title:           String,
    pub description:     Option<String>,  
    pub image:           Option<String>,
    pub image_thumbnail: Option<String>,
    pub tags:            Vec<String>,
    pub categories:      Vec<String>,
    pub created:         String,
    pub modified:        String,
    pub author:          i32,
}

#[derive(Debug, Deserialize)]
pub struct NewsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<NewData>,
}


//////////////  NEWS DETAIL  //////
//////////////  NEWS DETAIL //////


/// NewData