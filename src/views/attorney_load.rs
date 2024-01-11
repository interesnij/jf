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
    get_string_with_string, get_limit, get_string_withi64, get_string_withi32, 
    get_request_user, AuthResponseData, request_get, API
};


pub fn load_routes(config: &mut web::ServiceConfig) {
    config.route("/attorney_load/leads_and_clients/", web::get().to(leads_and_clients_load));
    config.route("/attorney_load/matters/", web::get().to(attorney_matters_load));
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
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LeadOrClientData {  
    pub id:            i32,
    pub first_name:    String,
    pub middle_name:   String,
    pub last_name:     String,
    pub phone:         String,
    pub avatar:        Option<String>,
    pub job:           String,
    pub company:       Option<String>,
    pub country_data:  crate::utils::CountryData,
    pub state_data:    Option<crate::utils::StateData>,
    pub city_data:     crate::utils::CityData,
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
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
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
            _type =  get_string_with_string(params.r#type.clone());
        }
        else {
            limit =  String::new();
            offset = String::new();
            search = String::new();
            _type =  String::new();
        }
        let resp = request_get::<crate::views::LeadsAndClientsData> (
            API.to_owned()
            + &"users/attorneys/".to_string() + &request_user.user_id + &"/leads_and_clients/".to_string()
            + &"&search=" + &search
            + &"&offset=" + &offset
            + &"&limit=" + &limit
            + &"&type=" + &_type,
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
    pub limit:       Option<i64>,
    pub offset:      Option<i64>,
    pub search:      Option<String>,
    pub attorney:    Option<i32>,
    pub status:      Option<String>,
    pub shared_with: Option<i32>,
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
    pub currency:         Vec<i32>,
    pub currency_data:    Vec<crate::utils::FeeCurrencyData>,
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
    pub envelope_data:         Vec<i32>,
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

        let params_some = web::Query::<AttorneyMattersData>::from_query(&req.query_string());
        if params_some.is_ok() {
            let params = params_some.unwrap();
            ordering = get_string_with_string(params.ordering);
            limit =  get_limit(params.limit);
            offset = get_string_withi64(params.offset);
            search = get_string_with_string(params.search);
            attorney = get_string_withi32(params.attorney);
            status = get_string_with_string(params.status);
            shared_with = get_string_with_string(params.shared_with);
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
        let resp = request_get::<crate::views::LeadsAndClientsData> (
            API.to_owned()
            + &"business/matters/?ordering=".to_string() + &ordering
            + &"&search=" + &search
            + &"&offset=" + &offset
            + &"&limit=" + &limit
            + &"&attorney=" + &attorney
            + &"&status=" + &status
            + &"&shared_with=" + &shared_with,
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