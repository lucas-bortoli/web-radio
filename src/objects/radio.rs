use std::collections::HashMap;

use super::{station::station::Station};

struct Radio {
    stations: HashMap<String, Station>,
    seed: u64,
    connections: i64,
    _frequency: String
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