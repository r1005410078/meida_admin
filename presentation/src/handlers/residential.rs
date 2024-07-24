use actix_web::{get, post, web, HttpResponse};
use application::use_cases::{
    get_residential::GetResidentialUseCase, save_residential::SaveResidentialUseCase,
};

use domain::residential::entities::residential::{NewResidential, UpdateResidential};
use infrastructure::repositories::mysql_residential_repository::MysqlResidentialRepository;
use log::error;

#[get("/query_name/{name}")]
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

#[get("/list")]
async fn get_all_residential(
    residential_repo: web::Data<MysqlResidentialRepository>,
) -> HttpResponse {
    let list = GetResidentialUseCase::new(residential_repo.into_inner().as_ref())
        .get_list()
        .await;

    HttpResponse::Ok().json(list)
}

#[post("/create")]
async fn create_residential(
    residential_repo: web::Data<MysqlResidentialRepository>,
    data: web::Json<NewResidential>,
) -> HttpResponse {
    match SaveResidentialUseCase::new(residential_repo.into_inner().as_ref())
        .execute_create(&data)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Residential created"),
        Err(_) => HttpResponse::InternalServerError().body("Please try again later."),
    }
}

#[post("/update")]
async fn update_residential(
    residential_repo: web::Data<MysqlResidentialRepository>,
    data: web::Json<UpdateResidential>,
) -> HttpResponse {
    match SaveResidentialUseCase::new(residential_repo.into_inner().as_ref())
        .execute_update(&data)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Residential updated"),
        Err(_) => HttpResponse::InternalServerError().body("Please try again later."),
    }
}
