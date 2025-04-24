use bytes::Bytes;
use std::{
    io::{BufReader, BufWriter, Read, Write},
    process::{ChildStdin, Command, Stdio},
    sync::Arc,
    thread,
};

use crate::{input_decoder::input_audio_file::AudioPacket, output_stream::OutputStream};

pub const INPUT_CHANNEL_COUNT: u32 = 2;
pub const INPUT_SAMPLE_RATE: u32 = 44100;
pub const INPUT_BYTE_DEPTH: u32 = 2; //16bits

pub const OUTPUT_CHANNEL_COUNT: u32 = 2;
pub const OUTPUT_SAMPLE_RATE: u32 = 44100;
pub const OUTPUT_BYTE_DEPTH: u32 = 2; //16bits

pub const MAX_STATION_LISTENERS: usize = 64; // quantos players podem ouvir essa estação de uma vez?

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum OutputCodec {
    Mp3_64kbps,
    Ogg96kbps,
    Opus128kbps,
}

fn ffmpeg_args(output_codec: &OutputCodec) -> Vec<String> {
    let sample_rate = INPUT_SAMPLE_RATE.to_string();
    let channel_count = INPUT_CHANNEL_COUNT.to_string();

    let mut args = vec![
        "-f",
        "s16le",
        "-ar",
        &sample_rate,
        "-ac",
        &channel_count,
        "-i",
        "-", // stdin como input pro ffmpeg
    ];

    args.append(&mut match output_codec {
        OutputCodec::Mp3_64kbps => vec!["-b:a", "64k", "-f", "mp3"],
        OutputCodec::Ogg96kbps => vec!["-b:a", "96k", "-f", "ogg"],
        OutputCodec::Opus128kbps => vec!["-c:a", "libopus", "-b:a", "128k", "-f", "opus"],
    });

    args.push("-"); // stdout como output pro ffmpeg

    return args.iter().map(|f| f.to_string()).collect();
}

pub type ConsumerPacket = Bytes;

// singleton - um por estação
pub struct AudioEncoder {
    encoder_in: BufWriter<ChildStdin>,
    child: std::process::Child,
}

impl AudioEncoder {
    pub fn new(output_codec: &OutputCodec, output: Arc<OutputStream>) -> AudioEncoder {
        let args: Vec<String> = ffmpeg_args(&output_codec);

        println!("encoder: Parâmetros ffmpeg: {:?}", args);

        let mut child = Command::new("ffmpeg")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("encoder: Falha ao spawnar o ffmpeg");

        if let Some(stdout) = child.stdout.take() {
            let mut stdout_reader = BufReader::new(stdout);
            thread::spawn(move || {
                println!("encoder: Thread de consumidor de áudio iniciada.");

                let mut buf = vec![0u8; 8192];
                loop {
                    let n = stdout_reader
                        .read(&mut buf)
                        .expect("encoder: Ler stdout do encoder falhou - processo crashou?");

                    match n {
                        0 => panic!("encoder: Stdout finalizou, estação acabou!"),
                        1.. => {
                            // não é exatamente zero-copy, mas sim "one-copy"
                            // uma vez que alocamos esse Bytes, ele é reference-counted, igual o Arc
                            // ao transmití-lo pelo tokio::sync::broadcast::Sender ele não vai fazer novas cópias de memória
                            // então pagamos um custo fixo, uma vez só
                            let packet = Bytes::copy_from_slice(&buf[..n]);

                            output.push(packet);
                        }
                    }
                }
            });
        }

        let stdin = child.stdin.take().expect("encoder: Falha ao ler stdin");
        let stdin_writer = BufWriter::new(stdin);

        AudioEncoder {
            encoder_in: stdin_writer,
            child: child,
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

impl Drop for AudioEncoder {
    fn drop(&mut self) {
        self.child
            .kill()
            .expect("encoder: ffmpeg não pôde ser fechado");
    }
}
