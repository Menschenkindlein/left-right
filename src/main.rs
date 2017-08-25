extern crate piston_window;
extern crate left_right;
extern crate find_folder;

use piston_window::*;

use left_right::App;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Left/Right", [512; 2]).build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let font = &assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let texture_settings = TextureSettings::new();

    let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();

    let mut app = App::new();

    while let Some(e) = window.next() {
        e.update(|args| app.update(args.dt));
        e.press(|button| if let Button::Keyboard(key) = button {
            app.key(key)
        });
        window.draw_2d(&e, |c, g| app.view().render(c, g, &mut glyphs));
    }
}
