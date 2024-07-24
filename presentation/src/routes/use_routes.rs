use actix_web::web;

use crate::handlers::residential::get_residential;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/user")
            .service(get_residential)
            .service(get_by_email),
    );
}
