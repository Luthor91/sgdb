use std::sync::{Arc, Mutex};
use crate::engine::Engine; // Assurez-vous que le chemin d'importation est correct

pub struct AppConfig {
    pub engine: Arc<Mutex<Engine>>,
}

impl AppConfig {
    pub fn new() -> Self {
        let engine = Arc::new(Mutex::new(Engine::new())); // Assurez-vous que votre Engine a un constructeur approprié
        AppConfig { engine }
    }
}