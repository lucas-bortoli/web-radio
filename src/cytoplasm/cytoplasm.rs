use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    input_decoder::input_audio_file::{self, AudioPacket},
    output_encoder::AudioEncoder,
};

pub struct Cytoplasm {
    decoder_handle: JoinHandle<()>,
    encoder_handle: JoinHandle<()>,

    buffer: Arc<Mutex<VecDeque<AudioPacket>>>,
}

impl Cytoplasm {
    pub fn new(mut audio_encoder: AudioEncoder) -> Cytoplasm {
        let shared_buffer = Arc::new(Mutex::new(VecDeque::<AudioPacket>::new()));

        const SETPOINT_LOW: usize = 5;
        const SETPOINT_HIGH: usize = 10;
        const FUCKALL_DURATION: Duration = Duration::from_millis(5);

        let decoder_buffer = shared_buffer.clone();
        let decoder_handle = thread::spawn(move || loop {
            let file = input_audio_file::open_input_file_strategy(
                "./CorruptSaveLonelyHeart.mp3".to_string(),
            );

            for packet in file {
                let mut buffer = decoder_buffer.lock().unwrap();
                if buffer.len() >= SETPOINT_HIGH {
                    // eprintln!("cytoplasm/d: Backpressure! Pausando encoder...");

                    drop(buffer); // liberar mutex imediatamente

                    // fazer porra nenhuma até o buffer estar quase vazio
                    'backpressure: loop {
                        thread::sleep(FUCKALL_DURATION);
                        let buffer = decoder_buffer.lock().unwrap();
                        if buffer.len() <= SETPOINT_LOW {
                            // eprintln!("cytoplasm/d: Backpressure acabou!");
                            break 'backpressure;
                        }
                    }

                    // finalmente continuar enfileirando pacotes
                    decoder_buffer.lock().unwrap().push_back(packet);
                } else {
                    // enfileirar pacote imediatamente; ainda cabe no buffer
                    buffer.push_back(packet);
                }
            }
        });

        let encoder_buffer = shared_buffer.clone();
        let encoder_handle = thread::spawn(move || loop {
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
            block_until_buffer_full(&encoder_buffer);

            let start = Instant::now();
            let mut playback_time = 0.0;

            loop {
                let mut buffer = encoder_buffer.lock().unwrap();
                if buffer.len() == 0 {
                    eprintln!("cytoplasm/e: Underrun...");
                    drop(buffer);
                    block_until_buffer_full(&encoder_buffer);
                } else {
                    // consumir todo o áudio da fila
                    let mut consumed_audio = Vec::new();
                    while buffer.len() > 0 {
                        // eprintln!("cytoplasm/e: consume...");
                        consumed_audio.push(buffer.pop_front().unwrap());
                    }

                    // liberar mutex para que possam continuar enfileirando pacotes na outra thread
                    drop(buffer);

                    // transmitir o áudio, dar sleep
                    for packet in consumed_audio {
                        playback_time += packet.audio_length;
                        audio_encoder.push_audio_packet(packet);
                    }

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

        return Cytoplasm {
            buffer: shared_buffer,
            decoder_handle,
            encoder_handle,
        };
    }
}
