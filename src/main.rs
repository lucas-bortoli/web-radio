use std::collections::HashMap;

use cytoplasm::cytoplasm::Cytoplasm;
use output_encoder::audio_encoder::OutputCodec;
use rocket::{
    http::ContentType,
    response::{content::RawHtml, stream::ByteStream},
};

pub mod cytoplasm;
pub mod input_decoder;
pub mod output_encoder;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<&'static [u8]> {
    return RawHtml(b"<!DOCTYPE html>\n<audio controls src='/station'>");
}

type StationMap = HashMap<String, Cytoplasm>;

#[get("/station")]
fn station_endpoint(state: &rocket::State<StationMap>) -> (ContentType, ByteStream![Vec<u8>]) {
    let station = state.get("flintnsteel").unwrap();

    let stream = station.create_output_stream(&OutputCodec::Mp3_64kbps);

    stream.unwrap()
}

#[launch]
fn rocket() -> _ {
    let mut stations: StationMap = HashMap::new();
    stations.insert(
        "flintnsteel".to_string(),
        Cytoplasm::new(&[OutputCodec::Mp3_64kbps]),
    );

    rocket::build()
        .manage(stations)
        .mount("/", routes![index, station_endpoint])
}
