use rocket::{
    http::ContentType,
    response::{
        content::{RawCss, RawHtml},
        stream::ByteStream,
    },
    tokio::sync::broadcast,
};

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    ops::DerefMut,
    sync::Arc,
    thread,
    time::Duration,
};

pub mod input_decoder;

#[macro_use]
extern crate rocket;

// #[get("/")]
// fn index() -> RawHtml<&'static [u8]> {
//     return RawHtml(include_bytes!("ui.html"));
// }

// #[get("/ui.css")]
// fn stylesheet() -> RawCss<&'static [u8]> {
//     return RawCss(include_bytes!("ui.css"));
// }

// struct AudioBroadcaster {
//     sender: broadcast::Sender<Box<[u8; POLL_BUFFER_SIZE_BYTES]>>,
// }

#[launch]
fn rocket() -> _ {
    //let (tx, _) = broadcast::channel::<Box<[u8; POLL_BUFFER_SIZE_BYTES]>>(8);

    //let broadcaster = Arc::new(AudioBroadcaster { sender: tx.clone() });

    // let station = Station::from_file("./diamond_city_radio/radio.yaml")
    //     .expect("Failed to parse station file");
    // let station_thread_clone = station.clone();

    rocket::build()
    //.manage(broadcaster)
    //.mount("/", routes![index, stylesheet])
    //.mount("/station", routes![station_diamondcity])
}
