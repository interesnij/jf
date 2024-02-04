use actix_web::{
    web,
    //web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
//use crate::errors::Error;
use sailfish::TemplateOnce;
use serde::{
    Deserialize, 
    //Serialize
};
use crate::utils::{
    get_string_with_string, get_limit, get_string_withi64, get_string_withi32, 
    get_string_withbool, get_request_user, AuthResponseData, request_get, API,
    get_id_withi32, get_string_withdate, gett_string_withi32,
};


pub fn load_routes(config: &mut web::ServiceConfig) {
    config.route("/attorney_load/leads_and_clients", web::get().to(leads_and_clients_load));
    config.route("/attorney_load/matters", web::get().to(attorney_matters_load));
    config.route("/load/documents", web::get().to(documents_load));
    config.route("/load/invoices", web::get().to(invoices_load));
    config.route("/load/billing", web::get().to(billing_load));
    config.route("/load/industry_contacts", web::get().to(contacts_load));
    config.route("/load/chats", web::get().to(chats_load));
    config.route("/load/messages/{id}", web::get().to(messages_load));
    config.route("/load/matter_messages", web::get().to(matter_messages_load));
    config.route("/load/matter_post_comments", web::get().to(matter_post_comments));
    config.route("/load/notes", web::get().to(notes_load));
    config.route("/load/posts", web::get().to(posts_load));
    config.route("/load/topics", web::get().to(topics_load));
    config.route("/load/events", web::get().to(events_load));
    //config.route("/locations/states/", web::get().to(states_load));
    //config.route("/locations/cities/", web::get().to(cities_load));
    //config.route("/users/specialities/", web::get().to(specialities_load));
    //config.route("/users/clients/search_attorneys_and_paralegals/", web::get().to(search_attorneys_and_paralegals));
    //config.route("/attorneys/stages/", web::get().to(stages_page));
}


#[derive(Debug, Deserialize)]
pub struct LeadsAndClientsParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
    //pub r#type: Option<String>,
} 

#[derive(Debug, Deserialize)]
pub struct LeadOrClientData {  
    pub id:            String,
    pub first_name:    String,
    pub middle_name:   String,
    pub last_name:     String,
    pub phone:         String,
    pub avatar:        Option<String>,
    pub job:           String,
    pub company:       Option<String>, 
    pub country_data:  crate::utils::CountryData,
    pub state_data:    Option<crate::utils::StateData>,
    pub city_data:     Option<crate::utils::CityData>,
    pub address:       String,
    pub zipcode:       String,
    pub note:          Option<String>,
    pub r#type:        String,
    pub email:         String,
    pub matters_count: i32,
    pub is_pending:    bool,
}

#[derive(Debug, Deserialize)]
pub struct LeadsAndClientsData { 
    //pub count:      i32,
    //pub next:       Option<String>,
    //pub page_count: i32,
    //pub previous:   Option<String>,
    pub results:    Vec<LeadOrClientData>,
}

pub async fn leads_and_clients_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<LeadOrClientData>;

        let limit:  String;
        let offset: String;
        let search: String;
        let _type:  String;

        let params_some = web::Query::<LeadsAndClientsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            //_type =  get_string_with_string(params.r#type.clone());
            _type =  String::new();
        }
        else {
            limit =  String::new();
            offset = String::new();
            search = String::new();
            _type =  String::new();
        }

        let url = concat_string!(
            API.to_owned(),
            "users/attorneys/", request_user.user_id, "/leads_and_clients/",
            "?search=", search,
            "&offset=", offset,
            "&limit=", limit,
            "&type=", _type
        );
        let resp = request_get::<LeadsAndClientsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = 0;
            next = None;
            page_count = 0;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/leads_and_clients.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<LeadOrClientData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}

#[derive(Debug, Deserialize)]
pub struct AttorneyMattersParams {
    pub ordering:    Option<String>,
    pub offset:      Option<i64>,
    pub limit:       Option<i64>,
    pub search:      Option<String>,
    pub attorney:    Option<i32>,
    pub status:      Option<String>,
    pub shared_with: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EnvelopeMatterrData {  
    pub id:          i32,
    pub docusign_id: i32,
    pub matter:      i32,
    pub status:      String,
    pub r#type:      String,
    pub documents:   Vec<crate::views::EnvelopeDocumentData>,
}

#[derive(Debug, Deserialize)]
pub struct MatterrrData { 
    pub id:              i32,
    pub lead:            Option<i32>,
    pub client:          i32,
    pub client_data:     crate::views::ClientofMatterData,
    pub attorney:        i32,
    pub attorney_data:   crate::views::AttorneyofMatterData,
    pub code:            String, 
    pub title:           String,
    pub description:     String,
    pub fees_earned:     Option<String>,
    pub rate:            String,
    pub rate_type:       crate::utils::RateTypeData,
    pub country:         i32, 
    pub country_data:    crate::utils::CountryData,
    pub speciality:      i32,
    pub speciality_data: crate::utils::SpecialitiesData,
    pub state:           Option<i32>,
    pub state_data:      Option<crate::utils::StateData>,
    pub city:            Option<i32>,
    pub city_data:       crate::utils::CityData,
    pub status:          String,
    pub stage:           i32, 
    pub stage_data:      crate::utils::StageData,
    pub referral:        Option<i32>,
    pub referral_data:   Option<crate::views::ReferralMatterData>,
    pub referral_request: bool,
    pub referral_pending: bool,
    pub referral_ignored: bool,
    pub referral_ignore_attorney_data: Option<crate::views::UserrCardData>,
    pub completed:        Option<String>,
    pub shared_with:      Vec<i32>,
    pub shared_with_data: Vec<crate::utils::UserSmallData>,
    pub is_shared:        bool,
    pub created:          String,
    pub modified:         String,
    pub currency:         i32,
    pub currency_data:    crate::utils::FeeCurrencyData,
    pub start_date:       String,
    pub close_date:       Option<String>,
    pub is_billable:      bool,
    pub fee_type:         i32,
    pub fee_note:         String,
    pub invite:          Option<i32>, 
    pub invite_data:     Option<crate::views::InviteMatterData>,

    pub unread_document_count: i32,
    pub unread_message_count:  i32,
    pub due_amount:            i32,
    pub envelope_data:         Option<EnvelopeMatterrData>, 
}  



#[derive(Debug, Deserialize)]
pub struct AttorneyMattersData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<MatterrrData>,
}

pub async fn attorney_matters_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<MatterrrData>;

        let ordering:    String;
        let limit:       String;
        let offset:      String;
        let search:      String;
        let attorney:    String;
        let status:      String;
        let shared_with: String;

        let params_some = web::Query::<AttorneyMattersParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            ordering = get_string_with_string(params.ordering.clone());
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            attorney = get_string_withi32(params.attorney);
            status = get_string_with_string(params.status.clone());
            shared_with = get_string_withi32(params.shared_with.clone());
        }
        else {
            ordering = String::new();
            limit =  String::new();
            offset = String::new();
            search = String::new();
            attorney = String::new();
            status = String::new();
            shared_with = String::new();
        }

        let url = concat_string!(
            API.to_owned(),
            "business/matters/?ordering=", ordering,
            "&offset=", offset,
            "&limit=", limit,
            "&search=", search,
            "&attorney=", attorney,
            "&status=", status,
            "&shared_with=", shared_with
        );
        let resp = request_get::<crate::views::AttorneyMattersData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/attorney_matters.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<MatterrrData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}


/////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub struct DocumentsParams {
    pub ordering:    Option<String>,
    pub offset:      Option<i64>,
    pub limit:       Option<i64>,
    pub search:      Option<String>,
    pub status:      Option<String>,
    pub is_template: Option<bool>,
    pub is_parent:   Option<bool>,
    pub is_vault:    Option<bool>,
    pub client:      Option<i32>,
    pub attorney:    Option<i32>,
    pub matter:      Option<i32>,
    pub owner:       Option<i32>,
    pub r#type:      Option<String>,
    pub shared_with: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MatterSmallData { 
    pub id:          i32,
    pub code:        String,
    pub title:       String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct DocumentData { 
    pub id:                 i32,
    pub owner:              i32,
    pub owner_data:         crate::utils::UserCardData,
    pub matter:             i32,
    pub matter_data:        MatterSmallData,
    pub client:             Option<i32>,
    pub client_data:        Option<crate::utils::UserCardData>,
    pub created:            String,
    pub modified:           String,
    pub parent:             Option<i32>,
    pub title:              String,
    pub is_template:        bool,
    pub is_vault:           bool,
    pub is_global_template: bool,
    pub file:               String,
    pub mime_type:          Option<String>,
    pub shared_with:        Vec<i32>,
    pub shared_with_data:   Vec<crate::utils::UserSmallData>,
    pub created_by:         i32,
    pub created_by_data:    crate::utils::UserCardData,
    //pub r#type:             String, 
}

#[derive(Debug, Deserialize)]
pub struct DocumentsData { 
    pub next:          Option<String>,
    pub page_count:    i32,
    pub previous:      Option<String>,
    pub results:       Vec<DocumentData>,
    pub highest_count: i32,
    pub overall_total: i32,
}

pub async fn documents_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2; 

        let next:          Option<String>;
        let page_count:    i32;
        let object_list:   Vec<DocumentData>;
        let highest_count: i32;
        let overall_total: i32;

        let ordering:    String;
        let offset:      String;
        let limit:       String;
        let search:      String;
        let status:      String;
        let is_template: String;
        let is_parent:   String;
        let is_vault:    String;
        let client:      String;
        let attorney:    String;
        let matter:      String;
        let owner:       String;
        //let _type:       String;
        let shared_with: String;

        let params_some = web::Query::<DocumentsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            ordering = get_string_with_string(params.ordering.clone());
            offset = get_string_withi64(params.offset);
            limit =  get_limit(params.limit);
            search = get_string_with_string(params.search.clone());
            status = get_string_with_string(params.status.clone());
            is_template = get_string_withbool(params.is_template);
            is_parent = get_string_withbool(params.is_parent);
            is_vault = get_string_withbool(params.is_vault);
            client = gett_string_withi32(params.client, "&client=".to_string()); 
            attorney = gett_string_withi32(params.attorney, "&attorney=".to_string());
            matter = gett_string_withi32(params.matter, "&matter=".to_string()); 
            owner = gett_string_withi32(params.owner, "&owner=".to_string());
            //_type = get_string_with_string(params.r#type.clone());
            shared_with = gett_string_withi32(params.shared_with, "&shared_with=".to_string());
        }
        else {
            ordering = String::new();
            limit =  String::new();
            offset = String::new();
            search = String::new();
            status = String::new();
            is_template = String::new();
            is_parent = String::new();
            is_vault = String::new();
            client = String::new();
            attorney = String::new();
            matter = String::new();
            owner = String::new();
            //_type = String::new();
            shared_with = String::new();
        }
        let url = concat_string!(
            API.to_owned(),
            "documents/?ordering=", ordering,
            "&limit=", limit,
            "&offset=", offset,
            "&search=", search,
            "&status=", status,
            "&is_template=", is_template,
            "&is_parent=", is_parent,
            "&is_vault=", is_vault,
            client,
            attorney,
            matter,
            owner,
            //"&type=", _type,
            shared_with
        );
        let resp = request_get::<crate::views::DocumentsData> (
            url,
            &request_user.key
        ).await; 

        if resp.is_ok() {
            let data = resp.expect("E.");
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
            highest_count = data.highest_count;
            overall_total = data.overall_total;
        }
        else {
            next = None;
            page_count = 0;
            object_list = Vec::new();
            highest_count = 0;
            overall_total = 0;
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/documents.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            next:          Option<String>,
            page_count:    i32,
            object_list:   Vec<DocumentData>,
            highest_count: i32,
            overall_total: i32,
        }
        let body = Template {
            request_user:  request_user,
            next:          next,
            page_count:    page_count,
            object_list:   object_list,
            highest_count: highest_count,
            overall_total: overall_total,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}


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
    pub activities_data:    Vec<crate::views::ActivitiesData>,
    pub address:            String,
    pub country:            i32,
    pub country_data:       crate::utils::CountryData,
    pub state:              i32,
    pub state_data:         Option<crate::utils::StateData>,
    pub city:               i32,
    pub city_data:          Option<crate::utils::CityData>,
    pub zip_code:           String,
    pub role:               String,
    pub note:               String,
    pub client_type:        String,
    pub organization_name:  String,
    pub message:            String,
    pub sent:               String,
    pub user_type:          String,
    pub due_date:           String,
    pub created:            String,
}

#[derive(Debug, Deserialize)]
pub struct InvoicesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<InvoiceData>,
}

pub async fn invoices_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<InvoiceData>;

        let limit:        String;
        let offset:       String;
        let search:       String;
        let matter:       String;
        let client:       String;
        let attorney:     String;
        let status:       String;
        let ordering:     String;
        let period_start: String;

        let params_some = web::Query::<InvoicesParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            matter = get_string_withi32(params.matter);
            client = get_string_withi32(params.client);
            attorney = get_string_withi32(params.attorney);
            status = get_string_with_string(params.status.clone());
            ordering = get_string_with_string(params.ordering.clone());
            period_start = get_string_withdate(params.period_start.clone());
        }
        else {
            limit = String::new();
            offset = String::new();
            search = String::new();
            matter = String::new();
            client = String::new();
            attorney = String::new();
            status = String::new();
            ordering = String::new();
            period_start = String::new();
        }

        let url = concat_string!(
            API.to_owned(),
            "business/invoices",
            "?search=", search,
            "&offset=", offset,
            "&limit=", limit,
            "&matter=", matter,
            "&client=", client,
            "&attorney=", attorney,
            "&status=", status,
            "&ordering=", ordering,
            "&period_start=", period_start
        );
        let resp = request_get::<crate::views::InvoicesData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/invoices.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<InvoiceData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}

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

pub async fn billing_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<TimeBillingData>;

        let limit:        String;
        let offset:       String;
        let search:       String;
        let billing_type: String;
        let matter:       String;
        let client:       String;
        let attorney:     String;
        let status:       String;
        let ordering:     String;
        let date__gte:    String;
        let total_fees:   i32;
        let total_time:   String;

        let params_some = web::Query::<TimeBillingParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            billing_type = get_string_with_string(params.billing_type.clone());
            matter = get_string_withi32(params.matter);
            client = get_string_withi32(params.client);
            attorney = get_string_withi32(params.attorney);
            status = get_string_with_string(params.status.clone());
            ordering = get_string_with_string(params.ordering.clone());
            date__gte = get_string_withdate(params.date__gte.clone());
        }
        else {
            limit = String::new();
            offset = String::new();
            search = String::new();
            billing_type = String::new();
            matter = String::new();
            client = String::new();
            attorney = String::new();
            status = String::new();
            ordering = String::new();
            date__gte = String::new();
        }

        let url = concat_string!(
            API.to_owned(),
            "business/time-billing",
            "?search=", search,
            "&offset=", offset,
            "&limit=", limit,
            "&billing_type=", billing_type,
            "&matter=", matter,
            "&client=", client,
            "&attorney=", attorney,
            "&status=", status,
            "&ordering=", ordering,
            "&date__gte=", date__gte
        );
        let resp = request_get::<crate::views::TimeBillingsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
            total_fees = data.total_fees;
            total_time = data.total_time;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
            total_fees = 0;
            total_time = String::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/billing.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<TimeBillingData>,
            total_fees:   i32,
            total_time:   String,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
            total_fees:   total_fees, 
            total_time:   total_time,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}


#[derive(Debug, Deserialize)]
pub struct ContactsParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub r#type: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct ContactData { 
    pub user_id: i32,
    pub name:    String,
    pub firm:    Option<String>,
    pub r#type:  String,
    pub phone:   String,
    pub pending: bool,
    pub email:   String,
    pub avatar:  Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct ContactsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<ContactData>,
}

pub async fn contacts_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<ContactData>;

        let limit:    String;
        let offset:   String;
        let search:   String;
        let _type:    String;

        let params_some = web::Query::<ContactsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            _type = get_string_with_string(params.r#type.clone());
        }
        else {
            limit = String::new();
            offset = String::new();
            search = String::new();
            _type = String::new();
        }

        let url = concat_string!(
            API.to_owned(),
            "/users/attorneys/", request_user.user_id,  "/industry_contacts/",
            "?search=", search,
            "&offset=", offset,
            "&limit=", limit,
            "&type=", _type
        );
        let resp = request_get::<crate::views::ContactsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/contacts.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<ContactData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}
/////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct ChatsParams {
    pub ordering:  Option<String>,
    pub chat_type: Option<String>,
    pub chat_id:   Option<i32>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct AddressData { 
    pub country:  String,
    pub state:    String,
    pub city:     String,
    pub address1: String,
    pub address2: String,
    pub zip_code: String,
} 
#[derive(Debug, Deserialize, Clone)]
pub struct UserChatCardData { 
    pub id:             i32,
    pub first_name:     String,
    pub middle_name:    String,
    pub last_name:      String,
    pub email:          String,
    pub phone:          String,
    pub avatar:         Option<String>,
    pub address:        AddressData,
    pub user_type:      String,
    pub opportunity_id: Option<i32>, 
    pub lead_id:        Option<i32>,
}
#[derive(Debug, Deserialize)]
pub struct FilesData { 
    pub message: i32,
    pub title:   String,
    pub size:    String,
    pub file:    String,
} 
#[derive(Debug, Deserialize)]
pub struct MessageData { 
    pub id:          i32, 
    pub author:      i32,
    pub author_data: crate::utils::UserCardData,
    pub r#type:      String,
    pub chat:        i32,
    pub text:        Option<String>,
    pub files:       Vec<FilesData>,
    pub timestamp1:  Option<String>,
    pub created:     String,
}
impl MessageData {
    pub fn get_preview(&self) -> String {
        if self.text.is_some() {
            let _text = self.text.as_deref().unwrap();
            if _text.len() > 60 {
                return _text[..60].to_string();
            }
            return _text.to_string();
        }
        else {
            let _len = self.files.len();
            if _len > 1 {
                return "Attached files".to_string();
            }
            else if _len == 1 {
                let file = self.files[0].file.clone();
                if file.contains(".wav") {
                    return "Voice message".to_string();
                }
                else {
                    return "Attached file".to_string();
                }
            }
            return "".to_string();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatData { 
    pub id:                i32,
    pub title:             String,
    pub participants:      Vec<i32>,
    pub participants_data: Vec<UserChatCardData>,
    pub last_message:      crate::views::MessageData,
    pub is_favorite:       bool,
    pub is_archived:       bool,
    pub is_group:          bool,
    pub chat_type:         String,
    pub created:           String,
}
impl ChatData {
    pub fn get_chat_user(&self, user_id: &String) -> UserChatCardData {
        let participants = self.participants_data.clone();
        let mut cur_user = UserChatCardData { 
            id:             0,
            first_name:     "".to_string(),
            middle_name:    "".to_string(),
            last_name:      "".to_string(),
            email:          "".to_string(),
            phone:          "".to_string(),
            avatar:         None,
            address:        AddressData {
                                country:  "".to_string(),
                                state:    "".to_string(),
                                city:     "".to_string(),
                                address1: "".to_string(),
                                address2: "".to_string(),
                                zip_code: "".to_string(),
                            },
            user_type:      "".to_string(),
            opportunity_id: None, 
            lead_id:        None,
        };
        for i in participants.into_iter() {
            if &i.id.to_string() != user_id {
                cur_user = i;
            }
        }
        return cur_user;
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatssData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<ChatData>,
}

pub async fn chats_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<ChatData>;

        let ordering:  String;
        let chat_type: String;
        let chat_id:  i32;

        let params_some = web::Query::<ChatsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            ordering = get_string_with_string(params.ordering.clone());
            chat_type = get_string_with_string(params.chat_type.clone());
            chat_id = get_id_withi32(params.chat_id);
        } 
        else {
            ordering = String::new();
            chat_type = String::new();
            chat_id = 0;
        }

        let url = concat_string!(
            API.to_owned(),
            "social/chats/",
            "?ordering=", ordering
        );
        let resp = request_get::<crate::views::ChatssData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/chats.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<ChatData>,
            chat_type:    String,
            chat_id:      i32,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
            chat_type:    chat_type,
            chat_id:      chat_id,
        } 
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}


#[derive(Debug, Deserialize)]
pub struct MessagesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<MessageData>,
}

pub async fn messages_load(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() { 
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<MessageData>;

        let url = concat_string!(
            API.to_owned(),
            "social/chats/", _id.to_string(), "/messages/"
        );
        let resp = request_get::<crate::views::MessagesData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/messages.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<MessageData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}


#[derive(Debug, Deserialize)]
pub struct MatterMessagesParams {
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub search:   Option<String>,
    pub matter:   Option<i32>,
    pub ordering: Option<String>,
    pub seen:     Option<bool>,
} 

#[derive(Debug, Deserialize)]
pub struct LastCommentData { 
    pub id:                i32,
    pub post:              i32,
    pub author:            crate::utils::UserCardData,
    pub text:              String,
    pub participants:      Vec<i32>,
    pub participants_data: Vec<crate::utils::UserCardData>,
    pub seen:              bool,
    pub seen_by_client:    bool,
    pub title:             String,
    pub attachments:       Vec<i32>,
    pub attachments_data:  Vec<AttachmentsData>,
    pub created:           String,
    pub modified:          String,
} 

#[derive(Debug, Deserialize)]
pub struct MatterMessageData { 
    pub id:                i32,
    pub matter:            i32,
    pub last_comment:      LastCommentData,
    pub created:           String,
    pub modified:          String,
    pub comment_count:     i32,
    pub participants:      Vec<i32>,
    pub participants_data: Vec<crate::utils::UserCardData>,
    pub seen:              bool,
    pub seen_by_client:    bool,
    pub title:             String,
} 
#[derive(Debug, Deserialize)]
pub struct MatterMessagesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<MatterMessageData>,
}

pub async fn matter_messages_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<MatterMessageData>;

        let limit:  String;
        let offset: String;
        let search:  String;
        let ordering: String;
        let matter_id:  i32;
        let seen: String;

        let params_some = web::Query::<MatterMessagesParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            ordering = get_string_with_string(params.ordering.clone());
            matter_id = get_id_withi32(params.matter);
            seen = get_string_withbool(params.seen);
        } 
        else {
            limit = String::new();
            offset = String::new();
            search = String::new();
            ordering = String::new();
            matter_id = 0;
            seen = String::new();
        }
 
        let url = concat_string!(
            API.to_owned(),
            "business/matter-post/",
            "?ordering=", ordering,
            "&limit=", limit,
            "&offset=", offset,
            "&matter=", matter_id.to_string(),
            "&seen=", seen
        );
        let resp = request_get::<crate::views::MatterMessagesData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/matter_messages.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<MatterMessageData>,
            matter_id:    i32,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
            matter_id:    matter_id,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
} 

//////////////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct MatterCommentsParams {
    pub post:   Option<i32>,
    pub matter: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MatterCommentsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<LastCommentData>,
}

pub async fn matter_post_comments(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {  
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<LastCommentData>;

        let post_id:   i32;
        let matter_id: i32;

        let params_some = web::Query::<MatterCommentsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            matter_id = get_id_withi32(params.matter);
            post_id =   get_id_withi32(params.post);
        } 
        else {
            matter_id = 0;
            post_id =   0;
        }
 
        let url = concat_string!(
            API.to_owned(),
            "business/matter-comment/",
            "?post=", post_id.to_string()
        );
        let resp = request_get::<crate::views::MatterCommentsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/matter_comments.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<LastCommentData>,
            matter_id:    i32,
            post_id:      i32,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
            matter_id:    matter_id,
            post_id:      post_id,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}
/////////////////////////////////////////


#[derive(Debug, Deserialize)]
pub struct NotesParams {
    pub limit:      Option<i64>,
    pub offset:     Option<i64>,
    pub search:     Option<String>,
    pub matter:     Option<i32>,
    pub ordering:   Option<String>,
    pub created_by: Option<i32>,
    pub types:      Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NoteData {
    pub attachments:       Vec<i32>,
    pub attachments_data:  Vec<AttachmentsData>,
    pub client:            Option<i32>,
    pub client_data:       Option<crate::utils::UserSmallData>,
    pub created:           String,
    pub modified:          String,
    pub created_by:        Option<i32>,
    pub created_by_data:   Option<crate::utils::UserSmallData>,
    pub id:                i32,
    pub matter:            i32,
    pub matter_data:       MatterSmallData,
    pub text:              String,
    pub title:             String,
} 
#[derive(Debug, Deserialize)]
pub struct NotesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<NoteData>,
}
 
pub async fn notes_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<NoteData>;

        let limit:  String;
        let offset: String;
        let search:  String;
        let ordering: String;
        let matter_id:  i32;
        let created_by: String;
        let types: String;

        let params_some = web::Query::<NotesParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search.clone());
            ordering = get_string_with_string(params.ordering.clone());
            matter_id = get_id_withi32(params.matter);
            created_by = get_string_withi32(params.created_by);
            types = get_string_with_string(params.types.clone());
        } 
        else {
            limit = String::new();
            offset = String::new();
            search = String::new();
            ordering = String::new();
            matter_id = 0;
            created_by = String::new();
            types = String::new();
        }
 
        let url = concat_string!(
            API.to_owned(),
            "business/notes/",
            "?ordering=", ordering,
            "&limit=", limit,
            "&offset=", offset,
            "&matter=", matter_id.to_string(),
            "&created_by=", created_by
        );
        let resp = request_get::<crate::views::NotesData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }

        if types == "mini".to_string() {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/notes_mini.stpl")]
            pub struct Template {
                request_user: AuthResponseData,
                count:        i32,
                next:         Option<String>,
                page_count:   i32,
                object_list:  Vec<NoteData>,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                count:        count,
                next:         next,
                page_count:   page_count,
                object_list:  object_list,
                matter_id:    matter_id,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/notes.stpl")]
            pub struct Template {
                request_user: AuthResponseData,
                count:        i32,
                next:         Option<String>,
                page_count:   i32,
                object_list:  Vec<NoteData>,
                matter_id:    i32,
            }
            let body = Template {
                request_user: request_user,
                count:        count,
                next:         next,
                page_count:   page_count,
                object_list:  object_list,
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
//////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct PostsParams {
    pub author:        Option<i32>,
    pub limit:         Option<i64>,
    pub offset:        Option<i64>,
    pub ordering:      Option<String>,
    pub followed:      Option<bool>,
    pub comment_count: Option<i32>,
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
pub struct ProposalsData { 
    pub id:                     i32,
    pub name:                   String,
    pub created:                String,
    pub modified:               String,
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

pub async fn posts_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {  
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<PostData>;

        let ordering:      String; 
        let limit:         String;
        let offset:        String;
        let author:        String;
        let followed:      String;
        let comment_count: String;

        let params_some = web::Query::<PostsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =         get_limit(params.limit);
            offset =        get_string_withi64(params.offset);
            ordering =      get_string_with_string(params.ordering.clone());
            author =        get_string_withi32(params.author);
            followed =      get_string_withbool(params.followed);
            comment_count = get_string_withi32(params.comment_count);
        } 
        else {
            limit =         String::new();
            offset =        String::new();
            ordering =      String::new();
            author =        String::new();
            followed =      String::new();
            comment_count = String::new();
        }
 
        let url = concat_string!(
            API.to_owned(),
            "forum/posts/",
            "?ordering=", ordering,
            "&limit=", limit,
            "&offset=", offset,
            "&author=", author,
            "&followed=", followed,
            "&comment_count=", comment_count
        );
        let resp = request_get::<PostsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/posts.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<PostData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}
/////////////////////////////////////


#[derive(Debug, Deserialize)]
pub struct TopicsParams {
    pub limit:    Option<i64>,
    pub offset:   Option<i64>,
    pub ordering: Option<String>,
    pub followed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TopicsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<TopicData>,
}

pub async fn topics_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {  
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<TopicData>;

        let ordering: String; 
        let limit:    String;
        let offset:   String;
        let followed: String;

        let params_some = web::Query::<TopicsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            limit =    get_limit(params.limit);
            offset =   get_string_withi64(params.offset);
            ordering = get_string_with_string(params.ordering.clone());
            followed = get_string_withbool(params.followed);
        } 
        else {
            limit =    String::new();
            offset =   String::new();
            ordering = String::new();
            followed = String::new();
        }
        
        let url: String;
        if !followed.is_empty() {
            url = concat_string!(API.to_owned(), "forum/followed_topics/");
        }
        else {
            url = concat_string!(
                API.to_owned(),
                "forum/topics/",
                "?ordering=", ordering,
                "&limit=", limit,
                "&offset=", offset
            );
        }
        let resp = request_get::<TopicsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/topics.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<TopicData>,
            followed:     String,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
            followed:     followed,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}

#[derive(Debug, Deserialize)]
pub struct EventsParams {
    pub ordering:  Option<String>,
    pub attorney:  Option<i32>,
    pub client:    Option<i32>,
    pub paralegal: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EventData {  
    pub id:            i32, 
    pub attorney:      Option<i32>,
    pub paralegal:     Option<i32>, 
    pub title:         String,
    pub description:   Option<String>,
    pub is_all_day:    bool,
    pub start:         String,
    pub end:           String,
    pub duration:      String,
    pub location:      String,
    pub timezone:      i32,
    pub timezone_data: crate::utils::TimezoneData,
}

#[derive(Debug, Deserialize)]
pub struct EventsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<EventData>,
}

pub async fn events_load(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_some = get_request_user(&req);
    if user_some.is_some() {  
        let request_user = user_some.unwrap();
        let l = 2;

        let count:       i32;
        let next:        Option<String>;
        let page_count:  i32;
        let object_list: Vec<EventData>;

        let ordering:  String; 
        let attorney:  String;
        let client:    String;
        let paralegal: String;

        let params_some = web::Query::<EventsParams>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            attorney =   get_string_withi32(params.attorney);
            client =     get_string_withi32(params.client);
            paralegal =  get_string_withi32(params.paralegal);
            ordering =   get_string_with_string(params.ordering.clone());
        } 
        else {
            attorney =  String::new();
            client =    String::new();
            paralegal = String::new();
            ordering =  String::new();
        }
        
        let url = concat_string!(
            API.to_owned(),
            "promotion/events/",
            "?ordering=", ordering,
            "&attorney=", attorney,
            "&client=", client,
            "&paralegal=", paralegal
        );
        
        let resp = request_get::<EventsData> (
            url,
            &request_user.key
        ).await;
        if resp.is_ok() {
            let data = resp.expect("E.");
            count = data.count;
            next = data.next;
            page_count = data.page_count;
            object_list = data.results;
        }
        else {
            count = 0;
            next = None;
            page_count = 0;
            object_list = Vec::new();
        }
        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/events.stpl")]
        pub struct Template {
            request_user: AuthResponseData,
            count:        i32,
            next:         Option<String>,
            page_count:   i32,
            object_list:  Vec<EventData>,
        }
        let body = Template {
            request_user: request_user,
            count:        count,
            next:         next,
            page_count:   page_count,
            object_list:  object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        crate::views::login_page(req).await
    }
}

//////////////////////////////////////

//////////////  COUNTRIES  //////
//////////////  COUNTRIES //////

#[derive(Debug, Deserialize)]
pub struct CountryData {
    pub id:        i32,
    pub name:      String,
    pub code2:     String,
    pub phone:     String,
    pub has_state: bool,
}

#[derive(Debug, Deserialize)]
pub struct CountriesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<CountryData>,
}


//////////////  STATES  //////
//////////////  STATES //////

#[derive(Debug, Deserialize)]
pub struct StatesParams {
    pub limit:   Option<i64>,
    pub offset:  Option<i64>,
    pub country: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StatesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::StateData>,
}


//////////////  CITIES  //////
//////////////  CITIES //////

#[derive(Debug, Deserialize)]
pub struct CityParams {
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
    pub region: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CitiesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::CityData>,
}

//////////////  SPECIALITIES  //////
//////////////  SPECIALITIES //////

#[derive(Debug, Deserialize)]
pub struct SpecialityParams {
    pub ordering: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpecialitiesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::SpecialitiesData>,
}

//////////////  SEARCH ATTORNEYS AND PARALEGALS  //////
//////////////  SEARCH ATTORNEYS AND PARALEGALS //////

#[derive(Debug, Deserialize)]
pub struct SearchAtttorneysParams {
    pub offset:   Option<i64>,
    pub sharable: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchAttorneysData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::UserSmallData>,
}
//////////////////////////////////////////////


//////////////  STAGES  //////
//////////////  STAGES //////

#[derive(Debug, Deserialize)]
pub struct StagesParams {
    pub attorney: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StagesData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<crate::utils::StageData>,
}
//////////////////////////////////////////////