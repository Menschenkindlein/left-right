# Creating left/right reaction game tutorial

Let's create a binary crate `left_right`:

```shell
cargo new --bin left_right
```

For now, the only dependency we need is `piston_window`. Let's add it to `Cargo.toml`:

```toml
[dependencies]
piston_window = "*"
```

Now, let's open `main.rs` and write some code.

First of all, we need to link our application to `piston_window` crate:

```rust
extern crate piston_window;
```

Also, it would be useful to import everything from `piston_window` into our namespace:

```rust
use piston_window::*;
```

Now, let's go to the `main` function and create the window for the game:

```rust
let mut window: PistonWindow = WindowSettings::new("Left/Right", [512; 2])
    .build()
    .unwrap();
```

Here we create an instance of `WindowSettings` that specifies the title of the window and its dimensions (note the syntax `[<initial_element>; <size>]`). Then we build an instance of `PistonWindow` from this settings. The result of building a window is a `Result`. It means that we have to handle the case when it is unsuccessful. We use `unwrap` method that extracts successful result and panics on unsuccessful one.

The next step will be to connect to window's event loop:

```rust
while let Some(e) = window.next() {}
```

So, now `main.rs` should look like this:

```rust
extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Left/Right", [512; 2])
        .build()
        .unwrap();

    while let Some(e) = window.next() {}
}
```

Now our application is ready to run. Go to its directory and execute `cargo run`. There will be no much fun, just a square window with nothing inside.

Let's add something to it.

A common way of organizing Rust binary crates is to have all logic in a library crate, and keep `main.rs` as small as possible. Let's create a file `lib.rs` in the same directory as `main.rs`.

Let's talk briefly about what the game is about. There is a starting screen, where a player can press `Space` to start a game. After that, there will be a small period of time for the player to prepare, and two rectangles show up. The player has to see which one is brighter and press the proper arrow button as fast as possible. The time required for the reaction is tracked. Pressing a wrong button, as well as pressing it before the rectangles show up is treated as a failure.

Let's define a language to speak about the game. First of all, the game is about sides: left and right. The game will choose one randomly, and then compare it to the player's guess.

```rust
enum Side {
    Left,
    Right,
}
```

There will be five main states of the game:

1. Initial state
2. State for the player to prepare. It must track the time that left for the actual game to start.
3. State of the running game. It contains the proposed side and tracks the time that takes for the player to react.
4. Result state. It contains the result time and whether the player's answer was correct.
5. A special state for the case when the player answers before the question was asked.

```rust
enum GameState {
    Init,
    Preparing { time_to_start: f64 },
    Running { elapsed_time: f64, side: Side },
    Result { elapsed_time: f64, is_correct: bool },
    FalseStart,
}
```

Let's create a container struct for our application, and add some methods to it:

```rust
pub struct App {
    game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        App {
            game_state: GameState::Init,
        }
    }
}
```

Well, this was straightforward. Now, let's link our application crate to the binary crate.

```rust
extern crate left_right;

use left_right::App;
```

And, create an `App` instance in the `main` function:

```rust
let mut app = App::new();
```

It compiles, but, obviously, does nothing.

Let's add some visuals. It's advisable to separate game logic from visualization logic. So, we'll create a separate module in `lib.rs`:

```rust
mod view;
```

And in the file `view.rs` we'll define the actual logic. For simplicity, let's define our view as some text (for instructions and timing) and two rectangles: same shade when not active, and one brighter when the game is active.

```rust
pub struct View {
    text: String,
    side: Option<Side>,
}
```

Note that in order to use `Side` enum in this module we will need to extract it to a separate module `side.rs`.

```rust
pub enum Side {
    Left,
    Right,
}
```

Then, `view.rs` will look like:

```rust
use side::Side;

pub struct View {
    text: String,
    side: Option<Side>,
}
```

And `lib.rs`:

```rust
mod side;
mod view;

use side::Side;
use view::View;

enum GameState {
    Init,
    Preparing { time_to_start: f64 },
    Running { elapsed_time: f64, side: Side },
    Result { elapsed_time: f64, is_correct: bool },
    FalseStart,
}

pub struct App {
    game_state: GameState,
}

impl App {
    pub fn new() -> Self {
        App {
            game_state: GameState::Init,
        }
    }
}
```

Let's create a method that creates a `View` structure that corresponds to the current state in `App`:

```rust
    pub fn view(&self) -> View {
        match self.game_state {
            GameState::Init => View {
                text: String::from("Press <Space> to start"),
                side: None,
            },
            GameState::Preparing { time_to_start } => View {
                text: format!("time to start: {}", time_to_start),
                side: None,
            },
            GameState::Running { elapsed_time, side } => View {
                text: format!("elapsed time: {}", elapsed_time),
                side: Some(side),
            },
            GameState::Result {
                elapsed_time,
                is_correct,
            } => View {
                text: format!(
                    "You {}! Elapsed time: {}",
                    if is_correct { "win" } else { "lose" },
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
```

Note that we pass `side` from the `Running` state to the newly created `View` structure. To satisfy the borrow checker, we need to create a separate instance of `Side` for `View`. We can do it by hand:

```rust
match side {
    Side::Left => Side::Left,
    Side::Right => Side::Right,
}
```

Or we can derive a `Clone` and `Copy` traits for `Side`, and it will be done automatically:

```rust
#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
}
```

Well, now it compiles. Let's draw something in our window.

First of all, let's take a look at the signature of `PistonWindow`'s `draw_2d` method. It takes a reference to the event and a closure that will be called only if event is `Render`. So, it is totally safe to call this method in the event loop without pattern matching on event type:

```rust
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| { app.view().render(c, g) });
    }
```

The arguments for the closure are `Context` and `Graphics` objects. From the former, you can get the size of the window, and the latter is representing the visual state of your window. You need to change it in order for something to be displayed.

So, let's write the `render` method for our `View`.

First of all, we'll need to import the necessary stuff to our namespace. Add to `view.rs`:

```rust
use piston_window::*;
```

But for this to work, we need to link our library crate (remember, it's separate from the binary crate) to `piston_window`. Add to `lib.rs`:

```rust
extern crate piston_window;
```

After that, write some rendering code:

```rust
impl View {
    pub fn render(&mut self, c: Context, g: &mut G2d) {
        // we will scale everything according to the window size
        let view_size = c.get_view_size();
        let w = view_size[0];
        let h = view_size[1];

        // calculate proper font size
        let font_size = (w / 512.0 * 32.0) as u32;

        // add some padding for a better view
        let padding = w / 512.0 * 20.0;
        // leave some space for text
        let side_top_padding = (font_size as f64) + padding * 2.0;
        let side_height = (h as f64) - side_top_padding - padding;
        let side_width = (w as f64) * 0.5 - padding * 1.5;

        // which rectangle will be brighter
        let left_color_difference = match self.side {
            None => 0.0,
            Some(Side::Left) => 0.125,
            Some(Side::Right) => -0.125,
        };

        // drawing part

        // clear the screen
        clear([0.5, 0.5, 0.5, 1.0], g);

        // draw left rectangle
        rectangle(
            [0.5 + left_color_difference, 0.0, 0.0, 1.0],
            [padding, side_top_padding, side_width, side_height],
            c.transform,
            g,
        );

        // draw right rectangle
        rectangle(
            [0.5 - left_color_difference, 0.0, 0.0, 1.0],
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
```

You may notice, that we didn't draw any text. It's because drawing text is difficult. First of all, to do it, we need a font. Let's go, and download FiraSans-Regular font from Mozilla. Put it to `assets` folder, and don't forget to put its license nearby.

To actually load the font we'll need to add another dependency to our crate:

```toml
find_folder = "*"
```

Let's load the font in the `main.rs`:

```rust
extern crate find_folder;
```

And in the `main` function:

```rust
let assets = find_folder::Search::ParentsThenKids(3, 3)
    .for_folder("assets")
    .unwrap();

let font = &assets.join("FiraSans-Regular.ttf");
let factory = window.factory.clone();
let texture_settings = TextureSettings::new();

let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();
```

After we have glyphs, we can pass them to the `render` method, and draw the text. In `main.rs` change the call:

```rust
window.draw_2d(&e, |c, g| app.view().render(c, g, &mut glyphs))
```

And in `view.rs` add an argument to `render` method:

```rust
    pub fn render(&mut self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
```

And following lines to actually draw the text:

```rust
text::Text::new(font_size).draw(
    &self.text,
    glyphs,
    &c.draw_state,
    c.transform.trans(padding, (font_size as f64) + padding),
    g,
);
```

Well, the view part looks completed. Let's add the logic for switching the states.

Event loop will provide us with `Update` events that contain the time passed. We can use them to change the states that include counting of time: `Preparing` and `Running`. So, let's add `update` method to `App`'s implementation.

```rust
pub fn update(&mut self, dt: f64) {
    match self.game_state {
        GameState::Preparing { time_to_start } => {
            let time_to_start = time_to_start - dt;

            if time_to_start < 0.0 {
                self.game_state = GameState::Running {
                    elapsed_time: 0.0,
                    // we'll add randomness later
                    side: Side::Left,
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
```

And in the `main.rs` we add to event loop the call to this method:

```rust
e.update(|args| app.update(args.dt));
```

But it will not change the behavior of the program as we need somehow to go to the state that depends on time. And `Init` isn't one. So, let's process keypresses. First of all, import `Key` enum to `lib.rs`:

```rust
use piston_window::keyboard::Key;
```

Here is the relevant method for `App`. Note how we avoid moving `self` when creating a temporary tuple for `match`:

```rust
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
```

And here is the call to this method in the `main.rs` event loop:

```rust
e.press(|button| if let Button::Keyboard(key) = button {
    app.key(key)
});
```

The game is fully operational now!

Well, not quite. We always choose the left side. Let's add some randomness. In `Cargo.toml`:

```rust
rand = "*"
```

In `lib.rs`:

```rust
extern crate rand;
```

And to be able to use trait's method:

```rust
use rand::Rng;
```

Let's create our source of randomness once and store it in the `App` struct:

```rust
pub struct App {
    game_state: GameState,
    rng: Box<Rng>,
}
```

We need `Box` here because `rand::Rng` has unknown size.

Let's modify the `new` method of `App`:

```rust
pub fn new() -> Self {
    App {
        game_state: GameState::Init,
        rng: Box::new(rand::thread_rng()),
    }
}

```

And, finally, add randomness to the place where we switch to `Running` state:

```rust
self.game_state = GameState::Running {
    elapsed_time: 0.0,
    side: if self.rng.gen() {
        Side::Left
    } else {
        Side::Right
    },
}
```

Great! The game works. Let's do the final tweaking and prettify the time output. This way the number will be printed with two digits after the point.

```rust
format!("{:.*}", 2, time)
```

So, let's change all the relevant `format!`s:

```rust
format!("time to start: {:.*}", 2, time_to_start)
```

```rust
format!("elapsed time: {:.*}", 2, elapsed_time)
```

```rust
format!(
    "You {}! Elapsed time: {:.*}",
    if is_correct { "win" } else { "lose" },
    2,
    elapsed_time
)
```

We are done. The final code is available on [github](https://github.com/Menschenkindlein/left-right).
