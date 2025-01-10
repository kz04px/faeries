use crate::GoKind;
use crate::GoSettings;
use crate::uci::go::to_uci_string;

use super::UCI;
use super::UCIGameResult;
use super::go;
use super::moves;
use super::position;
use super::setoption;

pub fn listen(
    state: &mut dyn UCI,
    mut read_input: impl FnMut(&mut String) -> std::io::Result<usize>,
) -> std::io::Result<()> {
    println!("id name {}", state.name());
    println!("id author {}", state.author());
    state.print_options();
    println!("uciok");

    let mut input = String::new();

    // Pre isready setup
    loop {
        read_input(&mut input)?;

        let mut stream = input.split_ascii_whitespace().peekable();
        match stream.next().unwrap_or("") {
            "setoption" => {
                setoption::parse(&mut stream, |name, value| {
                    if state.is_debug() {
                        println!("info string set option '{}' to '{}'", name, value);
                    }
                    state.set_option(name, value);
                });
            }
            "quit" => {
                if state.is_debug() {
                    println!("info string shutdown");
                }
                state.shutdown();
                return Ok(());
            }
            "ucinewgame" | "isready" | "print" | "pprint" | "options" | "stop" | "position"
            | "moves" | "go" | "perft" | "split" | "query" | "movelist" => {
                break;
            }
            _ => {}
        }

        input.clear();
    }

    if state.is_debug() {
        println!("info string start init");
    }
    state.init();
    if state.is_debug() {
        println!("info string finish init");
    }

    // Post isready
    loop {
        let mut stream = input.split_ascii_whitespace().peekable();
        let mut quit = false;

        while let Some(word) = stream.next() {
            match word {
                // Singles
                "ucinewgame" => state.ucinewgame(),
                "isready" => state.isready(),
                "print" => state.print(),
                "pprint" => state.pprint(),
                "options" => state.print_options(),
                "stop" => state.stop(),
                "quit" => {
                    quit = true;
                    break;
                }
                // Multiples
                "setoption" => setoption::parse(&mut stream, |name, value| {
                    if state.is_debug() {
                        println!("info string set option '{}' to '{}'", name, value);
                    }
                    state.set_option(name, value);
                }),
                "position" => match position::parse(&mut stream) {
                    Ok(fen) => {
                        if state.is_debug() {
                            println!("info string set fen '{}'", fen);
                        }
                        state.position(&fen);
                    }
                    Err(e) => println!("info string position error {}", e),
                },
                "moves" => moves::parse(&mut stream, |movestr| {
                    if state.is_debug() {
                        println!("info string make move '{}'", movestr);
                    }
                    state.moves(movestr);
                }),
                "go" => match go::parse(&mut stream) {
                    Ok(n) => {
                        if state.is_debug() {
                            println!("info string {}", to_uci_string(&n));
                        }
                        match n.kind {
                            GoKind::Search => state.go(&n),
                            GoKind::Perft => state.perft(&n),
                            GoKind::FastPerft => todo!(),
                            GoKind::SplitPerft => state.split(&n),
                        }
                    }
                    Err(e) => println!("info string go error {}", e),
                },
                "perft" => {
                    if stream.peek() == Some(&"depth") {
                        stream.next();
                    };

                    if let Some(word) = stream.next() {
                        if let Ok(depth) = word.parse::<i32>() {
                            state.perft(&GoSettings::from_depth(depth));
                        }
                    }
                }
                "split" => {
                    if stream.peek() == Some(&"depth") {
                        stream.next();
                    };

                    if let Some(word) = stream.next() {
                        if let Ok(depth) = word.parse::<i32>() {
                            state.split(&GoSettings::from_depth(depth));
                        }
                    }
                }
                "query" => {
                    match stream.next() {
                        Some("p1turn") => match state.query_p1turn() {
                            true => println!("response true"),
                            false => println!("response false"),
                        },
                        Some("gameover") => match state.query_gameover() {
                            true => println!("response true"),
                            false => println!("response false"),
                        },
                        Some("result") => match state.query_result() {
                            Some(UCIGameResult::P1Win) => println!("response p1win"),
                            Some(UCIGameResult::P2Win) => println!("response p2win"),
                            Some(UCIGameResult::Draw) => println!("response draw"),
                            None => println!("response none"),
                        },
                        _ => {}
                    };
                }
                "movelist" => state.movelist(),
                _ => {}
            }
        }

        if quit {
            break;
        }

        input.clear();
        read_input(&mut input)?;
    }

    if state.is_debug() {
        println!("info string shutdown");
    }
    state.shutdown();

    Ok(())
}
