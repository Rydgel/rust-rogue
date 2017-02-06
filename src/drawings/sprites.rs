use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use graphics::{Context, Transformed};
use graphics::draw_state::DrawState;
use graphics::Image;
use image::{self, DynamicImage};
use find_folder;
use graphics::color::gamma_srgb_to_linear;
use image::Pixel;
use image::Rgba;


pub const PLAYER_IDLE: (f64, f64) = (0.0, 6.0);
pub const SKELINGTON_IDLE: (f64, f64) = (6.0, 6.0);
pub const STINKBAG_IDLE: (f64, f64) = (12.0, 6.0);
pub const BOTTLE_FOOD: f64 = 18.0;
pub const TOMATO_FOOD: f64 = 19.0;
pub const EXIT_TILE: f64 = 20.0;
pub const WALL_TILE: [f64; 11] = [21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0];
pub const FLOOR_TILE: [f64; 8] = [32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0];
pub const PLAYER_ATTACK: (f64, f64) = (40.0, 2.0);
pub const SKELINGTON_ATTACK: (f64, f64) = (42.0, 2.0);
pub const STINKBAG_ATTACK: (f64, f64) = (44.0, 2.0);
pub const PLAYER_HIT: (f64, f64) = (46.0, 2.0);
pub const DAMAGEABLE_TILES: [f64; 7] = [48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0];


pub struct Sprites {
    texture: Texture,
}

impl Sprites {
    /// Loading sprite sheet from image.
    pub fn new() -> Sprites {
        let img = Sprites::load_image("Scavengers_SpriteSheet.png");
        let new_img = Sprites::convert_image_from_srgb_to_linear(img);
        let texture = Sprites::create_texture_from_image(new_img);
        Sprites { texture: texture }
    }

    /// Get the right slice of the sprite. Sprite is 8x8
    fn animated_sprites_coord(state: f64, start: f64) -> [f64; 4] {
        let x = (state + start) % 8.0;
        let y = (((state + start) as i32) / 8) as f64;
        [x * 64.0, y * 64.0, 64.0, 64.0]
    }

    /// Draw an animated sprite with current state
    pub fn draw_chars(&self, x: f64, y: f64, i: f64, st: f64, c: &Context, gl: &mut GlGraphics) {
        let sprite_coord = Sprites::animated_sprites_coord(st, i);
        let transf = c.transform.trans(x, y);
        Image::new()
            .src_rect(sprite_coord)
            .draw(&self.texture, &DrawState::new_alpha(), transf, gl);
    }

    // private helpers

    fn load_image(path: &str) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
        let resources = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("resources")
            .unwrap();
        let sprite = resources.join(path);
        let img = image::open(&sprite).unwrap();
        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            x => x.to_rgba(),
        };
        img
    }

    fn convert_image_from_srgb_to_linear(img: image::ImageBuffer<Rgba<u8>, Vec<u8>>)
                                         -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut new_img = img.clone();

        for (x, y, pixel) in img.enumerate_pixels() {
            let (r, g, b, a) = pixel.channels4();
            let r = r as f32 / 255.0;
            let g = g as f32 / 255.0;
            let b = b as f32 / 255.0;
            let a = a as f32 / 255.0;
            let new_color = gamma_srgb_to_linear([r, g, b, a]);
            let r = (new_color[0] * 255.0) as u8;
            let g = (new_color[1] * 255.0) as u8;
            let b = (new_color[2] * 255.0) as u8;
            let a = (new_color[3] * 255.0) as u8;
            let new_pixel = image::Pixel::from_channels(r, g, b, a);
            new_img.put_pixel(x, y, new_pixel);
        }

        new_img
    }

    fn create_texture_from_image(img: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> Texture {
        let mut texture_settings = TextureSettings::new();
        texture_settings.set_convert_gamma(true);
        texture_settings.set_compress(true);
        Texture::from_image(&img, &texture_settings)
    }
}
