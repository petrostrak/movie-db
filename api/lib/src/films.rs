use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all::<R>))
            .route("/{film_id}", web::get().to(get::<R>))
            .route("", web::post().to(create::<R>))
            .route("", web::put().to(update::<R>))
            .route("/{film_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: FilmRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get<R: FilmRepository>(repo: web::Data<R>, film_id: web::Path<Uuid>) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn create<R: FilmRepository>(
    repo: web::Data<R>,
    create_film: web::Json<CreateFilm>,
) -> HttpResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn update<R: FilmRepository>(
    repo: web::Data<R>,
    updated_film: web::Json<Film>,
) -> HttpResponse {
    match repo.update_film(&updated_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: FilmRepository>(repo: web::Data<R>, film_id: web::Path<Uuid>) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}
