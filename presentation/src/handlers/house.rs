use actix_web::{get, post, web, HttpResponse};
use application::use_cases::{get_house::GetHouseUseCase, save_house::SaveHouseUseCase};
use domain::house::entities::house::{NewHouse, UpdateHouse};
use infrastructure::repositories::mysql_house_repository::MySqlHouseRepository;

#[get("/list")]
async fn get_all_house(house_repo: web::Data<MySqlHouseRepository>) -> HttpResponse {
    let list = GetHouseUseCase::new(house_repo.into_inner().as_ref())
        .get()
        .await;

    HttpResponse::Ok().json(list)
}

#[post("/create")]
async fn create_house(
    house_repo: web::Data<MySqlHouseRepository>,
    data: web::Json<NewHouse>,
) -> HttpResponse {
    match SaveHouseUseCase::new(house_repo.into_inner().as_ref())
        .execute_create(&data)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("House created"),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/update")]
async fn update_house(
    house_repo: web::Data<MySqlHouseRepository>,
    data: web::Json<UpdateHouse>,
) -> HttpResponse {
    match SaveHouseUseCase::new(house_repo.into_inner().as_ref())
        .execute_update(&data)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("House updated"),
        Err(_) => HttpResponse::InternalServerError().body("Please try again later."),
    }
}
