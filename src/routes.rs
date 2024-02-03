use actix_web::web;

use crate::views::{
    auth,
    clients,
    attorneys,
    attorney_load,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(clients::clients_routes)
    .configure(attorneys::attorneys_routes)
    .configure(attorney_load::load_routes)
    ; 
}
