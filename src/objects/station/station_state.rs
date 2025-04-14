// estrutura de dados para armazenar o estado da estação
pub trait StationState {

}

pub struct MockStationState {
    pub teste: i64,
}

impl MockStationState {
    pub fn new() -> MockStationState {
        MockStationState { teste: 1 }
    }
}

impl StationState for MockStationState {}