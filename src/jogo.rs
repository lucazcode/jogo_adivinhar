use crate::ui::{fim_de_jogo, informacoes, interface};
use crate::util::mudar_feedback;

use rand::Rng;
use std::cmp::Ordering;
use std::num::IntErrorKind;

use std::io::{self, Write, stdout};

pub fn logica_jogo() -> bool {
    const MIN: u8 = 1;
    const MAX: u8 = 100;

    let numero_secreto = rand::rng().random_range(MIN..=MAX);
    let mut msg_feedback: String = String::from("[>] Confie no seu instinto!");
    let mut tentativas: u8 = 6;
    let mut num_repetidos: Vec<u8> = Vec::new();

    loop {
        interface();
        informacoes(&msg_feedback, tentativas);

        print!("\n[?] Digite seu palpite: ");
        stdout().flush().unwrap();

        let mut palpite: String = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("[!] ERRO: Falha ao receber entrada.");

        // Filtra a entrada do usuário
        let palpite: u8 = match palpite.trim().parse::<i32>() {
            // É um número válido || É um palpite repetido
            Ok(num)
                if (MIN as i32..=MAX as i32).contains(&num)
                    && num_repetidos.contains(&(num as u8)) =>
            {
                mudar_feedback(
                    &mut msg_feedback,
                    &format!("[!] ERRO: Você já sugeriu este número ({}).", num),
                );
                continue;
            }

            // É um número válido || É um novo palpite
            Ok(num) if (MIN as i32..=MAX as i32).contains(&num) => {
                num_repetidos.push(num as u8);
                num as u8
            }

            // É um número válido || não está no escopo
            Ok(_) => {
                mudar_feedback(
                    &mut msg_feedback,
                    &format!("[!] ERRO: Seu palpite deve estar entre {} e {}.", MIN, MAX),
                );
                continue;
            }

            Err(e) => {
                match e.kind() {
                    // É um número inválido (estoura o limite do buffer)
                    IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                        mudar_feedback(
                            &mut msg_feedback,
                            &format!("[!] ERRO: Seu palpite deve estar entre {} e {}.", MIN, MAX),
                        );
                    }

                    // Não é um número
                    _ => mudar_feedback(
                        &mut msg_feedback,
                        "[!] ERRO: Seu palpite deve ser um número.",
                    ),
                };
                continue;
            }
        };

        // Retorna comparação e trata individualmente com match
        match palpite.cmp(&numero_secreto) {
            // Trata retorno 'igual'
            Ordering::Equal => {
                mudar_feedback(
                    &mut msg_feedback,
                    &if tentativas > 1 {
                        format!(
                            "[>] Seu palpite ({}) é o número secreto! Você venceu com folga! =)",
                            palpite
                        )
                    } else {
                        format!(
                            "[>] Seu palpite ({}) é o número secreto! Você venceu POR POUCO! =O",
                            palpite
                        )
                    },
                );

                fim_de_jogo(&msg_feedback, tentativas);

                return reiniciar_jogo(&msg_feedback, tentativas);
            }

            // Trata retorno 'menor' ou 'maior'
            ordem => {
                let dica = match ordem {
                    Ordering::Less => "MENOR",
                    Ordering::Greater => "MAIOR",
                    _ => unreachable!(),
                };

                tentativas -= 1;

                mudar_feedback(
                    &mut msg_feedback,
                    &format!(
                        "[>] Seu palpite ({}) é {} que o número secreto.",
                        palpite, dica
                    ),
                );
            }
        }

        if checar_derrota(tentativas) {
            mudar_feedback(
                &mut msg_feedback,
                "[>] Suas tentativas acabaram... Você perdeu. =(",
            );

            fim_de_jogo(&msg_feedback, tentativas);

            return reiniciar_jogo(&msg_feedback, tentativas);
        }
    }
}

fn reiniciar_jogo(msg: &str, ten: u8) -> bool {
    loop {
        let mut entrada: String = String::new();

        print!("[?] Deseja jogar novamente? '1' (Sim) ou '2' (Não): ");
        stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut entrada)
            .expect("[!] ERRO: Falha ao receber entrada.");

        match entrada.trim().parse::<u8>() {
            Ok(1) => return true,
            Ok(2) => {
                fim_de_jogo(msg, ten);
                return false;
            }
            _ => {
                // Imprime mensagem temporária para preservar mensagem original
                fim_de_jogo("[!] ERRO: Insira '1' (Sim) ou '2' (Não).", ten);
                continue;
            }
        };
    }
}

fn checar_derrota(ten: u8) -> bool {
    ten < 1
}
