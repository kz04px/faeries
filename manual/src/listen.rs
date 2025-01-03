use crate::{Manual, ManualGameResult};
use std::io::Write;

pub fn listen(state: &mut Box<dyn Manual>) -> std::io::Result<()> {
    let depth = 3;
    let mut update = true;

    while !state.is_gameover() {
        if update {
            state.print();
            update = false;
        }

        print!("> ");
        std::io::stdout().flush()?;

        // Get input
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }
        println!();

        // Manual move
        let stream = input.split_ascii_whitespace();
        for word in stream {
            match word {
                "pass" => {}
                "hint" => {
                    state.hint(depth);
                    println!();
                }
                "quit" => return Ok(()),
                _ => {
                    let success = state.makemove(word);

                    // Computer move
                    if success {
                        state.play(depth);
                        update = true;
                    }
                }
            }
        }
    }

    state.print();

    match state.get_result() {
        Some(ManualGameResult::P1Win) => println!("Player 1 wins!"),
        Some(ManualGameResult::P2Win) => println!("Player 2 wins!"),
        Some(ManualGameResult::Draw) => println!("Draw!"),
        None => println!("Game not finished"),
    }

    Ok(())
}
