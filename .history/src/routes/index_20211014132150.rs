use actix_web::HttpResponse;

pub struct IndexResponse {
    uptime: u64
}

pub async fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .json(value))
}