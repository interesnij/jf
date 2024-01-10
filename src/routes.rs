use actix_web::web;

use crate::views::{
    auth,
    clients,
    attorneys,
    load,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(clients::clients_routes)
    .configure(attorneys::attorneys_routes)
    .configure(load::load_routes)
    ;
}
