use actix_web::web;

use crate::views::{
    auth,
    clients,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(clients::clients_routes)
    ;
}
