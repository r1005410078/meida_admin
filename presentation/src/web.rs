use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::repositories::mysql_residential_repository::MysqlResidentialRepository;
use log::info;

use crate::routes;

pub async fn run() -> std::io::Result<()> {
    let repo = MysqlResidentialRepository::new();
    let app_data = web::Data::new(repo);

    info!("Web server Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routes::residential_routes::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
