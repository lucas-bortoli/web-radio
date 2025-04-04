use std::collections::HashMap;

use rand::SeedableRng;
use rocket::time::Date;
use super::{station::station::Station};

struct Radio {
    pub stations: HashMap<String, Station>,
    pub seed: u64,
    pub connections: i64,
    pub _frequency: String
}

impl Radio {

    fn new(seed: u64, _frequency: String) -> Self {
        Self {
            stations: HashMap::new(),
            seed,
            connections: 0,
            _frequency,
        }
    }

    fn determine_station() {
        // recebe o endpoint aqui e aponta para o spawn correto
    }

    fn station_controller() {
        // controla as threds das stations
    }    

    fn spawn_station(frequency: &str) {
        // spawna a thread referente a stação da frequencia
    }

    fn set_frequency() {
        // pega o endpoint e trata
    }

}