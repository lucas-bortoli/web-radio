use rocket::time::Date;

use crate::objects::{subscriber::Subscriber, track::track::Track};

struct StationSnapshot {
    name: String,
    current_track: Track,
    subscribers: Vec<Subscriber>,
    created_on: Date,
}
