#[cfg(test)]
pub mod tests_track_iterator {
    use web_radio::objects::track::track_iterator::TrackIterator;
    use web_radio::objects::track::track::Track;

    pub fn run_test() {
        test_track_iterator_initialization();
        test_track_iterator_go_next();
        test_track_iterator_has_more();
    }

    #[test]
    fn test_track_iterator_initialization() {
        let tracks = vec![
            Track {
                name: "Track 1".to_string(),
                file_format: "mp3".to_string(),
                file_path: "/path/to/track1.mp3".to_string(),
            },
            Track {
                name: "Track 2".to_string(),
                file_format: "mp3".to_string(),
                file_path: "/path/to/track2.mp3".to_string(),
            },
        ];

        let iterator = TrackIterator::new(tracks.clone(), 42);

        assert_ne!(iterator.get_current().name, "");
        assert_ne!(iterator.get_current().file_format, "");
        assert_ne!(iterator.get_current().file_path, "");
    }

    #[test]
    fn test_track_iterator_go_next() {
        let tracks = vec![
            Track {
                name: "Track 1".to_string(),
                file_format: "mp3".to_string(),
                file_path: "/path/to/track1.mp3".to_string(),
            },
            Track {
                name: "Track 2".to_string(),
                file_format: "mp3".to_string(),
                file_path: "/path/to/track2.mp3".to_string(),
            },
        ];

        let mut iterator = TrackIterator::new(tracks.clone(), 42);

        iterator.go_next();
        assert_ne!(iterator.get_current().name, "");
    }

    #[test]
    fn test_track_iterator_has_more() {
        let tracks = vec![
            Track {
                name: "Track 1".to_string(),
                file_format: "mp3".to_string(),
                file_path: "/path/to/track1.mp3".to_string(),
            },
        ];

        let mut iterator = TrackIterator::new(tracks.clone(), 42);

        assert!(!iterator.has_more()); // Apenas um item, não há mais
    }
}