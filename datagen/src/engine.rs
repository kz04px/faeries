use faeries::primary::GameSearch;
use faeries::search::SearchInfo;
use faeries::search::alphabeta::alphabeta;
use faeries::search::eval::Eval;
use faeries::search::flatmc::flatmc;
use faeries::search::minimax::minimax;
use faeries::search::random::random;
use faeries::state::{ExtraOptions, GameState};
use faeries::ugi::UgiGame;
use games::gamerules::GameRules;
use protocols::GoSettings;
use std::cell::Cell;

/// Score reported for a forced win or loss.
pub const MATE_CP: i32 = 32_000;

/// What a search found.
pub struct Searched<M> {
    pub mv: Option<M>,
    /// Centipawns for the side to move, if the search reported one.
    pub score: Option<i32>,
}

/// A game's engine, as datagen drives it.
pub trait Engine: Default {
    type Game: GameRules;

    fn init(&mut self);

    fn newgame(&mut self);

    fn set_search(&mut self, name: &str) -> Result<(), String>;

    fn search(
        &mut self,
        pos: &Self::Game,
        settings: &GoSettings,
    ) -> Searched<<Self::Game as GameRules>::MoveType>;
}

impl<G: UgiGame + Eval, S: GameSearch<Game = G>, X: ExtraOptions> Engine for GameState<G, S, X> {
    type Game = G;

    fn init(&mut self) {
        self.game_search.init();
        self.initialised = true;
    }

    fn newgame(&mut self) {
        self.game_search.newgame();
    }

    fn set_search(&mut self, name: &str) -> Result<(), String> {
        if self.search.options.iter().any(|o| o == name) {
            self.search.value = name.to_owned();
            Ok(())
        } else {
            Err(format!(
                "Unknown search '{}', expected one of: {}",
                name,
                self.search.options.join(", ")
            ))
        }
    }

    fn search(&mut self, pos: &G, settings: &GoSettings) -> Searched<G::MoveType> {
        let score = Cell::new(None);
        let info_handler = |_: &G, info: &SearchInfo<G::MoveType>| {
            if let Some(mate) = info.mate {
                score.set(Some(if mate > 0 { MATE_CP } else { -MATE_CP }));
            } else if let Some(cp) = info.score {
                score.set(Some(cp.clamp(-MATE_CP, MATE_CP)));
            }
        };
        let eval = &G::eval;

        let name = self.search.value.as_str();
        let found = match self
            .game_search
            .search(name, pos.clone(), settings, &info_handler)
        {
            Some(found) => found,
            None => match name {
                "random" => random(pos, || self.prng.next()),
                "minimax" => minimax(pos.clone(), settings, &info_handler, eval),
                "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, eval),
                "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next())
                    .unwrap_or(None),
                _ => panic!("Unknown search type"),
            },
        };

        Searched {
            mv: found,
            score: score.get(),
        }
    }
}
