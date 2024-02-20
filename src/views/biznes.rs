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


pub fn biznes_routes(config: &mut web::ServiceConfig) {
    //config.route("/business/invoices/", web::get().to(invoices_page));
    //config.route("/business/time-billing/", web::get().to(time_billing_page));
    //config.route("/business/notes/", web::get().to(notes_page));
    //config.route("/business/posted-matter/", web::get().to(posted_matter_page));
    //config.route("/business/posted-matter/practice_area_stats/", web::get().to(posted_matter_stat_page));
}


//////////////  NOTES  //////
//////////////  NOTES //////

#[derive(Debug, Deserialize)]
pub struct NotesParams {
    pub limit:      Option<i64>,
    pub offset:     Option<i64>,
    pub search:     Option<String>,
    pub matter:     Option<i32>,
    pub client:     Option<i32>,
    pub attorney:   Option<i32>,
    pub created_by: Option<i32>,
    pub ordering:   Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct NoteData {  
    pub id:               i32,
    pub title:            String,
    pub text:             String,
    pub matter:           i32,
    pub matter_data:      MatterSmallData,
    pub client:           Option<i32>, 
    pub client_data:      Option<crate::utils::UserCardData>,
    pub created_by:       i32,
    pub created_by_data:  crate::utils::UserSmallData, 
    pub created:          String,
    pub modified:         String,
    pub attachments:      Vec<i32>,
    pub attachments_data: Vec<AttachmentsData>,
}

#[derive(Debug, Deserialize)]
pub struct NotesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<NoteData>,
}
/////////////////////////////


//////////////  POSTEDMATTER  //////
//////////////  POSTEDMATTER //////

#[derive(Debug, Deserialize)]
pub struct PostedMatterParams {
    pub client:   Option<i32>,
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub ordering: Option<String>,
    pub status:   Option<String>, 
}

#[derive(Debug, Deserialize)]
pub struct ProposalsData { 
    pub id:                     i32,
    pub name:                   String,
    pub created:                String,
    pub rate:                   String,
    pub rate_type:              String,
    pub description:            String,
    pub attorney_data:          crate::utils::UserCardData,
    pub status:                 String,
    pub status_modified:        String,
    pub currency:               Vec<i32>,
    pub currency_data:          Vec<crate::utils::FeeCurrencyData>,
    pub is_hidden_for_client:   bool,
    pub is_hidden_for_attorney: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostedMatterData { 
    pub id:                     i32,
    pub title:                  String,
    pub description:            String,
    pub budget_min:             String,
    pub budget_max:             String,
    pub budget_type:            String,
    //pub budget_detail:          Option<String>,
    pub practice_area:          i32,
    pub created:                String,
    pub proposals:              Vec<ProposalsData>,
    pub client:                 i32,
    pub client_data:            crate::utils::UserCardData,
    pub practice_area_data:     crate::utils::PracticeAreaData,
    pub currency:               Vec<i32>,
    pub currency_data:          Vec<crate::utils::FeeCurrencyData>,
    pub status:                 String,
    pub status_modified:        String,
    pub is_hidden_for_client:   bool,
    pub is_hidden_for_attorney: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostedMattersData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<PostedMatterData>,
}
/////////////////////////////


//////////////  POSTEDMATTER STAT  //////
//////////////  POSTEDMATTER STAT  //////

#[derive(Debug, Deserialize)]
pub struct PostedMatterStatParams {
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub ordering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostedMattersStatData { 
    pub id:                           i32,
    pub title:                        String,
    pub description:                  Option<String>,  
    pub last_posted:                  Option<String>,
    pub posted_matters_count:         i32,
    pub posted_matters:               Vec<PostedMatterData>,
    pub created:                      String,
    pub latest_posted_matter_created: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostedMattersStatsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<PostedMattersStatData>,
}

