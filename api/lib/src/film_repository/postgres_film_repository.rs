use shared::models::Film;

use super::{FilmRepository, FilmResult};

pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}

impl PostgresFilmRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as::<_, Film>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM films
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM films
            WHERE id = $1
        "#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
