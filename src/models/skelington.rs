use rand::{thread_rng, Rng};
use graphics::{Context};
use opengl_graphics::{GlGraphics};

use drawings::Point;
use drawings::Sprites;


pub struct Skelington {
    coordinates: Point,
    animation_state: i32, // there is 6 animation frames.
    life: u32,
}

impl Skelington {
    /// Spawn the player in a random location
    pub fn spawn() -> Skelington {
        let mut rng = thread_rng();
        let x = rng.gen_range(1, 9) as f64 * 64.0;
        let y = rng.gen_range(1, 9) as f64 * 64.0;

        Skelington {
            coordinates: Point::new(x, y),
            animation_state: 0,
            life: 100,
        }
    }

    /// Draw the player
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics, sprites: &Sprites) {
        let x = self.coordinates.x() + 50.0;
        let y = self.coordinates.y() + 50.0;
        sprites.draw_skelington(x, y, self.animation_state, c, gl);
    }

    pub fn update_animation_state(&mut self, state: f64) {
        self.animation_state = ((state / 0.1) as i32) % 6;
    }

    /// When the enemy takes some damage
    pub fn take_damage(&mut self, amount: u32) {
        self.life -= amount;
    }
}
