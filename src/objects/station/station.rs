use std::path;

use crate::objects::{subscriber::Subscriber, station::station_state::StationState};

pub struct Station {
    pub name: String,
    pub _subscribers: Vec<Subscriber>,
    pub path: String,
    pub frequency: f32,
    pub _state: Box<dyn StationState>,
}


impl Station {
    pub fn new(name: String, path: String, frequency: f32, _state: Box<dyn StationState>) -> Station {
        Station {
            name,
            _subscribers: Vec::new(),
            path,
            frequency,
            _state,
        }
    }

    pub fn add_subscriber(&mut self, subscriber: Subscriber) {
        self._subscribers.push(subscriber);
    }

    pub fn remove_subscriber(&mut self, subscriber: &Subscriber) {
        self._subscribers.retain(|s| s != subscriber);
    }

    pub fn change_state(&mut self, state: Box<dyn StationState>) {
        self._state = state;
    }

    pub fn get_music_vec(&self) -> Vec<String> {
        let mut music_vec = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.path().to_str() {
                            music_vec.push(file_name.to_string());
                        }
                    }
                }
            }
        }
        
        music_vec
    }

    // Funções que acredito que vão estar no state
    pub fn go_next(&self) {
        // self._state.go_next();
    }

    pub fn play(&self) {
        // self._state.play();
    }


}