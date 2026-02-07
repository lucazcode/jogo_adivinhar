mod jogo;
mod ui;
mod util;

use rpassword::prompt_password;

fn main() {
    loop {
        if jogo::logica_jogo() {
            continue;
        } else {
            println!("[>] Obrigado por jogar! - @lucazcode\n");
            let _ = prompt_password("[?] Pressione ENTER para sair...");
            println!();
            break;
        }
    }
}
