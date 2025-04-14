#[cfg(test)]
pub mod tests_track_iterator {
    use web_radio::objects::station::station::Station;
    use web_radio::objects::station::station_state::MockStationState;
    use web_radio::objects::track::track::{Narration, Track};
    use web_radio::objects::track::track_iterator::TrackIterator;

    pub fn run_test() {
        test_track_iterator_initialization();
        //test_track_iterator_go_next();
        test_track_iterator_has_more();
    }

    #[test]
    fn test_track_iterator_initialization() {
        let station = get_station_test();

        let iterator = TrackIterator::new(station.tracks.clone(), 42);

        assert_ne!(iterator.get_current().title, "");
        assert_ne!(iterator.get_current().file_format, "");
        assert_ne!(iterator.get_current().source, "");
    }

    // #[test]
    // fn test_track_iterator_go_next() {
    //     let tracks = vec![
    //         mock_track()
    //     ];

    //     let mut iterator = TrackIterator::new(tracks.clone(), 42);

    //     iterator.go_next();
    //     assert_ne!(iterator.get_current().title, "");
    // }

    #[test]
    fn test_track_iterator_has_more() {
        let tracks = vec![mock_track()];

        let iterator = TrackIterator::new(tracks.clone(), 42);

        assert!(!iterator.has_more()); // Apenas um item, não há mais
    }

    fn get_station_test() -> Station {
        let station_state = MockStationState::new(); // Assuming StationState has a `new` method
        let station = Station::new(
            "Diamond City Radio".to_owned(),
            "./diamond_city_radio/".to_owned(),
            98.9,
            Box::new(station_state),
        );
        station
    }

    fn mock_track() -> Track {
        Track::new(
            "Mocked Title".to_string(),
            "Mocked Artist".to_string(),
            "Mocked Album".to_string(),
            300, // duração em segundos (5 minutos)
            "mp3".to_string(),
            "mocked_source.mp3".to_string(),
            vec![Narration {
                title: "Mocked After Narration".to_string(),
                duration: 30, // duração em segundos
                file_format: "mp3".to_string(),
                source: "mocked_after_narration.mp3".to_string(),
            }],
            vec![Narration {
                title: "Mocked Before Narration".to_string(),
                duration: 20, // duração em segundos
                file_format: "mp3".to_string(),
                source: "mocked_before_narration.mp3".to_string(),
            }],
        )
    }
}
