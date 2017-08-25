extern crate piston_window;
extern crate rand;

mod side;
mod view;

use side::Side;
use view::View;
use piston_window::keyboard::Key;
use rand::Rng;

enum GameState {
    Init,
    Preparing { time_to_start: f64 },
    Running { elapsed_time: f64, side: Side },
    Result { elapsed_time: f64, is_correct: bool },
    FalseStart,
}

pub struct App {
    game_state: GameState,
    rng: Box<Rng>,
}

impl App {
    pub fn new() -> Self {
        App {
            game_state: GameState::Init,
            rng: Box::new(rand::thread_rng()),
        }
    }

    pub fn update(&mut self, dt: f64) {
        match self.game_state {
            GameState::Preparing { time_to_start } => {
                let time_to_start = time_to_start - dt;

                if time_to_start < 0.0 {
                    self.game_state = GameState::Running {
                        elapsed_time: 0.0,
                        side: if self.rng.gen() {
                            Side::Left
                        } else {
                            Side::Right
                        },
                    }
                } else {
                    self.game_state = GameState::Preparing {
                        time_to_start: time_to_start,
                    }
                }
            }
            GameState::Running { elapsed_time, side } => {
                self.game_state = GameState::Running {
                    elapsed_time: elapsed_time + dt,
                    side: side,
                }
            }
            _ => (),
        }
    }

    pub fn key(&mut self, key: Key) {
        match (&self.game_state, key) {
            (&GameState::Preparing { .. }, _) => self.game_state = GameState::FalseStart,
            (&GameState::Running { elapsed_time, side }, Key::Left) |
            (&GameState::Running { elapsed_time, side }, Key::Right) => {
                self.game_state = GameState::Result {
                    elapsed_time: elapsed_time,
                    is_correct: match (key, side) {
                        (Key::Left, Side::Left) | (Key::Right, Side::Right) => true,
                        _ => false,
                    },
                }
            }
            (&GameState::Init { .. }, Key::Space) |
            (&GameState::Result { .. }, Key::Space) |
            (&GameState::FalseStart, Key::Space) => {
                self.game_state = GameState::Preparing { time_to_start: 1.0 }
            }
            _ => (),
        }
    }

    pub fn view(&self) -> View {
        match self.game_state {
            GameState::Init => View {
                text: String::from("Press <Space> to start"),
                side: None,
            },
            GameState::Preparing { time_to_start } => View {
                text: format!("time to start: {:.*}", 2, time_to_start),
                side: None,
            },
            GameState::Running { elapsed_time, side } => View {
                text: format!("elapsed time: {:.*}", 2, elapsed_time),
                side: Some(side),
            },
            GameState::Result {
                elapsed_time,
                is_correct,
            } => View {
                text: format!(
                    "You {}! Elapsed time: {:.*}",
                    if is_correct { "win" } else { "lose" },
                    2,
                    elapsed_time
                ),
                side: None,
            },
            GameState::FalseStart => View {
                text: String::from("False start!"),
                side: None,
            },
        }
    }
}
