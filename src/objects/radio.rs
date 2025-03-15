use std::collections::HashMap;

use rocket::time::Date;
use super::{station::station::Station};

struct Radio {
    stations: HashMap<String, Station>,
    seed: u64,
    connections: i64
}


