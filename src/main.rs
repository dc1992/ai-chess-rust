use std::io::{self, Write};
use ::chess::Color;

mod game;
mod engine;

fn main() {
    println!("AI Chess (Rust CLI) - digita 'quit' per uscire");

    // Stato di gioco minimo
    let mut state = game::GameState::new();
    let engine = engine::MctsEngine::new(engine::MctsConfig::default());
    let human_color = Color::White; // Human gioca con il Bianco
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
                println!("Comandi: help, quit, move <uci>, board");
            }
            "board" => {
                println!("{}", state.board_string());
            }
            cmd if cmd.starts_with("move ") => {
                let mv_txt = cmd.trim_start_matches("move ").trim();
                if mv_txt.is_empty() {
                    println!("Specifica una mossa UCI, es: move e2e4");
                } else if state.side_to_move() != human_color {
                    println!("Non è il tuo turno.");
                } else if let Some(mv) = game::GameState::parse_uci_move(mv_txt) {
                    let legal = state.legal_moves();
                    if legal.contains(&mv) {
                        // Applica mossa umana
                        state.apply_move(mv);
                        println!("{}", state.board_string());
                        // Controlla fine partita
                        if state.is_terminal() {
                            print_result(&state);
                            continue;
                        }
                        // Turno AI automatico
                        if let Some(ai_mv) = engine.choose_move(&state) {
                            println!("AI sceglie: {ai_mv}");
                            state.apply_move(ai_mv);
                            println!("{}", state.board_string());
                            if state.is_terminal() {
                                print_result(&state);
                            }
                        } else {
                            println!("AI senza mosse disponibili");
                        }
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

fn print_result(state: &game::GameState) {
    use ::chess::BoardStatus;
    match state.board.status() {
        BoardStatus::Checkmate => {
            let winner = !state.board.side_to_move();
            println!("Scaccomatto. Vince {:?}.", winner);
        }
        BoardStatus::Stalemate => println!("Patta per stallo."),
        BoardStatus::Ongoing => {}
    }
}
