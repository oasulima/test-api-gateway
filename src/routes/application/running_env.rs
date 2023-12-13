use actix_web::HttpResponse;

pub async fn get_running_env() -> HttpResponse {
    HttpResponse::Ok().body("Local")
}
