mod gameboard;
mod button;

use gameboard::{Gameboard, GameboardController, GameboardView, GameboardViewSettigs};
use button::{Button, ButtonController, ButtonView, ButtonViewSettings};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Sudoku", [512 + 128, 512])
        .graphics_api(opengl)
        .resizable(false)
        .build()
        .expect("Couldn't create window!");

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/UbuntuMono.ttf", (), texture_settings)
        .expect("Couldn't load font!");

    let gameboard_view = GameboardView::new(GameboardViewSettigs::default());
    let mut gameboard_controller = GameboardController::new(Gameboard::new(), gameboard_view);

    let button_view = ButtonView::new(ButtonViewSettings::new([498.0, 241.0], [100.0, 30.0]));
    let mut button_controller = ButtonController::new(Button::new("Check".into()), button_view);

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |ref c, g| {
                use graphics::*;

                clear([1.0; 4], g);
                gameboard_controller.draw(c, g, glyphs);
                button_controller.draw(c, g, glyphs);
            });
        }

        gameboard_controller.handle_event(&event);
        if button_controller.handle_event(&event) {
            println!("Check: {}", gameboard_controller.check());
        }
    }
}
