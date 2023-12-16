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
    //config.route("/finance/profiles/current/", web::get().to(finance_current_page));
    //config.route("/finance/payment-method/", web::get().to(payment_method_page));
}


//////////////  profiles current  //////
//////////////  profiles current  //////

#[derive(Debug, Deserialize)]
pub struct ProductData { 
    pub id:          String,
    pub name:        String,
    pub description: String,
    pub r#type:      String,
    pub active:      bool,
    pub created:     String,
}

#[derive(Debug, Deserialize)]
pub struct PlanData { 
    pub id:                String,
    pub amount:            String,
    pub currency:          String,
    pub interval:          String,
    pub nickname:          String,
    pub description:       String,
    pub product:           i32,
    pub product_data:      ProductData,
    pub trial_period_days: Option<String>,
    pub is_premium:        bool,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionData {  
    pub id:                    String,
    pub billing:               String,
    pub status:                String,
    pub billing_cycle_anchor:  String,
    pub canceled_at:           Option<String>,
    pub cancel_date:           Option<String>,
    pub renewal_date:          String,
    pub cancel_at_period_end:  bool,
    pub current_period_end:    String,
    pub current_period_start:  String,
    pub start:                 String,
    pub plan_data:             PlanData,
    pub plan:                  i32,
    pub customer:              i32,
    pub trial_end:             Option<String>,
    pub trial_start:           Option<String>,
    pub metadata:              Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentMethodChecksData { 
    pub address_line1_check:       String,
    pub address_postal_code_check: String,
    pub cvc_check:                 String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentMethodNetworksData { 
    pub available: Vec<String>,
    pub preferred: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentMethodSecureData { 
    pub supported: bool,
}

#[derive(Debug, Deserialize)]
pub struct PaymentMethodData { 
    pub brand:                String,
    pub checks:               PaymentMethodChecksData,
    pub country:              String,
    pub exp_month:            i32,
    pub exp_year:             i32,
    pub fingerprint:          String,
    pub funding:              i32,
    pub generated_from:       Option<String>,
    pub last4:                String,
    pub networks:             PaymentMethodNetworksData,
    pub three_d_secure_usage: PaymentMethodSecureData,
    pub wallet:               Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BillingAddressData { 
    pub city:        String,
    pub country:     String,
    pub line1:       String,
    pub line2:       String,
    pub postal_code: String,
    pub state:       String,
}

#[derive(Debug, Deserialize)]
pub struct BillingDetailData { 
    pub address: BillingAddressData,
    pub email:   Option<String>,
    pub name:    String,
    pub phone:   Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileCurrentInvoice { 
    pub date:   String,
    pub amount: f64,
    pub link:   String,
}

#[derive(Debug, Deserialize)]
pub struct BillingItemData { 
    pub current_invoice: ProfileCurrentInvoice,
    pub next_invoice:    Option<ProfileCurrentInvoice>,
}

#[derive(Debug, Deserialize)]
pub struct FinanceProfileCurrentData { 
    pub id:                     String,
    pub subscription_data:      SubscriptionData,
    pub next_subscription_data: Option<SubscriptionData>,
    pub payment_method_data:    PaymentMethodData,
    pub billing_detail_data:    BillingDetailData,
    pub name:                   Option<String>,
    pub billing_item:           BillingItemData,
}


//////////////  payment method  //////
//////////////  payment method  //////

#[derive(Debug, Deserialize)]
pub struct FinanceProfileCurrentData { 
    pub djstripe_id:      i32,
    pub default:          bool,
    pub card:             PaymentMethodData,
    pub billing_details:  BillingDetailData,
    pub id:               String,
    pub livemode:         bool,
    pub created:          String,
    pub metadata:         Option<String>,
    pub description:      String,
    pub djstripe_created: String,
    pub djstripe_updated: String,
    pub card_present:     String,
    pub r#type:           String,
    pub customer:         i32,
}

#[derive(Debug, Deserialize)]
pub struct PaymentttMethodsData { 
    pub count:      i32,
    pub next:       Option<String>,
    pub page_count: i32,
    pub previous:   Option<String>,
    pub results:    Vec<PaymentttMethod>,
}