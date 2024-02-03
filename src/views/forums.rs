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
    get_request_user, UserSmallData, SpecialitiesData, AuthResponseData
};


pub fn forum_routes(config: &mut web::ServiceConfig) {
    //config.route("/forum/posts/", web::get().to(posts_page));
    //config.route("/forum/topics/", web::get().to(topics_page));
    //config.route("/forum/followed_topics/", web::get().to(followed_topics_page));
}


//////////////  TOPICS  //////
//////////////  TOPICS  //////

#[derive(Debug, Deserialize)]
pub struct TopicStatParams {
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub ordering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TopicsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<TopicData>,
}
/////////////////////////////////////////////


//////////////  FOLLOWED TOPICS  //////
//////////////  FOLLOWED TOPICS  //////

#[derive(Debug, Deserialize)]
pub struct FollowedTopicData { 
    pub id:         i32,
    pub title:      String,
    pub post_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct FollowedTopicsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<FollowedTopicData>,
}
/////////////////////////////////////////////
