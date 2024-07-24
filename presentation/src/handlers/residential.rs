use actix_web::{get, web, HttpResponse};
use application::use_cases::get_residential::GetResidentialUseCase;

use infrastructure::repositories::mysql_residential_repository::MysqlResidentialRepository;
use log::error;

#[get("/{name}")]
async fn get_residential(
    residential_repo: web::Data<MysqlResidentialRepository>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    match GetResidentialUseCase::new(residential_repo.into_inner().as_ref())
        .get(&path.0)
        .await
    {
        Some(residential) => HttpResponse::Ok().json(residential),
        None => {
            error!("Error registering residential: {}", path.0);
            HttpResponse::InternalServerError().body("Please try again later.")
        }
    }
}
