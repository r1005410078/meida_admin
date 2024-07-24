use actix_web::web;

use crate::handlers::residential::{
    create_residential, get_all_residential, get_residential, update_residential,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/residential")
            .service(get_residential)
            .service(get_all_residential)
            .service(create_residential)
            .service(update_residential),
    );
}
