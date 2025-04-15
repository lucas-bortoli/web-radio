use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use input_decoder::input_audio_file;
use output_encoder::{ConsumerPacket, OutputCodec};
use rocket::tokio::sync::{self};

pub mod input_decoder;
pub mod output_encoder;

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
//
//#[launch]
//fn rocket() -> _ {
//    //let (tx, _) = broadcast::channel::<Box<[u8; POLL_BUFFER_SIZE_BYTES]>>(8);
//
//    //let broadcaster = Arc::new(AudioBroadcaster { sender: tx.clone() });
//
//    // let station = Station::from_file("./diamond_city_radio/radio.yaml")
//    //     .expect("Failed to parse station file");
//    // let station_thread_clone = station.clone();
//
//    //rocket::build()
//    //.manage(broadcaster)
//    //.mount("/", routes![index, stylesheet])
//    //.mount("/station", routes![station_diamondcity])
//
//    return (());
//}

fn main() {
    let (rx, tx) = sync::broadcast::channel::<ConsumerPacket>(8);
    let station_subscribers = Arc::new(RwLock::new(vec![rx.clone()]));
    let mut output =
        output_encoder::AudioEncoder::new(OutputCodec::Opus128kbps, station_subscribers.clone());

    loop {
        let input = input_audio_file::open_input_file_strategy(
            "./Blank Banshee - Gunshots.mp3".to_string(),
        );

        for packet in input {
            println!("Áudio: {:.02}s", packet.audio_length);
            //println!("{:?}", packet.buffer);
            output.push_audio_packet(&packet);

            thread::sleep(Duration::from_nanos(
                (packet.audio_length * 1000_000_000.0) as u64,
            ));
        }
    }
}
