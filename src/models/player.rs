use rand::{thread_rng, Rng};
use graphics::{Context, Transformed};
use opengl_graphics::{GlGraphics, Texture};


pub struct Player {
    x: u32,
    y: u32,
    animation_state: u32, // there is 6 animation frames.
    life: u32,
    texture: Texture,
}

impl Player {
    /// Spawn the player in a random location
    pub fn spawn() -> Player {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(1, 9);
        let y: u32 = rng.gen_range(1, 9);

        // todo moves that in a drawing class
        // Textures
        let resources = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources").unwrap();

        let sprite = resources.join("Scavengers_SpriteSheet.png");
        // A texture to use with the image
        let texture = Texture::from_path(sprite).unwrap();

        Player {
            x: x,
            y: y,
            animation_state: 1,
            life: 100,
            texture: texture,
        }
    }

    /// Draw the player
    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        use graphics::*;
        let position_x = self.x as f64 * 64.0;
        let position_y = self.y as f64 * 64.0;
        let transform = c.transform.trans(position_x, position_y);
        Image::new()
            .src_rect([0, 0, 64, 64])
            .draw(&self.texture, default_draw_state(), transform, gl);
    }

    /// When the player get something to eat on the ground
    pub fn eat(&mut self) {
        self.life += 50
    }

    /// When the player takes some damage
    pub fn take_damage(&mut self, damage: u32) {
        self.life -= damage;
    }
}
