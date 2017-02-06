extern crate sdl2_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate find_folder;
extern crate image;

mod models;
mod drawings;
mod game;

use sdl2_window::Sdl2Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{Button, Input};
use piston::window::WindowSettings;

use game::Game;

const OPEN_GL: OpenGL = OpenGL::V4_1;

// Returns a result containing a Sdl2Window or an error if the window
// settings are not supported
fn try_create_window(samples: u8) -> Result<Sdl2Window, String> {
    WindowSettings::new("Rust Rogue", [640, 640])
        .exit_on_esc(true)
        .opengl(OPEN_GL)
        .samples(samples)
        .vsync(true)
        .fullscreen(true)
        .build()
}

fn main() {
    // Create a window with a AA sampling of 8 or fall back to 0
    let mut window = try_create_window(8).or_else(|_| try_create_window(0)).unwrap();
    // window.sdl_context.mouse().set_relative_mouse_mode(true);

    let mut gl = GlGraphics::new(OPEN_GL);

    // Event handling
    let mut event_settings = EventSettings::new();
    event_settings.max_fps = 60;
    let mut events = Events::new(event_settings);

    // Game state
    let mut game = Game::new();

    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Press(Button::Keyboard(key)) => {
                game.key_press(key);
            }

            Input::Release(Button::Keyboard(key)) => {
                game.key_release(key);
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |c, g| game.render(c, g));
            }

            Input::Update(args) => {
                game.update(args);
            }

            _ => {}
        }
    }
}
