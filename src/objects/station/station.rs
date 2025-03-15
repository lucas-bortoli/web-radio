use crate::objects::{subscriber::Subscriber, track::track::Track};

pub struct Station {
    name: String,
    current_track: Track,
    subscribers: Vec<Subscriber>,
}