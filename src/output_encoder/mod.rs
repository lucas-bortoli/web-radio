use std::{
    io::{BufReader, BufWriter, Read, Write},
    process::{ChildStdin, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use rocket::tokio::sync::broadcast;

use crate::input_decoder::input_audio_file::AudioPacket;

pub const INPUT_CHANNEL_COUNT: u32 = 2;
pub const INPUT_SAMPLE_RATE: u32 = 44100;
pub const INPUT_BYTE_DEPTH: u32 = 2; //16bits

pub const OUTPUT_CHANNEL_COUNT: u32 = 2;
pub const OUTPUT_SAMPLE_RATE: u32 = 44100;
pub const OUTPUT_BYTE_DEPTH: u32 = 2; //16bits

pub const MAX_STATION_LISTENERS: usize = 64; // quantos players podem ouvir essa estação de uma vez?

pub enum OutputCodec {
    Mp3,
    OggVorbis,
    Opus,
}

type ConsumerPacket = Box<Vec<u8>>;
type Consumer = broadcast::WeakSender<ConsumerPacket>;
type ProtectedConsumerVec = Arc<Mutex<Vec<Consumer>>>;

// singleton - um por estação
pub struct AudioEncoder {
    encoder_in: BufWriter<ChildStdin>,
    consumers: ProtectedConsumerVec,
}

impl AudioEncoder {
    pub fn new(output_codec: OutputCodec, consumers: ProtectedConsumerVec) -> AudioEncoder {
        let mut child = Command::new("ffmpeg")
            .args(&[
                "-f",
                "s16le",
                "-ar",
                &INPUT_SAMPLE_RATE.to_string(),
                "-ac",
                &INPUT_CHANNEL_COUNT.to_string(),
                "-i",
                "-", // stdin como input pro ffmpeg
                "-f",
                match output_codec {
                    OutputCodec::Mp3 => "mp3",
                    OutputCodec::OggVorbis => "ogg",
                    OutputCodec::Opus => "opus",
                },
                "-", // stdout como output pro ffmpeg
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("encoder: Falha ao spawnar o ffmpeg");

        let stdin = child.stdin.take().expect("encoder: Falha ao ler stdin");
        let stdin_writer = BufWriter::new(stdin);
        let stdout = child.stdout.take().expect("encoder: Falha ao ler stdout");
        let mut stdout_reader = BufReader::new(stdout);

        let encoder_consumers = consumers.clone();
        thread::spawn(move || {
            println!("encoder: Thread de consumidor de áudio iniciada.");

            let mut buf = Vec::<u8>::new();
            loop {
                let n = stdout_reader
                    .read(buf.as_mut())
                    .expect("encoder: Ler stdout do encoder falhou - processo crashou?");

                match n {
                    0 => panic!("encoder: Stdout finalizou, estação acabou!"),
                    1.. => {
                        println!("encoder: {} bytes retornados do encoder!", n);

                        let potential_consumers = encoder_consumers.lock().unwrap().to_vec();
                        for potential_consumer in potential_consumers {
                            if let Some(consumer) = potential_consumer.upgrade() {
                                if let Err(e) = consumer.send(Box::new(buf[..n].to_vec())) {
                                    eprintln!("encoder: Falha ao enviar para consumer: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        });

        AudioEncoder {
            encoder_in: stdin_writer,
            consumers,
        }
    }

    pub fn push_audio_packet(&mut self, packet: AudioPacket) {
        self.encoder_in
            .write(&packet.buffer)
            .expect("encoder: A fila do ffmpeg está cheia?");

        // bypass do buffer do stdin; manda direto pro ffmpeg, já que áudio é em real-time e talvez não seja legal ter esse comportamento de buffering
        // ignoramos o Result propositalmente, não há nenhuma ação cabível a ser tomada se o buffer de stdin não pode ser flushado - meio que não importa
        let _ = self.encoder_in.flush();
    }
}
