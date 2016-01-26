use opengl_graphics::{GlGraphics, Texture};
use graphics::{Context, Transformed};
use graphics::default_draw_state;
use graphics::Image;
use find_folder;


pub const PLAYER_IDLE: (i32,i32) = (0,6);
pub const SKELINGTON_IDLE: (i32,i32) = (6,6);
pub const STINKBAG_IDLE: (i32,i32) = (12,6);
pub const BOTTLE_FOOD: i32 = 18;
pub const TOMATO_FOOD: i32 = 19;
pub const EXIT_TILE: i32 = 20;
pub const WALL_TILE: [i32; 11] = [21,22,23,24,25,26,27,28,29,30,31];
pub const FLOOR_TILE: [i32; 8] = [32,33,34,35,36,37,38,39];
pub const PLAYER_ATTACK: (i32,i32) = (40,2);
pub const SKELINGTON_ATTACK: (i32,i32) = (42,2);
pub const STINKBAG_ATTACK: (i32,i32) = (44,2);
pub const PLAYER_HIT: (i32,i32) = (46,2);
pub const DAMAGEABLE_TILES: [i32; 7] = [48,49,50,51,52,53,54];


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
