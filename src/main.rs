mod jogo;
mod ui;
mod util;

fn main() {
    loop {
        if jogo::logica_jogo() {
            continue;
        } else {
            println!("[>] Obrigado por jogar! - @lucazcode\n");
            break;
        }
    }
}
