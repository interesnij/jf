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


pub fn page_routes(config: &mut web::ServiceConfig) {
    //config.route("/business/invoices/", web::get().to(invoices_page));
    //config.route("/business/time-billing/", web::get().to(time_billing_page));
    //config.route("/business/notes/", web::get().to(notes_page));
    //config.route("/business/posted-matter/", web::get().to(posted_matter_page));
    //config.route("/business/posted-matter/practice_area_stats/", web::get().to(posted_matter_stat_page));
}


//////////////  INVOICES  //////
//////////////  INVOICES  //////

#[derive(Debug, Deserialize)]
pub struct InvoicesParams {
    pub limit:        Option<i64>,
    pub offset:       Option<i64>,
    pub search:       Option<String>,
    pub matter:       Option<i32>,
    pub client:       Option<i32>,
    pub attorney:     Option<i32>,
    pub status:       Option<String>,
    pub ordering:     Option<String>,
    pub period_start: Option<chrono::NaiveDate>, 
}

#[derive(Debug, Deserialize)]
pub struct MatterSmallData { 
    pub id:          i32,
    pub code:        String,
    pub title:       String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct BillingItemsSmallData {  
    pub id:             i32,
    pub billed_by:      i32,
    pub billed_by_data: crate::utils::UserCardData,
    pub billing_type:   String,
    pub client:         i32,
    pub created:        String,
    pub currency:       i32,
    pub date:           String,
    pub description:    String,
    pub fees:           i32,
    pub hourly_rate:    String,
    pub is_billable:    bool,
    pub modified:       String,
    pub quantity:       i32,
    pub rate:           String,
    pub time_spent:     Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActivitiesData { 
    pub activity: String,
    pub created:  String,
    pub id:       i32,
    pub modified: String,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceData { 
    pub id:                 i32,
    pub number:             String,
    pub invoice_id:         String,
    pub email:              String,
    pub matter:             i32,
    pub matter_data:        MatterSmallData,
    pub client:             i32,
    pub client_data:        crate::utils::UserCardData,
    pub attorney:           i32,
    pub attorney_data:      crate::utils::UserCardData,
    pub billing_items:      Vec<i32>,
    pub billing_items_data: Vec<BillingItemsSmallData>,
    pub billable_sum:       i32,
    pub activities_data:    Vec<ActivitiesData>,
    pub address:            String,
    pub country:            i32,
    pub country_data:       crate::utils::CountryData,
    pub state:              i32,
    pub state_data:         Option<crate::utils::StateData>,
    pub city:               i32,
    pub city_data:          crate::utils::CityData,
    pub zip_code:           String,
    pub role:               String,
    pub note:               String,
    pub client_type:        String,
    pub organization_name:  String,
    pub message:            String,
    pub sent:               String,
    pub user_type:          String,
}

#[derive(Debug, Deserialize)]
pub struct InvoicesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::views::MatterData>,
}

//////////////  TIME BILLINGS  //////
//////////////  TIME BILLINGS //////


#[derive(Debug, Deserialize)]
pub struct TimeBillingParams {
    pub limit:        Option<i64>,
    pub offset:       Option<i64>,
    pub matter:       Option<i32>,
    pub client:       Option<i32>,
    pub attorney:     Option<i32>,
    pub search:       Option<String>,
    pub billing_type: Option<String>,
    pub status:       Option<String>,
    pub ordering:     Option<String>,
    pub date__gte:    Option<chrono::NaiveDate>,   
} 


#[derive(Debug, Deserialize)]
pub struct TimeEntryData { 
    pub start_time:   String,
    pub end_time:     Option<String>,
    pub billing_item: i32,
    pub created_by:   i32,
}
#[derive(Debug, Deserialize)]
pub struct AttachmentsData { 
    pub file:      String,
    pub file_size: String,
    pub file_name: String,
    pub mime_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TimeBillingData { 
    pub id:                    i32,
    pub matter:                i32,
    pub client:                i32, 
    pub client_data:           crate::utils::UserCardData,
    pub is_billable:           i32,
    pub billed_by:             i32,
    pub billed_by_data:        crate::utils::UserCardData,
    pub matter_data:           MatterSmallData,
    pub created_by:            i32,
    pub created_by_data:       crate::utils::UserSmallData,
    pub invoice:               i32,
    pub description:           String,
    pub time_spent:            String,
    pub hourly_rate:           Option<String>,
    pub currency:              i32,
    pub date:                  String,
    pub available_for_editing: String,
    pub created:               String,
    pub modified:              String,
    pub quantity:              i32,
    pub billing_type:          String,
    pub rate:                  String,
    pub total_amount:          String,
    pub attachments:           Vec<i32>,
    pub attachments_data:      Vec<AttachmentsData>,
    pub time_entries:          Vec<TimeEntryData>,
    pub is_billed:             bool,
}

#[derive(Debug, Deserialize)]
pub struct TimeBillingsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<TimeBillingData>,
    pub total_fees: i32,
    pub total_time: String,
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
    pub budget_detail:          String,
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

