use std::io::{self, Write};
// Import not needed; types inferred where used

mod chess;
mod engine;

fn main() {
    println!("AI Chess (Rust CLI) - digita 'quit' per uscire");

    // Stato di gioco minimo
    let mut state = chess::GameState::new();
    let engine = engine::MctsEngine::new(engine::MctsConfig::default());
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        let _ = io::stdout().flush();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Errore di input");
            continue;
        }

        let cmd = input.trim();
        match cmd {
            "quit" | "exit" => {
                println!("Ciao!");
                break;
            }
            "help" => {
                println!("Comandi: help, quit, ai, move <uci>, board");
            }
            "board" => {
                println!("{}", state.board_string());
            }
            "ai" => {
                match engine.choose_move(&state) {
                    Some(mv) => {
                        println!("AI sceglie: {mv}");
                        state.apply_move(mv);
                        println!("{}", state.board_string());
                    }
                    None => println!("Nessuna mossa disponibile"),
                }
            }
            cmd if cmd.starts_with("move ") => {
                let mv_txt = cmd.trim_start_matches("move ").trim();
                if mv_txt.is_empty() {
                    println!("Specifica una mossa UCI, es: move e2e4");
                } else if let Some(mv) = chess::GameState::parse_uci_move(mv_txt) {
                    let legal = state.legal_moves();
                    if legal.contains(&mv) {
                        state.apply_move(mv);
                        println!("{}", state.board_string());
                    } else {
                        println!("Mossa illegale nel contesto attuale");
                    }
                } else {
                    println!("Formato UCI non valido (es: e2e4, e7e8q)");
                }
            }
            _ if cmd.is_empty() => {}
            other => {
                println!("Comando non riconosciuto: {other}");
            }
        }
    }
}
