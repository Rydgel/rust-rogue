extern crate glutin_window;
extern crate graphics;
extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod game;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventLoop};
use piston::input::{Button, Event, Input, RenderEvent};
use piston::window::WindowSettings;

use game::Game;

const OPEN_GL: OpenGL = OpenGL::V3_2;

// Returns a result containing a GlutinWindow or an error if the window
// settings are not supported
fn try_create_window(samples: u8) -> Result<GlutinWindow, String> {
    WindowSettings::new("Rust Rogue", [1024, 1024])
        .exit_on_esc(true)
        .opengl(OPEN_GL)
        .samples(samples)
        .build()
}

fn main() {
    // Create a window with a sampling of 8 or fall back to 0
    let mut window = try_create_window(8).or_else(|_| try_create_window(0)).unwrap();

    let mut gl = GlGraphics::new(OPEN_GL);

    // Event handling
    let mut events = window.events().ups(60).max_fps(60);

    // Game state
    let mut game = Game::new();

    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                game.key_press(key);
            }

            Event::Input(Input::Release(Button::Keyboard(key))) => {
                game.key_release(key);
            }

            Event::Render(args) => {
                gl.draw(args.viewport(), |c, g| game.render(c, g));
            }

            Event::Update(args) => {
                game.update(args);
            }

            _ => {}
        }
    }
}
