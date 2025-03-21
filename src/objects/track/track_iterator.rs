// estrutura para armazenamento de estado anterior da estação

use super::track::Track;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

struct TrackIterator {
    track: Track,
    track_queue: Vec<Track>,
    current_index: usize,
    seed: u64,
}

impl TrackIterator {
    fn new(track: Track, track_queue: Vec<Track>, seed: u64) -> Self {
        TrackIterator {
            track,
            track_queue,
            current_index: 0,
            seed
        }
    }

    fn has_more(&self) -> bool {
        self.track_queue.len() <= 1
    }

    fn go_next(&mut self) {
        let nx_track = self.get_next();
        self.track = nx_track;
    }

    fn get_next(&mut self) -> Track {
        self.shuffle();
        let next_index = self.pick_next();

        let nx_track= self.track_queue[next_index];
        self.track_queue.remove(next_index);
        nx_track
    }

    fn shuffle(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        rng.shuffle(&mut self.track_queue);
    }

    fn pick_next(&mut self) -> usize {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let next_index = rng.gen_range(0..self.track_queue.len());
        self.current_index = next_index;
        next_index
    }

    fn get_current(&self) -> Track {
        self.track
    }
}