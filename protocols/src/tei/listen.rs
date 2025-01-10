use crate::GoKind;
use crate::GoSettings;
use crate::tei::go::to_tei_string;

use super::TEI;
use super::go;
use super::moves;
use super::position;
use super::setoption;

pub fn listen(
    state: &mut dyn TEI,
    mut read_input: impl FnMut(&mut String) -> std::io::Result<usize>,
) -> std::io::Result<()> {
    println!("id name {}", state.name());
    println!("id author {}", state.author());
    println!("id version {}", state.version());
    state.print_size_komi();
    state.print_options();
    println!("teiok");

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
            "teinewgame" | "isready" | "print" | "pprint" | "options" | "stop" | "position"
            | "moves" | "go" | "perft" | "split" | "movelist" => {
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
                "teinewgame" => {
                    if stream.next() != Some(&"size") {
                        panic!("Parse error");
                    };

                    // Parse size
                    let size = if let Some(word) = stream.next() {
                        if let Ok(depth) = word.parse::<i32>() {
                            depth
                        } else {
                            panic!("Uh oh");
                        }
                    } else {
                        panic!("Uh oh");
                    };

                    // Parse halfkomi
                    let halfkomi = if stream.next() == Some(&"halfkomi") {
                        if let Some(word) = stream.next() {
                            if let Ok(gg) = word.parse::<i32>() {
                                gg
                            } else {
                                panic!("Could not parse halfkomi value of {}", word);
                            }
                        } else {
                            panic!("Uh oh {}", word);
                        }
                    } else {
                        0
                    };

                    if state.is_debug() {
                        println!("info string new game size {} halfkomi {}", size, halfkomi);
                    }
                    state.teinewgame(size, halfkomi);
                }
                "setoption" => setoption::parse(&mut stream, |name, value| {
                    if state.is_debug() {
                        println!("info string set option '{}' to '{}'", name, value);
                    }
                    state.set_option(name, value);
                }),
                "position" => match position::parse(&mut stream) {
                    Ok(tps) => {
                        if state.is_debug() {
                            println!("info string set tps '{}'", tps);
                        }
                        state.position(&tps);
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
                            println!("info string {}", to_tei_string(&n));
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
