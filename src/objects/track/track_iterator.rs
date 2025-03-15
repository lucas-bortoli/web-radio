// estrutura para armazenamento de estado anterior da estação

use super::track::Track;

struct TrackIterator {
    track: Track,
    next_track: Track,
    current_index: usize,
}