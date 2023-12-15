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
    //config.route("/forum/posts/", web::get().to(posts_page));
    //config.route("/forum/topics/", web::get().to(topics_page));
    //config.route("/forum/followed_topics/", web::get().to(followed_topics_page));
}


//////////////  POSTS  //////
//////////////  POSTS //////
#[derive(Debug, Deserialize)]
pub struct PostsParams {
    pub author:   Option<i32>,
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub ordering: Option<String>,
} 

#[derive(Debug, Deserialize)]
pub struct PostComment { 
    pub id:        i32,
    pub post:      i32,
    pub author:    crate::utils::UserCardData,
    pub text:      String,
    pub created:   String,
    pub modified:  String,
    pub user_type: String,
    pub position:  i32,
}

#[derive(Debug, Deserialize)]
pub struct TopicData { 
    pub id:                 i32,
    pub title:              String,
    pub icon:               String,
    pub description:        String,
    pub practice_area:      i32,
    pub practice_area_data: crate::utils::PracticeAreaData,
    pub post_count:         i32,
    pub comment_count:      i32,
    pub followers_count:    i32,
    pub followed:           bool,
}

#[derive(Debug, Deserialize)]
pub struct PostData { 
    pub id:                 i32,
    pub topic:              i32,
    pub title:              String,
    pub last_comment:       Option<PostComment>,
    pub first_comment:      Option<PostComment>,  
    pub comment_count:      i32,
    pub created:            String,
    pub modified:           String,
    pub followers_count:    i32,
    pub followed:           bool,
    pub description:        String,
    pub last_comment_time:  Option<String>,
    pub topic_data:         TopicData,
    pub proposals:          Vec<ProposalsData>,
    pub client:             i32,
    pub client_data:        crate::utils::UserCardData,
    pub practice_area_data: crate::utils::PracticeAreaData,
    pub author:             crate::utils::UserSmallData,
    pub message:            String,
}

#[derive(Debug, Deserialize)]
pub struct PostsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<PostData>,
}
/////////////////////////////////////////////

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
