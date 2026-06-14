use std::thread::sleep;
use std::time::Duration;

pub fn limpar_tela() {
    // Limpa a tela (funciona em sistemas Unix)
    clearscreen::clear().expect("Falha ao limpar a tela");
}

pub fn esperar(tempo:u64) {
    sleep(Duration::from_secs(tempo));
}