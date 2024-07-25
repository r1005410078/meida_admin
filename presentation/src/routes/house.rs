use actix_web::web;

use crate::handlers::house::{create_house, get_all_house, update_house};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/house")
            .service(get_all_house)
            .service(create_house)
            .service(update_house),
    );
}
