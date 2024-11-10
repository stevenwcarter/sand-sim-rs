use std::cmp::{max, min};

use config::CONFIG;
use opengl_graphics::GlGraphics;
use particle::ParticleSet;
use piston_window::*;
use rand::Rng;

pub mod config;
mod particle;

pub struct App {
    pub gl: GlGraphics,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_pressed: bool,
    pub particle_set: ParticleSet,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let particle_set = ParticleSet::default();

        Self {
            particle_set,
            gl,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_pressed: false,
        }
    }
}

impl App {
    pub fn render(&mut self, e: &RenderArgs) {
        self.gl.draw(e.viewport(), |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            self.particle_set.draw(c, g);
            // self.ballset.balls().iter().for_each(|b| b.draw(c, g));
        });

        if self.mouse_pressed {
            let lower_limit_x = (self.mouse_x - 10.0).max(0.0);
            let upper_limit_x = (self.mouse_x + 10.0).min(CONFIG.width as f64);
            let lower_limit_y = (self.mouse_y - 10.0).max(0.0);
            let upper_limit_y = (self.mouse_y + 10.0).min(CONFIG.height as f64);

            let mut rng = rand::thread_rng();
            (0..CONFIG.particle_rate).for_each(|_| {
                let x = rng.gen_range(lower_limit_x..upper_limit_x);
                let y = rng.gen_range(lower_limit_y..upper_limit_y);
                self.particle_set.add_particle(x, y);
            })
        }
        self.particle_set.update();
        // self.ballset.update_loop();
    }
    pub fn update_mouse_pos(&mut self, mouse_pos: [f64; 2]) {
        self.mouse_x = mouse_pos[0];
        self.mouse_y = mouse_pos[1];
    }
    pub fn press_mouse(&mut self) {
        log::debug!("Pressing mouse");
        self.mouse_pressed = true;
    }
    pub fn release_mouse(&mut self) {
        log::debug!("Releasing mouse");
        self.mouse_pressed = false;
    }
    pub fn report(&self) {
        log::debug!("Particle count: {}", self.particle_set.particles.len());
        log::debug!("Peaks count: {}", self.particle_set.peaks.len());
    }
}
