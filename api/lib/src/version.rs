use actix_web::web::{self, ServiceConfig};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/version", web::get().to(version));
}

#[tracing::instrument]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version..");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    result.unwrap_or(String::from("error: unknown"))
}
