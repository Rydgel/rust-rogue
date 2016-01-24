use piston::input::*;
use graphics::*;
use opengl_graphics::GlGraphics;
use itertools::Itertools;

use models::Player;
use models::Skelington;
use drawings::Sprites;

#[derive(Default)]
struct Timers {
    current_time: f64,
}

pub struct Game {
    rotation: f64,
    x: f64,
    y: f64,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    player: Player,
    skelingtons: Vec<Skelington>,
    timers: Timers,
    sprites: Sprites,
}

impl Game {
    pub fn new() -> Game {
        Game {
            rotation: 0.0,
            x: 300.0,
            y: 300.0,
            left: false,
            right: false,
            up: false,
            down: false,
            player: Player::spawn(),
            skelingtons: vec![Skelington::spawn(), Skelington::spawn()],
            timers: Timers::default(),
            sprites: Sprites::new(),
        }
    }

    pub fn update(&mut self, upd: UpdateArgs) {
        self.timers.current_time += upd.dt;
        self.rotation += 15.0 * upd.dt;
        // animations
        self.player.update_animation_state(self.timers.current_time);
        for s in &mut self.skelingtons {
            s.update_animation_state(self.timers.current_time);
        }
        self.move_player(upd.dt);
    }

    pub fn key_press(&mut self, k: Key) {
        match k {
            Key::Up => self.up = true,
            Key::Down => self.down = true,
            Key::Right => self.right = true,
            Key::Left => self.left = true,
            _ => {}
        }
    }

    pub fn key_release(&mut self, k: Key) {
        match k {
            Key::Up => self.up = false,
            Key::Down => self.down = false,
            Key::Right => self.right = false,
            Key::Left => self.left = false,
            _ => {}
        }
    }

    fn move_player(&mut self, dt: f64) {
        if self.up {
            self.y -= 450.0 * dt;
        }
        if self.down {
            self.y += 450.0 * dt
        }
        if self.right {
            self.x += 450.0 * dt;
        }
        if self.left {
            self.x -= 450.0 * dt;
        }
    }

    pub fn render(&mut self, c: Context, g: &mut GlGraphics) {
        clear([0.0, 0.0, 0.0, 1.0], g);
        let center = c.transform.trans(self.x, self.y);
        let square = rectangle::square(0.0, 0.0, 100.0);
        let red = [1.0, 0.0, 0.0, 1.0];
        rectangle(red,
                  square,
                  center.rot_rad(self.rotation).trans(-50.0, -50.0),
                  g);
        // Draw player
        self.player.draw(&c, g, &self.sprites);
        // Draw skelingtons
        self.skelingtons.iter().foreach(|s| s.draw(&c, g, &self.sprites));
    }
}
