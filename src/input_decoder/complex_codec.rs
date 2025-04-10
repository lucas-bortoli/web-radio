use std::{
    fs::File,
    io::{BufReader, Read},
    process::{ChildStdout, Command, Stdio},
};

use crate::input_decoder::input_audio_file::calculate_buffer_length;

use super::input_audio_file::{AudioFile, AudioPacket, BYTE_DEPTH, CHANNEL_COUNT, SAMPLE_RATE};

pub const FFMPEG_STDOUT_BUFFER_SIZE: u32 = 1 * (SAMPLE_RATE * CHANNEL_COUNT * BYTE_DEPTH); // 1 segundo de áudio

pub struct ComplexCodecFile {
    file_path: String,
    file_size: u64,

    reader: BufReader<ChildStdout>,
}

impl AudioFile for ComplexCodecFile {
    fn new(file_path: String) -> ComplexCodecFile {
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

        ComplexCodecFile {
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

impl Iterator for ComplexCodecFile {
    type Item = AudioPacket;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = [0u8; FFMPEG_STDOUT_BUFFER_SIZE as usize];
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
