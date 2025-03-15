struct StationSnapshot {
    name: String,
    current_track: Track,
    subscribers: Vec<Subscriber>,
    created_on: Date,
}
