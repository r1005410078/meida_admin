use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::repositories::{
    mysql_house_repository::MySqlHouseRepository,
    mysql_residential_repository::MysqlResidentialRepository,
};
use log::info;

use crate::routes;

pub async fn run() -> std::io::Result<()> {
    let residential = web::Data::new(MysqlResidentialRepository::new());
    let house_data = web::Data::new(MySqlHouseRepository::new());

    info!("Web server Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(residential.clone())
            .app_data(house_data.clone())
            .wrap(Logger::default())
            .configure(routes::residential_routes::routes)
            .configure(routes::house::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
