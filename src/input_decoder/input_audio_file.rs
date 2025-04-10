pub trait AudioFile: Iterator {
    fn new(file_path: String) -> Self;
    fn audio_file_path(&self) -> String;
    fn audio_file_size_bytes(&self) -> u64;
}

pub const CHANNEL_COUNT: u32 = 2;
pub const SAMPLE_RATE: u32 = 44100;
pub const BYTE_DEPTH: u32 = 2; //16bits

#[derive(Debug)]
pub struct AudioPacket {
    /**
     * Quanto tempo de áudio este quadro tem, em segundos
     */
    pub audio_length: f64,

    /**
     * O buffer de áudio, formato PCM, com as especificações acima.
     */
    pub buffer: Vec<u8>,
}

/// Converte o número de bytes de um buffer PCM para sua duração em segundos
pub fn calculate_buffer_length(buffer_capacity_bytes: u32) -> f64 {
    let bytes_per_sample = CHANNEL_COUNT * BYTE_DEPTH;
    let samples_per_second = SAMPLE_RATE;
    let buffer_length_seconds =
        buffer_capacity_bytes as f64 / (bytes_per_sample as f64 * samples_per_second as f64);
    buffer_length_seconds
}
