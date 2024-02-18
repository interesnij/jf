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
    get_request_user, UserSmallData, SpecialitiesData, AuthResponseData
};


pub fn matter_routes(config: &mut web::ServiceConfig) {
    //config.route("/business/matters/", web::get().to(matters_page));
    //config.route("/business/matters/{id}/", web::get().to(matter_page));
    //config.route("/business/matter-post/", web::get().to(matter_post_page));
    //config.route("/business/matter-comment/", web::get().to(matter_comment_page));
}


#[derive(Debug, Deserialize)]
pub struct MattersParams {
    pub ordering:    Option<String>,
    pub offset:      Option<i64>,
    pub limit:       Option<i64>,
    pub search:      Option<String>,
    pub status:      Option<String>,
    pub client:      Option<i32>,
    pub attorney:    Option<i32>,
    pub shared_with: Option<Vec<i32>>,
}

//////////////  MATTERS  //////
#[derive(Debug, Deserialize)]
pub struct ClientofMatterData {  
    pub id:           i32,
    pub first_name:   String,
    pub middle_name:  String,
    pub last_name:    String,
    pub email:        String,
    pub avatar:       Option<String>,
    pub phone:        String,
    pub country_data: crate::utils::CountryData,
    pub state_data:   Option<crate::utils::StateData>,
    pub city_data:    Option<crate::utils::CityData>,
    pub address1:     String,
    pub address2:     String,
    pub zip_code:     String,
}

#[derive(Debug, Deserialize)]
pub struct AttorneyofMatterData {  
    pub id:          i32,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub avatar:      Option<String>,
    pub phone:       String,
}
#[derive(Debug, Deserialize)]
pub struct ReferralMatterData {  
    pub id:                      i32,
    pub attorney:                i32,
    pub first_name:              String,
    pub middle_name:             String,
    pub last_name:               String,
    pub email:                   String,
    pub phone:                   String,
    pub avatar:                  Option<String>,
    pub is_verified:             bool,
    pub verification_status:     String,
    pub featured:                bool,
    pub sponsored:               bool,
    pub has_active_subscription: bool,
    pub specialities:            Vec<i32>,
    pub is_attorney:             bool,
    pub is_paralegal:            bool,
}

#[derive(Debug, Deserialize)]
pub struct UserrCardData { 
    pub id:          i32,
    pub first_name:  String,
    pub middle_name: String,
    pub last_name:   String,
    pub email:       String,
    pub phone:       String,
    pub avatar:      Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InviteMatterData {
    pub uuid:              String,
    pub user:              crate::utils::UserCardData,
    pub client:            crate::utils::UserCardData,
    pub first_name:        String,
    pub middle_name:       String,
    pub last_name:         String,
    pub email:             String,
    pub phone:             String,
    pub address:           String,
    pub country:           i32,
    pub country_data:      crate::utils::CountryData,
    pub state:             i32,
    pub state_data:        Option<crate::utils::StateData>,
    pub city:              i32,
    pub city_data:         crate::utils::CityData,
    pub zip_code:          String,
    pub role:              String,
    pub note:              String,
    pub client_type:       String,
    pub organization_name: String,
    pub message:           String,
    pub sent:              String,
    pub user_type:         String,
}

#[derive(Debug, Deserialize)]
pub struct EnvelopeDocumentData { 
    pub id:    i32,
    pub file:  String,
    pub order: i32,
    pub name:  String,
}

#[derive(Debug, Deserialize)]
pub struct EnvelopeMatterData { 
    pub id:          i32,
    pub docusign_id: i32,
    pub matter:      i32,
    pub status:      String,
    pub r#type:      String,
    pub documents:   Vec<EnvelopeDocumentData>,
}

#[derive(Debug, Deserialize)]
pub struct MatterData {  
    pub id:              i32,
    pub lead:            Option<i32>,
    pub client:          i32,
    pub client_data:     ClientofMatterData,
    pub attorney:        i32,
    pub attorney_data:   AttorneyofMatterData,
    pub code:            String,
    pub title:           String,
    pub description:     String,
    pub fees_earned:     Option<String>,
    pub rate:            String,
    pub rate_type:       Vec<crate::utils::RateTypeData>,
    pub country:         i32,
    pub country_data:    crate::utils::CountryData,
    pub speciality:      i32,
    pub speciality_data: crate::utils::SpecialitiesData,
    pub state:           i32,
    pub state_data:      Option<crate::utils::StateData>,
    pub city:            i32,
    pub city_data:       crate::utils::CityData,
    pub status:          String,
    pub stage:           i32, 
    pub stage_data:      crate::utils::StageData,
    pub referral:        Option<i32>,
    pub referral_data:   Option<ReferralMatterData>,
    pub referral_request: bool,
    pub referral_pending: bool,
    pub referral_ignored: bool,
    pub referral_ignore_attorney_data: Option<UserrCardData>,
    pub completed:        Option<String>,
    pub shared_with:      Vec<i32>,
    pub shared_with_data: Vec<crate::utils::UserSharedData>,
    pub is_shared:        bool,
    pub created:          String,
    pub modified:         String,
    pub currency:         Vec<i32>,
    pub currency_data:    Vec<crate::utils::FeeCurrencyData>,
    pub start_date:       String,
    pub close_date:       Option<String>,
    pub is_billable:      bool,
    pub fee_type:         i32,
    pub fee_note:         String,
    pub invite:          Option<i32>,
    //pub invite_data:     Option<InviteMatterData>,

    pub unread_document_count: i32,
    pub unread_message_count:  i32,
    pub due_amount:            i32,
    //pub envelope_data:    Option<EnvelopeMatterData>,
} 

#[derive(Debug, Deserialize)]
pub struct MattersData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<MatterData>,
}


//////////////  MATTER DETAIL  //////
//////////////  MATTER DETAIL  //////

#[derive(Debug, Deserialize)]
pub struct ActivitieData {
    pub id:        i32,
    pub title:     String,
    pub user_data: crate::utils::UserCardData,
    pub created:   String,
    pub matter:    i32,
}

#[derive(Debug, Deserialize)]
pub struct AttachmentData { 
    pub file:       String,
    pub name:       String,
    pub size:       String,
    pub mime_type:  String,
    pub created_by: i32,
    pub matter:     i32,
    pub owner:      i32,
}

#[derive(Debug, Deserialize)]
pub struct MatterDetailData {   
    pub id:                    i32,
    pub lead:                  Option<i32>,
    pub client:                i32,
    pub client_data:           ClientofMatterData,
    pub attorney:              i32,
    pub attorney_data:         AttorneyofMatterData,
    pub code:                  String,
    pub title:                 String,
    pub description:           String,
    pub fees_earned:           Option<String>,
    pub rate:                  String,
    pub rate_type:             Vec<crate::utils::RateTypeData>,
    pub country:               i32,
    pub country_data:          crate::utils::CountryData,
    pub speciality:            i32,
    pub speciality_data:       crate::utils::SpecialitiesData,
    pub state:                 i32,
    pub state_data:            Option<crate::utils::StateData>,
    pub city:                  i32,
    pub city_data:             crate::utils::CityData,
    pub status:                String,
    pub stage:                 i32, 
    pub stage_data:            crate::utils::StageData,
    pub referral:              Option<i32>,
    pub referral_data:         Option<ReferralMatterData>,
    pub referral_request:      bool,
    pub referral_pending:      bool,
    pub referral_ignored:      bool,
    pub referral_ignore_attorney_data: Option<UserrCardData>,
    pub completed:             Option<String>,
    pub shared_with:           Vec<i32>,
    pub shared_with_data:      Vec<crate::utils::UserSharedData>,
    pub is_shared:             bool,
    pub created:               String,
    pub modified:              String,
    pub currency:              Vec<i32>,
    pub currency_data:         Vec<crate::utils::FeeCurrencyData>,
    pub start_date:            String,
    pub close_date:            Option<String>,
    pub is_billable:           bool,
    pub fee_type:              i32,
    pub fee_note:              String,
    pub invite:                Option<i32>,
    pub invite_data:           Option<InviteMatterData>,
    pub unread_document_count: i32,
    pub unread_message_count:  i32,
    pub due_amount:            i32,
    pub envelope_data:         Vec<i32>,
    pub unread_comment_count:  i32,
    pub total_amount:          i32,
    pub paid:                  i32,
    pub unpaid:                i32,
    pub overdue:               i32,
    pub unbilled:              i32,
    pub activities:            Vec<ActivitieData>,
    pub attachment:            Vec<AttachmentData>,

}

//////////////  MATTER POSTS  //////
//////////////  MATTER POSTS  //////

#[derive(Debug, Deserialize)]
pub struct MatterPostsParams {
    pub ordering: Option<String>,
    pub offset:   Option<i64>,
    pub limit:    Option<i64>,
    pub search:   Option<String>,
    pub matter:   Option<i32>,
    pub seen:     Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AttachmenttData {  
    pub file:       String,
    pub file_size:  String,
    pub file_nane:  String,
    pub mime_type:  String,
}
