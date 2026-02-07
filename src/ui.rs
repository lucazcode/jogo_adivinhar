use clearscreen::ClearScreen;
use std::sync::OnceLock; // Cria uma abstração permitindo leitura global segura de instâncias

static LIMPA_TELA: OnceLock<ClearScreen> = OnceLock::new();

pub fn interface() {
    limpar_tela();
    println!("[>] ADIVINHE O NÚMERO SECRETO!\n");
    println!("[>] ATENÇÃO: O valor está entre 1 e 100.\n");
}

pub fn informacoes(msg: &str, ten: u8) {
    println!("[>] Tentativas restantes: {}\n", ten);
    println!("{}\n", msg);
}

fn limpar_tela() {
    // Instancia apenas no primeiro uso com 'OnceLock'
    LIMPA_TELA
        .get_or_init(ClearScreen::default)
        .clear()
        .expect("[ERRO]: Falha ao limpar tela.");
}

pub fn fim_de_jogo(msg: &str, ten: u8) {
    interface();
    informacoes(msg, ten);
    println!("\n[*] FIM DE JOGO.\n");
}
