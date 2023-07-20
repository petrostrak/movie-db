use std::{collections::HashMap, sync::RwLock};

use shared::models::Film;
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
}
