extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;

mod app;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Left/Right", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut app = app::App::new(&window, assets, Box::new(rand::thread_rng()));

    while let Some(e) = window.next() {
        match e {
            Input::Update(args) => app.update(args.dt),
            Input::Release(button) => match button {
                Button::Keyboard(key) => app.key(key),
                _ => (),
            },
            Input::Render(args) => {
                window.draw_2d(&e, |c, g| { app.render(c, g, args.width, args.height); });
            }
            _ => (),
        }
    }
}
