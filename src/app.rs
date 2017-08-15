use std::mem;

use piston_window::*;
use rand::Rng;
use keyboard::Key;

#[derive(PartialEq)]
enum Side {
    Left,
    Right,
}

enum GameState {
    Init,
    Preparing { remaining_time: f64 },
    FalseStart,
    Running { passed_time: f64, side: Side },
    Result {
        result_time: f64,
        side: Side,
        player_guess: Side,
    },
}

struct Format {
    text: String,
    left: f32,
    right: f32,
}

pub struct App {
    game_state: GameState,
    rng: Box<Rng>,
}

impl App {
    pub fn new(rng: Box<Rng>) -> App {
        App {
            game_state: GameState::Init,
            rng: rng,
        }
    }

    fn start_game() -> GameState {
        GameState::Preparing {
            remaining_time: 1.5,
        }
    }

    pub fn update(&mut self, dt: f64) {
        match mem::replace(&mut self.game_state, GameState::Init) {
            GameState::Preparing { remaining_time } => {
                let remaining_time = remaining_time - dt;

                if remaining_time < 0.0 {
                    self.game_state = GameState::Running {
                        passed_time: 0.0,
                        side: {
                            if self.rng.gen() {
                                Side::Left
                            } else {
                                Side::Right
                            }
                        },
                    }
                } else {
                    self.game_state = GameState::Preparing {
                        remaining_time: remaining_time,
                    }
                }
            }
            GameState::Running { passed_time, side } => {
                self.game_state = GameState::Running {
                    passed_time: passed_time + dt,
                    side: side,
                }
            }
            current => self.game_state = current,
        }
    }

    pub fn key(&mut self, key: keyboard::Key) {
        match (mem::replace(&mut self.game_state, GameState::Init), key) {
            (GameState::Preparing { .. }, _) => self.game_state = GameState::FalseStart,
            (GameState::Running { passed_time, side }, Key::Left) |
            (GameState::Running { passed_time, side }, Key::Right) => {
                self.game_state = GameState::Result {
                    result_time: passed_time,
                    side: side,
                    player_guess: if key == Key::Left {
                        Side::Left
                    } else {
                        Side::Right
                    },
                }
            }
            (GameState::Init { .. }, Key::Space) |
            (GameState::Result { .. }, Key::Space) |
            (GameState::FalseStart, Key::Space) => self.game_state = App::start_game(),
            (current, _) => self.game_state = current,
        }
    }

    fn no_action_format(text: String) -> Format {
        Format {
            text: text,
            left: 0.5,
            right: 0.5,
        }
    }

    fn some_side_format(text: String, side: &Side) -> Format {
        let diff = 0.125;
        Format {
            text: text,
            left: if let Side::Left = *side {
                0.5 + diff
            } else {
                0.5 - diff
            },
            right: if let Side::Right = *side {
                0.5 + diff
            } else {
                0.5 - diff
            },
        }
    }

    fn format_time(time: f64) -> String {
        format!("{:.precision$}", time, precision = 2)
    }

    fn format(&self) -> Format {
        match self.game_state {
            GameState::Init => App::no_action_format(String::from("Press <Space> to start")),
            GameState::Preparing { remaining_time } => App::no_action_format(format!(
                "time to start: {}",
                App::format_time(remaining_time)
            )),
            GameState::Running {
                passed_time,
                ref side,
            } => App::some_side_format(
                format!("time running: {}", App::format_time(passed_time)),
                side,
            ),
            GameState::FalseStart => App::no_action_format(String::from("You lose! False start!")),
            GameState::Result {
                ref side,
                ref player_guess,
                result_time,
            } => App::some_side_format(
                format!(
                    "You {} in {} seconds",
                    if side == player_guess { "win" } else { "lose" },
                    App::format_time(result_time)
                ),
                side,
            ),
        }
    }

    pub fn render(&mut self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        let format = self.format();

        let view_size = c.get_view_size();
        let w = view_size[0];
        let h = view_size[1];

        let padding = w / 512.0 * 20.0;
        let font_size = (w / 512.0 * 32.0) as u32;
        let side_width = (w as f64) * 0.5 - padding * 1.5;
        let side_height = (h as f64) - (font_size as f64) - padding * 3.0;
        let side_top_padding = (font_size as f64) + padding * 2.0;

        clear([0.5, 0.5, 0.5, 1.0], g);

        text::Text::new(font_size).draw(
            &format.text,
            glyphs,
            &c.draw_state,
            c.transform.trans(padding, (font_size as f64) + padding),
            g,
        );

        rectangle(
            [format.left, 0.0, 0.0, 1.0],
            [padding, side_top_padding, side_width, side_height],
            c.transform,
            g,
        );

        rectangle(
            [format.right, 0.0, 0.0, 1.0],
            [
                side_width + padding * 2.0,
                side_top_padding,
                side_width,
                side_height,
            ],
            c.transform,
            g,
        );
    }
}
