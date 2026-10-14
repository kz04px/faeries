use crate::events::Event;
use games::gamerules::GameRules;
use protocols::GoSettings;
use std::sync::mpsc;

// fn info_handler<G: GameRules>(
//     _: &G,
//     _: Option<i32>,
//     _: Option<i32>,
//     _: Option<i32>,
//     _: Option<i32>,
//     _: Option<u64>,
//     _: Option<u128>,
//     _: Option<i32>,
//     _: &Vec<G::MoveType>,
// ) {
// }

fn get_move<G: GameRules>(pos: &G, _settings: &GoSettings) -> Option<G::MoveType> {
    Some(pos.legal_moves()[0])
}

pub fn play<G: GameRules>(
    thread_id: usize,
    fen: &str,
    id: usize,
    settings: &GoSettings,
    tx: &mpsc::Sender<Event<G>>,
) {
    tx.send(Event::GameStart {
        thread_id,
        id,
        fen: fen.to_string(),
    })
    .unwrap();

    let mut pos = G::from_fen(fen);

    while !pos.is_gameover() {
        let res = get_move(&pos, settings);

        if let Some(mv) = res {
            tx.send(Event::Move { thread_id, id, mv }).unwrap();
            pos.makemove(&mv);
        } else {
            panic!("No move returned");
        }
    }

    tx.send(Event::GameFinish {
        thread_id,
        id,
        result: pos.get_result(),
    })
    .unwrap();
}
