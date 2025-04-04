pub trait AudioFile: Iterator {
    fn new(file_path: String) -> Self;
    fn audio_file_path(&self) -> String;
    fn audio_file_size_bytes(&self) -> u64;
}

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
