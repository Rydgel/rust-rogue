use opengl_graphics::{GlGraphics, Texture};
use graphics::{Context, Transformed};
use graphics::default_draw_state;
use graphics::Image;
use find_folder;


pub struct Sprites {
    texture: Texture,
}

impl Sprites {
    /// Loading sprite sheet from image.
    pub fn new() -> Sprites {
        let resources = find_folder::Search::ParentsThenKids(3, 3)
                            .for_folder("resources")
                            .unwrap();
        let sprite = resources.join("Scavengers_SpriteSheet.png");
        Sprites { texture: Texture::from_path(sprite).unwrap() }
    }

    /// Get the right slice of the sprite. Sprite is 8x8
    fn animated_sprites_coord(state: i32, start: i32) -> [i32; 4] {
        let x = (state + start) % 8;
        let y = (state + start) / 8;
        [x * 64, y * 64, 64, 64]
    }

    /// Draw an animated sprite with current state
    pub fn draw_chars(&self, x: f64, y: f64, i: i32, st: i32, c: &Context, gl: &mut GlGraphics) {
        let sprite_coord = Sprites::animated_sprites_coord(st, i);
        let transf = c.transform.trans(x, y);
        Image::new()
            .src_rect(sprite_coord)
            .draw(&self.texture, default_draw_state(), transf, gl);
    }
}
