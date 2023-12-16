use actix_web::{
  HttpRequest,
  web::block,
  HttpResponse,
  http::header::{Header, HeaderValue, TryIntoHeaderValue},
};
use std::{result::Result, env};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use crate::models::User;


#[derive(Debug, Deserialize)]
pub struct CountryData {
    pub id:    i32,
    pub name:  String,
    pub code2: String,
    pub phone: String,
}
#[derive(Debug, Deserialize)]
pub struct StateData {
    pub id:   i32,
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct StageData {
    pub id:    i32,
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct CityData {
    pub id:        i32,
    pub name:      String,
    pub latitude:  Option<String>,
    pub longitude: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TimezoneData {
    pub id:    i32,
    pub title: String,
}

#[derive(Debug, Deserialize)] 
pub struct AuthResponseData {  
    pub key:              String,
    pub user_type:        String,
    pub avatar:           Option<String>,
    pub user_id:          String,
    pub plan_id:          Option<String>,
    pub onboarding:       bool,
    pub phone:            String,
    pub timezone_data:    TimezoneData,
    pub owned_enterprise: Option<String>,
    pub enterprise:       Option<String>,
    pub role:             String,
    pub email:            String,
    pub first_name:       String,
    pub middle_name:      String,
    pub last_name:        String,
}
impl AuthResponseData {
    pub fn get_image(&self) -> String {
        if self.avatar.is_some() {
            return self.avatar.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/avatar.svg".to_string();
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct RequestClient {
    pub id:                i32,
    pub first_name:        String,
    pub middle_name:       String,
    pub last_name:         String,
    pub email:             String,
    pub phone:             String,
    pub r#type:            Option<String>,
    pub twofa:             bool,
    pub job:               Option<String>,
    pub avatar:            Option<String>,
    pub organization_name: Option<String>,
    pub note:              Option<String>,
    pub country:           i32,
    pub country_data:      CountryData,
    pub state:             i32,
    pub state_data:        Option<StateData>,
    pub city:              i32,
    pub city_data:         CityData,
    pub timezone:          i32,
    pub timezone_data:     TimezoneData,
    pub help_description:  Option<String>,
    pub specialities:      Vec<i32>,
    pub zip_code:          String,
    pub address1:          String,
    pub address2:          String,
    pub matters_count:     i32,
}



#[derive(Debug, Deserialize)]
pub struct FirmLocationData {
    pub id:           i32,
    pub country:      i32,
    pub country_data: CountryData,
    pub state:        i32,
    pub state_data:   Option<StateData>,
    pub city:         i32,
    pub city_data:    CityData,
    pub address:      String,
    pub zip_code:     String,
    pub latitude:     Option<String>,
    pub longitude:    Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EducationData {
    pub id:         i32,
    pub year:       i32,
    pub university: String,
}

#[derive(Debug, Deserialize)]
pub struct SpecialitiesData {
    pub id:         i32,
    pub title:      String,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FeeTypesData {
    pub id:    i32,
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct AppointmentTypeData {
    pub id:    i32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct PracticeAreaData {
    pub id:    i32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct PracticeJurisdictionsData {
    pub id:           i32,
    pub country:      i32,
    pub country_data: CountryData,
    pub state:        i32,
    pub state_data:   Option<StateData>,
    pub city:         i32,
    pub city_data:    CityData,
    pub agency:       Option<String>,
    pub number:       Option<String>,
    pub year:         Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentTypeData {
    pub id:    i32,
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct PaymentMethodData {
    pub id:    i32,
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct LanguageData {
    pub id:    i32,
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct FeeCurrencyData {
    pub id:    i32,
    pub title: String,
}

#[derive(Debug, Deserialize)] 
pub struct RequestAttorney {
    pub id:                       i32,
    pub distance:                 Option<String>,
    pub first_name:               String,
    pub middle_name:              String,
    pub last_name:                String,
    pub email:                    String,
    pub phone:                    String,
    pub r#type:                   Option<String>,
    pub avatar:                   Option<String>,
    pub biography:                Option<String>,
    pub firm_name:                Option<String>,
    pub website:                  Option<String>,
    pub firm_locations:           Vec<FirmLocationData>,
    pub is_verified:              bool,
    pub has_active_subscription:  bool,
    pub verification_status:      String, 
    pub featured:                 bool,
    pub sponsored:                bool,
    pub sponsor_link:             Option<String>,
    pub followers:                Vec<i32>,
    pub education:                Vec<EducationData>,
    pub practice_jurisdictions:   Vec<PracticeJurisdictionsData>,
    pub practice_description:     Option<String>,
    pub years_of_experience:      Option<i32>,
    pub have_speciality:          bool,
    pub specialities:             Vec<i32>,
    pub specialities_data:        Vec<SpecialitiesData>,
    pub speciality_time:          Option<String>,
    pub speciality_matters_count: Option<String>,
    pub fee_rate:                 String,
    pub fee_types:                Vec<i32>, 
    pub extra_info:               Option<String>,
    pub charity_organizations:    Option<String>,
    pub fee_types_data:           Vec<FeeTypesData>,
    pub keywords:                 Vec<String>,
    pub is_submittable_potential: bool,
    pub appointment_type:         Vec<i32>,
    pub payment_type:             Vec<i32>, 
    pub spoken_language:          Vec<i32>,
    pub appointment_type_data:    Vec<AppointmentTypeData>,
    pub payment_type_data:        Vec<PaymentTypeData>,
    pub spoken_language_data:     Vec<LanguageData>,
    pub fee_currency:             Vec<i32>,
    pub fee_currency_data:        Vec<FeeCurrencyData>,
    pub tax_rate:                 String,
    pub timezone:                 Vec<i32>,
    pub timezone_data:            Vec<TimezoneData>,
    pub enterprise:               Option<String>,
    pub owned_enterprise:         Option<String>,
    pub twofa:                    bool,
    pub subscribed:               bool,
    pub expiration_date:          Option<String>,
    pub enterprises_pending:      Vec<i32>,
}