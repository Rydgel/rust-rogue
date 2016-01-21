use rand::{thread_rng, Rng};
use graphics::{Context};
use opengl_graphics::{GlGraphics};

use drawings::Sprites;


pub struct Skelington {
    x: u32,
    y: u32,
    animation_state: i32, // there is 6 animation frames.
    life: u32,
}

impl Skelington {
    /// Spawn the player in a random location
    pub fn spawn() -> Skelington {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(1, 9);
        let y: u32 = rng.gen_range(1, 9);

        Skelington {
            x: x,
            y: y,
            animation_state: 0,
            life: 100,
        }
    }

    /// Draw the player
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics, sprites: &Sprites) {
        let position_x = self.x as f64 * 64.0;
        let position_y = self.y as f64 * 64.0;
        sprites.draw_skelington(position_x + 50.0, position_y + 50.0, self.animation_state, c, gl);
    }

    pub fn update_animation_state(&mut self, state: f64) {
        self.animation_state = ((state / 0.1) as i32) % 6;
    }

    /// When the enemy takes some damage
    pub fn take_damage(&mut self, amount: u32) {
        self.life -= amount;
    }
}
