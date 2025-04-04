use std::{
    fs::File,
    io::{BufReader, Read},
    process::{ChildStdout, Command, Stdio},
};

use super::input_audio_file::{AudioFile, AudioPacket};

pub const CHANNEL_COUNT: u32 = 2;
pub const SAMPLE_RATE: u32 = 44100;
pub const BYTE_DEPTH: u32 = 2; //16bits

pub const STDOUT_BUFFER_SIZE: u32 = SAMPLE_RATE * CHANNEL_COUNT * BYTE_DEPTH;

/// Converte o número de bytes de um buffer PCM para sua duração em segundos
fn calculate_buffer_length(buffer_capacity_bytes: u32) -> f64 {
    let bytes_per_sample = CHANNEL_COUNT * BYTE_DEPTH;
    let samples_per_second = SAMPLE_RATE;
    let buffer_length_seconds =
        buffer_capacity_bytes as f64 / (bytes_per_sample as f64 * samples_per_second as f64);
    buffer_length_seconds
}

pub struct MP3File {
    file_path: String,
    file_size: u64,

    reader: BufReader<ChildStdout>,
}

impl AudioFile for MP3File {
    fn new(file_path: String) -> MP3File {
        let file = File::open(file_path.clone()).expect("Failed to open file");
        let file_size = file.metadata().unwrap().len();

        let mut child = Command::new("ffmpeg")
            .args(&[
                "-i",
                &file_path,
                "-f",
                "s16le",
                "-ac",
                &CHANNEL_COUNT.to_string(),
                "-ar",
                &SAMPLE_RATE.to_string(),
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Falha ao spawnar o ffmpeg");

        let stdout = child.stdout.take().expect("Falha ao ler stdout");
        let reader = BufReader::new(stdout);

        MP3File {
            file_path,
            file_size,
            reader,
        }
    }

    fn audio_file_path(&self) -> String {
        self.file_path.clone()
    }

    fn audio_file_size_bytes(&self) -> u64 {
        self.file_size
    }
}

impl Iterator for MP3File {
    type Item = AudioPacket;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; STDOUT_BUFFER_SIZE as usize];
        let n = self.reader.read(&mut buffer).expect("Falha ao ler bytes");

        if n == 0 {
            println!("EOF");
            return None;
        }

        let audio_length = calculate_buffer_length(n as u32);
        println!("{} bytes lidos ({:.4}s)", n, audio_length);

        return Some(AudioPacket {
            audio_length,
            buffer: buffer[..n].to_vec(),
        });
    }
}
