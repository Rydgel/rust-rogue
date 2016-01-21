use opengl_graphics::{GlGraphics, Texture};
use graphics::{Context, Transformed};
use graphics::default_draw_state;
use graphics::Image;
use find_folder;

pub struct Sprites {
    texture: Texture
}

impl Sprites {
    /// Loading sprite sheet from image.
    pub fn new() -> Sprites {
        let resources = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources")
            .unwrap();
        let sprite = resources.join("Scavengers_SpriteSheet.png");

        Sprites {
            texture: Texture::from_path(sprite).unwrap()
        }
    }

    /// Get the right slice of the sprite. Sprite is 8x8
    fn animated_sprites_coord(state: i32, start: i32) -> [i32; 4] {
        let x = (state + start) % 8;
        let y = (state + start) / 8;
        [x * 64, y * 64, 64, 64]
    }

    /// Get player sprite, with animation state
    pub fn draw_player(&self, x: f64, y: f64, animation_state: i32, c: &Context, gl: &mut GlGraphics) {
        Image::new()
            .src_rect(Sprites::animated_sprites_coord(animation_state, 0))
            .draw(&self.texture, default_draw_state(), c.transform.trans(x, y), gl);
    }

    /// Get Jon Skelington sprite, with animation state
    pub fn draw_skelington(&self, x: f64, y: f64, animation_state: i32, c: &Context, gl: &mut GlGraphics) {
        Image::new()
            .src_rect(Sprites::animated_sprites_coord(animation_state, 6))
            .draw(&self.texture, default_draw_state(), c.transform.trans(x, y), gl);
    }
}
