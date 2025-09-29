// Basic repository pattern implementation
use aion_core::{AionResult};

pub trait Repository<T, K> {
    fn save(&self, entity: T) -> AionResult<K>;
    fn find_by_id(&self, id: K) -> AionResult<Option<T>>;
    fn find_all(&self) -> AionResult<Vec<T>>;
    fn delete(&self, id: K) -> AionResult<()>;
}

pub struct BaseRepository;

impl BaseRepository {
    pub fn new() -> Self {
        Self
    }
}