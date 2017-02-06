use rand::{thread_rng, Rng};
use graphics::Context;
use opengl_graphics::GlGraphics;

use drawings::Point;
use drawings::Sprites;
use drawings::STINKBAG_IDLE;


pub struct Stinkbag {
    coordinates: Point,
    animation_state: f64, // there is 6 animation frames.
    life: u32,
}

impl Stinkbag {
    /// Spawn the player in a random location
    pub fn spawn() -> Stinkbag {
        let mut rng = thread_rng();
        let x = rng.gen_range(1, 9) as f64 * 64.0;
        let y = rng.gen_range(1, 9) as f64 * 64.0;

        Stinkbag {
            coordinates: Point::new(x, y),
            animation_state: 0.0,
            life: 100,
        }
    }

    /// Draw the player
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics, sprites: &Sprites) {
        let x = self.coordinates.x();
        let y = self.coordinates.y();
        let (index, _) = STINKBAG_IDLE;
        sprites.draw_chars(x, y, index, self.animation_state, c, gl);
    }

    pub fn update_animation_state(&mut self, state: f64) {
        let (_, animation_length) = STINKBAG_IDLE;
        self.animation_state = (((state / 0.1) as i32) as f64) % animation_length;
    }

    /// When the enemy takes some damage
    pub fn take_damage(&mut self, amount: u32) {
        self.life -= amount;
    }
}
