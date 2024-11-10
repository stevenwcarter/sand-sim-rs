use std::{ops::Sub, thread, time::Duration};

use hashbrown::HashMap;
use opengl_graphics::GlGraphics;
use piston_window::*;
use rand::Rng;

use crate::config::CONFIG;

#[derive(Default)]
pub struct ParticleSet {
    particles: Vec<Particle>,
    settled: Vec<Particle>,
}

impl ParticleSet {
    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.push(Particle::new(x, y));
    }
    pub fn draw(&mut self, c: Context, g: &mut GlGraphics) {
        self.particles.iter().for_each(|p| {
            p.draw(c, g);
        });
    }
    pub fn update(&mut self) {
        let settled: Vec<Particle> = self
            .particles
            .iter()
            .filter(|p| p.settled)
            .copied()
            .collect();
        if settled.len() == self.particles.len() {
            thread::sleep(Duration::from_millis(100));
        } else {
            let heights = get_max_heights(&settled);
            self.particles
                .iter_mut()
                .filter(|p| !p.settled)
                .for_each(|p| {
                    p.update(&heights);
                });
        }
    }
}

fn random_color() -> types::Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..1.0);
    let g = rng.gen_range(0.0..1.0);
    let b = rng.gen_range(0.0..1.0);

    [r, g, b, 1.0]
}

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub color: types::Color,
    pub settled: bool,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        let color = random_color();
        let radius = 1.0;
        let vx: f64 = 0.0;
        let vy: f64 = 0.0;

        Self {
            x: x as u32,
            y: y as u32,
            vx,
            vy,
            radius,
            color,
            settled: false,
        }
    }
    pub fn draw(&self, c: Context, g: &mut GlGraphics) {
        ellipse(
            self.color,
            graphics::rectangle::square(self.x as f64, self.y as f64, self.radius),
            c.transform,
            g,
        );
    }
    pub fn update(&mut self, heights: &HashMap<u32, u32>) {
        if self.settled {
            return;
        }

        let new_x = self.x as f64 + self.vx;
        let new_y = self.y as f64 + self.vy;
        self.x = new_x as u32;
        self.y = new_y as u32;

        let this_col_height = *heights.get(&self.x).unwrap();
        let left_col_height = *heights.get(&(self.x - 1)).unwrap();
        let right_col_height = *heights.get(&(self.x + 1)).unwrap();

        if self.y > this_col_height {
            self.y = this_col_height - 1;
            self.vx = 0.0;
            self.vy = 0.0;
        } else if self.vy > 0.0 {
            self.vy *= 1.0 - CONFIG.air_resistance;
            self.vx *= 1.0 - CONFIG.air_resistance;
            self.vy += 1.0;
        } else if left_col_height > (self.y + 1) && self.x > 0 {
            self.x -= 1;
            self.vy += 1.0;
        } else if right_col_height > self.y + 1 {
            self.x += 1;
            self.vy += 1.0;
        } else {
            self.settled = true;
        }
    }
}

fn get_max_heights(settled_particles: &[Particle]) -> HashMap<u32, u32> {
    let mut heights: HashMap<u32, u32> = HashMap::new();
    (0..CONFIG.width).for_each(|x| {
        let max = settled_particles
            .iter()
            .filter(|p| p.x == x)
            .map(|p| p.y)
            .min()
            .unwrap_or(CONFIG.height);
        heights.insert(x, max);
    });

    heights
}
