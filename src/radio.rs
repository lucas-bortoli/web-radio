

struct Radio {
    tracks: Vec<String>,
    current_track: String,
    seed: u64,

}

struct Station {
    station_name: String,
    subscribers: Vec<StationSubscriber>,
}


// estrutura de dados para armazenar o estado da estação
struct StateStation {
    
}

// estrutura para armazenamento de estado anterior da estação
// talvez de pra usar pra n ter repetição de música ou algo assim, n sei ainda
struct StationSnapshot {

}

// estrutura para armazenaro os clientes que estão escutando a estação
// pode ser utilizado para deixar em sleep a estação caso n tenha ninguém escutando
// ou podemos só usar para gerenciar o estado atual, se fizermos a implementação spawnar uma radio quando algum cliente se conectar
struct StationSubscriber {
    
}