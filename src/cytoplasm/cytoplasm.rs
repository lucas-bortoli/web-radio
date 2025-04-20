use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
    time::{Duration, Instant},
};

use bytes::Bytes;
use rocket::{http::ContentType, response::stream::ByteStream};

use crate::{
    input_decoder::input_audio_file::{self, AudioPacket},
    output_encoder::{
        audio_encoder::{AudioEncoder, OutputCodec},
        null_frames::get_null_frame,
    },
};

const FUCKALL_DURATION: Duration = Duration::from_millis(5);
const SETPOINT_HIGH: usize = 10;
const SETPOINT_LOW: usize = 5;

pub struct Cytoplasm {
    encoders: Arc<Mutex<HashMap<OutputCodec, AudioEncoder>>>,
}

impl Cytoplasm {
    pub fn new(station_directory: PathBuf, output_codecs: &[OutputCodec]) -> Cytoplasm {
        let buffer = Arc::new(Mutex::new(VecDeque::<AudioPacket>::new()));
        let encoders = Self::init_encoders(&output_codecs);

        Self::init_decoder_thread(station_directory.clone(), buffer.clone());
        Self::init_encoder_thread(encoders.clone(), buffer.clone());

        return Cytoplasm { encoders };
    }

    pub fn create_output_stream(
        &self,
        codec: &OutputCodec,
    ) -> Result<(ContentType, ByteStream![Vec<u8>]), &'static str> {
        let encoders = self.encoders.lock().unwrap();

        let stream = encoders.get(&codec);
        if stream.is_none() {
            return Err("cytoplasm: stream not found");
        }

        let (tx, rx) = mpsc::channel::<Bytes>();

        stream.unwrap().register_consumer(tx);

        let codec_owned = codec.to_owned();
        Ok((
            ContentType::new("audio", "mpeg"),
            ByteStream! {
                yield get_null_frame(&codec_owned).to_vec();
                eprintln!("server: Frame MP3 null enviado");

                'receive: loop {
                    match rx.recv() {
                        Ok(chunk) => {
                            yield Vec::from_iter(chunk.into_iter());
                        }
                        Err(e) => {
                            eprintln!("server: Broadcast channel closed: {:?}", e);
                            break 'receive;
                        }
                    }
                }

                eprintln!("server: Closing stream")
            },
        ))
    }

    /// cria e inicializa um encoder de áudio para cada codec de saída solicitado
    fn init_encoders(codecs: &[OutputCodec]) -> Arc<Mutex<HashMap<OutputCodec, AudioEncoder>>> {
        let mut map = HashMap::new();
        for codec in codecs {
            let encoder = AudioEncoder::new(&codec);
            map.insert(codec.clone(), encoder);
        }
        Arc::new(Mutex::new(map))
    }

    /// inicia a thread responsável por decodificar arquivos de áudio
    /// ela carrega trilhas conforme definidas e enfileira pacotes no buffer compartilhado
    fn init_decoder_thread(station_directory: PathBuf, buffer: Arc<Mutex<VecDeque<AudioPacket>>>) {
        thread::spawn(move || loop {
            let next_track = station_directory.join("bicameral_mind.mp3");

            eprintln!(
                "cytoplasm/d: abrindo arquivo: {}",
                next_track.to_str().unwrap()
            );

            let file =
                input_audio_file::open_input_file_strategy(next_track.to_str().unwrap().to_owned());
            for packet in file {
                let mut buf_guard = buffer.lock().unwrap();
                if buf_guard.len() >= SETPOINT_HIGH {
                    // eprintln!("cytoplasm/d: Backpressure! Pausando encoder...");

                    drop(buf_guard); // liberar mutex imediatamente

                    // fazer porra nenhuma até o buffer estar quase vazio
                    'backpressure: loop {
                        thread::sleep(FUCKALL_DURATION);
                        let buf_guard = buffer.lock().unwrap();
                        if buf_guard.len() <= SETPOINT_LOW {
                            // eprintln!("cytoplasm/d: Backpressure acabou!");
                            break 'backpressure;
                        }
                    }

                    // finalmente continuar enfileirando pacotes
                    buffer.lock().unwrap().push_back(packet);
                } else {
                    // enfileirar pacote imediatamente; ainda cabe no buffer
                    buf_guard.push_back(packet);
                }
            }
        });
    }

    /// inicia a thread que consome pacotes do buffer, envia para os encoders e mantém o timing de reprodução
    fn init_encoder_thread(
        encoders: Arc<Mutex<HashMap<OutputCodec, AudioEncoder>>>,
        buffer: Arc<Mutex<VecDeque<AudioPacket>>>,
    ) {
        thread::spawn(move || loop {
            fn block_until_buffer_full(buffer: &Arc<Mutex<VecDeque<AudioPacket>>>) {
                // fazer porra nenhuma até o buffer estar cheio
                loop {
                    thread::sleep(FUCKALL_DURATION);
                    let guard = buffer.lock().unwrap();
                    if guard.len() >= SETPOINT_HIGH {
                        // finalmente buffer cheio; a outra thread deve ter printado "BACKPRESSURE!!"
                        eprintln!("cytoplasm/e: Buffering alcançado!");
                        break;
                    }
                }
            }

            // inicialmente vamos deixar o buffer encher completamente, antes de começar a consumi-lo
            // isso previne underruns durante o setup
            block_until_buffer_full(&buffer);

            let start = Instant::now();
            let mut playback_time = 0.0;

            loop {
                let mut buf_guard = buffer.lock().unwrap();
                if buf_guard.len() == 0 {
                    eprintln!("cytoplasm/e: Underrun...");
                    drop(buf_guard);
                    block_until_buffer_full(&buffer);
                } else {
                    // consumir todo o áudio da fila
                    let mut consumed_audio = Vec::new();
                    while buf_guard.len() > 0 {
                        // eprintln!("cytoplasm/e: consume...");
                        consumed_audio.push(buf_guard.pop_front().unwrap());
                    }

                    // liberar mutex para que possam continuar enfileirando pacotes na outra thread
                    drop(buf_guard);

                    // transmitir o áudio para todos os encoders, dar sleep
                    let mut encoders_guard = encoders.lock().unwrap();
                    for packet in consumed_audio {
                        playback_time += packet.audio_length;
                        for encoder in encoders_guard.values_mut() {
                            encoder.push_audio_packet(packet.clone());
                        }
                    }
                    drop(encoders_guard);

                    // ao calcular o "next_time" com base em um start_time fixo, garantimos que pequenos atrasos não se acumulem ao longo do tempo.
                    // usar apenas thread::sleep() pela duração de cada packet causaria desvios cumulativos, já que o tempo de execução de cada iteração varia.
                    // assim, mesmo que uma iteração atrase um pouco, a próxima tentará se alinhar com o tempo real correto.
                    let next_time = start + Duration::from_secs_f64(playback_time);
                    let now = Instant::now();
                    if next_time > now {
                        thread::sleep(next_time - now);
                    } else {
                        eprintln!("cytoplasm/e: Time underrun...");
                    }
                }
            }
        });
    }
}
