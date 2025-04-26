use std::{collections::HashMap, path::PathBuf};

use bytes::Bytes;
use cytoplasm::cytoplasm::Cytoplasm;
use output_encoder::audio_encoder::OutputCodec;
use rocket::{
    http::ContentType,
    response::{content::RawHtml, stream::ByteStream},
};

pub mod audio_file_info;
pub mod cytoplasm;
pub mod input_decoder;
pub mod output_encoder;
pub mod output_stream;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<&'static [u8]> {
    return RawHtml(b"<!DOCTYPE html>\n<audio controls src='/station'>");
}

type StationMap = HashMap<String, Cytoplasm>;

#[get("/station")]
fn station_endpoint(state: &rocket::State<StationMap>) -> (ContentType, ByteStream![Bytes]) {
    let station = state.get("diamondcityradio").unwrap();
    let stream = station
        .output_streams
        .get(&OutputCodec::Mp3_64kbps)
        .unwrap();

    stream.create_consumer_http_stream()
}

#[launch]
fn rocket() -> _ {
    let mut stations: StationMap = HashMap::new();
    stations.insert(
        "diamondcityradio".to_string(),
        Cytoplasm::new(
            PathBuf::from("./DiamondCityRadio"),
            &[OutputCodec::Mp3_64kbps],
        ),
    );

    rocket::build()
        .manage(stations)
        .mount("/", routes![index, station_endpoint])
}
