use actix_web::web;

use crate::views::{
    auth,
    clients,
    attorneys,
    attorney_load,
    create,
    pages,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(clients::clients_routes)
    .configure(attorneys::attorneys_routes)
    .configure(attorney_load::load_routes)
    .configure(create::create_routes)
    .configure(pages::pages_routes)
    ; 
}
