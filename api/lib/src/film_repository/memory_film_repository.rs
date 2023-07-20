use std::{collections::HashMap, sync::RwLock};

use shared::models::{CreateFilm, Film};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

use super::{FilmRepository, FilmResult};

pub struct MemoryFilmRepository {
    films: RwLock<HashMap<Uuid, Film>>,
}

impl MemoryFilmRepository {
    pub fn new() -> Self {
        Self {
            films: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryFilmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl FilmRepository for MemoryFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        let result = self
            .films
            .read()
            .map(|films| films.clone().into_values().collect::<Vec<_>>())
            .map_err(|e| format!("An error occured while trying to read films: {}", e));

        if result.is_err() {
            tracing::error!("Could not retrive films");
        }

        result
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        let result = self
            .films
            .read()
            .map_err(|e| format!("An error occured while trying to read films: {}", e))
            .and_then(|films| {
                films
                    .get(film_id)
                    .cloned()
                    .ok_or_else(|| format!("Couln't fild film: {}", film_id))
            });

        if result.is_err() {
            tracing::error!("Could not retrive film with id {}", film_id);
        }

        result
    }

    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let new_film = Film {
                    id: uuid::Uuid::new_v4(),
                    title: create_film.title.clone(),
                    director: create_film.director.clone(),
                    year: create_film.year,
                    poster: create_film.poster.clone(),
                    created_at: Some(Utc::now()),
                    updated_at: None,
                };
                films.insert(new_film.id, new_film.clone());
                tracing::trace!("Film with id {} correctly created", new_film.id);
                Ok(new_film)
            }
            Err(e) => {
                let err = format!("An error occured while trying to update film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }
}
