use crate::state::GameState;
use crate::tak6x6::{DEFAULT_HASH_MB, TakState};
use crate::ugi::{UgiGame, info_handler};
use games::gamerules::GameRules;
use games::tak::{Dir, TakMove, TakPosition};
use protocols::ugi::options::Spin;
use protocols::ugi::{GoSettings, UGI, UGIGameResult};

impl<const SIZE: usize> UgiGame for TakPosition<SIZE> {
    fn as_ugi(&self, mv: &TakMove<SIZE>) -> String {
        match mv {
            TakMove::Drop(sq, kind) => format!(
                "{}{}",
                match kind {
                    games::tak::PieceType::Flat => "",
                    games::tak::PieceType::Standing => "S",
                    games::tak::PieceType::Cap => "C",
                },
                sq
            ),
            TakMove::Spread(sq, dir, height, coverage, is_crush) => {
                debug_assert!(*height > 0);
                debug_assert!(*height as usize <= SIZE);
                debug_assert!(*coverage != 0);

                format!(
                    "{}{}{}{}{}",
                    if *height > 1 {
                        height.to_string()
                    } else {
                        "".to_string()
                    },
                    sq,
                    match dir {
                        Dir::Up => "+",
                        Dir::Down => "-",
                        Dir::Left => "<",
                        Dir::Right => ">",
                    },
                    {
                        let mut gg = String::new();

                        let mut head = 0;
                        let mut tail = 0;

                        while ((coverage >> head) & 1) == 0 {
                            head += 1;
                            tail += 1;
                        }

                        let end = head + *height;
                        while head < end {
                            head += 1;

                            if ((coverage >> head) & 1) == 1 {
                                gg += &(head - tail).to_string();
                                tail = head;
                            }
                        }

                        if head > tail {
                            gg += &(head - tail).to_string();
                        }

                        if gg.len() != 1 { gg } else { "".to_string() }
                    },
                    if *is_crush { "*" } else { "" }
                )
            }
        }
    }

    fn unrecognised_command(&self, word: &str) {
        if word == "print" {
            println!("FEN: {}", self.get_fen());
            print!("{}", self);
        }
    }
}

pub enum TakStates {
    Size3(GameState<TakPosition<3>>),
    Size4(GameState<TakPosition<4>>),
    Size5(GameState<TakPosition<5>>),
    Size6(GameState<TakPosition<6>>),
    Size7(GameState<TakPosition<7>>),
    Size8(GameState<TakPosition<8>>),
}

/// Run `$body` against whichever board size is active, bound as `$state`.
macro_rules! dispatch {
    ($states:expr, $state:ident => $body:expr) => {
        match $states {
            TakStates::Size3($state) => $body,
            TakStates::Size4($state) => $body,
            TakStates::Size5($state) => $body,
            TakStates::Size6($state) => $body,
            TakStates::Size7($state) => $body,
            TakStates::Size8($state) => $body,
        }
    };
}

impl TakStates {
    /// A new game of the given size, keeping the options and prng of `old`.
    fn resized<const SIZE: usize>(size: i32, old: GameState<TakPosition<SIZE>>) -> Self {
        match size {
            3 => Self::Size3(old.with_pos(TakPosition::startpos())),
            4 => Self::Size4(old.with_pos(TakPosition::startpos())),
            5 => Self::Size5(old.with_pos(TakPosition::startpos())),
            6 => Self::Size6(old.with_pos(TakPosition::startpos())),
            7 => Self::Size7(old.with_pos(TakPosition::startpos())),
            8 => Self::Size8(old.with_pos(TakPosition::startpos())),
            _ => panic!("Unsupported size"),
        }
    }
}

pub struct UGITakState {
    pub state: TakStates,
    pub primary: TakState,
    // Options
    pub size: Spin<i32>,
    pub hash: Spin<usize>,
}

impl Default for UGITakState {
    fn default() -> Self {
        Self {
            state: TakStates::Size6(GameState::default()),
            primary: TakState::default(),
            // Options
            size: Spin {
                name: "size".to_owned(),
                min: 3,
                max: 8,
                value: 6,
            },
            hash: Spin {
                name: "hash".to_owned(),
                min: 1,
                max: 65536,
                value: DEFAULT_HASH_MB,
            },
        }
    }
}

impl UGI for UGITakState {
    fn init(&mut self) {
        self.primary.init();
        dispatch!(&mut self.state, state => state.init())
    }

    fn shutdown(&mut self) {
        dispatch!(&mut self.state, state => state.shutdown())
    }

    fn name(&self) -> String {
        dispatch!(&self.state, state => state.name())
    }

    fn author(&self) -> String {
        dispatch!(&self.state, state => state.author())
    }

    fn uginewgame(&mut self) {
        self.primary.newgame();
        dispatch!(&mut self.state, state => state.uginewgame())
    }

    fn isready(&mut self) {
        dispatch!(&mut self.state, state => state.isready())
    }

    fn position(&mut self, fen: &str) {
        dispatch!(&mut self.state, state => state.position(fen))
    }

    fn moves(&mut self, movestr: &str) {
        dispatch!(&mut self.state, state => state.moves(movestr))
    }

    fn go(&mut self, settings: &GoSettings) {
        // The 6x6 primary search keeps its state between searches
        if let TakStates::Size6(state) = &self.state
            && state.search.value == "primary"
            && !state.pos.is_gameover()
        {
            let settings = &protocols::GoSettings::from(settings);
            let info_handler = &info_handler::<TakPosition<6>>;
            match self.primary.search(state.pos.clone(), settings, info_handler) {
                Some(mv) => println!("bestmove {}", state.pos.as_ugi(&mv)),
                None => println!("bestmove 0000"),
            }
            return;
        }

        dispatch!(&mut self.state, state => state.go(settings))
    }

    fn stop(&mut self) {
        dispatch!(&mut self.state, state => state.stop())
    }

    fn unrecognised_command(
        &mut self,
        word: &str,
        args: &mut std::iter::Peekable<std::str::SplitAsciiWhitespace>,
    ) {
        dispatch!(&mut self.state, state => state.unrecognised_command(word, args))
    }

    fn print_options(&self) {
        dispatch!(&self.state, state => {
            println!("{}", state.debug);
            println!("{}", self.size);
            println!("{}", self.hash);
            println!("{}", state.search);
        })
    }

    fn set_option(&mut self, name: &str, value: &str) {
        if name == "size" {
            self.size.value = value.parse().unwrap();
            let old = std::mem::replace(&mut self.state, TakStates::Size3(GameState::default()));
            self.state = dispatch!(old, state => TakStates::resized(self.size.value, state));
        } else if name == "hash" {
            if let Ok(mb) = value.parse::<usize>() {
                self.hash.value = mb.clamp(self.hash.min, self.hash.max);
                self.primary.set_hash(self.hash.value);
            }
        } else {
            dispatch!(&mut self.state, state => state.set_option(name, value))
        }
    }

    fn query_p1turn(&self) -> bool {
        dispatch!(&self.state, state => state.query_p1turn())
    }

    fn query_result(&self) -> Option<UGIGameResult> {
        dispatch!(&self.state, state => state.query_result())
    }

    fn perft(&mut self, depth: u32) {
        dispatch!(&mut self.state, state => state.perft(depth))
    }

    fn split(&mut self, depth: u32) {
        dispatch!(&mut self.state, state => state.split(depth))
    }

    fn movelist(&self) {
        dispatch!(&self.state, state => {
            state.movelist();
            println!("Total: {}", state.pos.count_moves());
        })
    }
}
