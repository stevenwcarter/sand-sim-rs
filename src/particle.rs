use std::{thread, time::Duration};

use hashbrown::HashMap;
use opengl_graphics::GlGraphics;
use piston_window::*;
use rand::Rng;

use crate::config::CONFIG;

#[derive(Default)]
pub struct ParticleSet {
    pub particles: Vec<Particle>,
    pub settled: Vec<Particle>,
    pub peaks: HashMap<u32, u32>,
}

impl ParticleSet {
    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.push(Particle::new(x, y));
    }
    pub fn draw(&mut self, c: Context, g: &mut GlGraphics) {
        self.peaks.iter().for_each(|(x, y)| {
            rectangle(
                [0.5, 0.5, 0.5, 1.0],
                [
                    *x as f64,
                    ((CONFIG.height + *y) as f64 / 2.0),
                    1.0,
                    (CONFIG.height - *y) as f64,
                ],
                c.transform,
                g,
            );
        });
        self.settled.iter().for_each(|p| {
            p.draw(c, g);
        });
        self.particles.iter().for_each(|p| {
            p.draw(c, g);
        });
    }
    pub fn update(&mut self) {
        if self.particles.is_empty() {
            thread::sleep(Duration::from_millis(100));
        } else {
            let heights = self.peaks.clone();
            let mut particles_to_remove = Vec::<Particle>::new();
            self.particles
                .iter_mut()
                .filter(|p| !p.settled)
                .for_each(|p| {
                    p.update(&heights);
                    if p.settled {
                        particles_to_remove.push(*p);
                        self.settled.push(*p);
                    }
                });

            self.peaks = get_max_heights(Some(self.peaks.clone()), &self.settled);

            self.particles.retain(|p| !particles_to_remove.contains(p));
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
        // let color = random_color();
        let color = [0.5, 0.5, 0.5, 1.0];
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

        let this_col_height = *heights.get(&self.x).unwrap_or(&CONFIG.height);
        let left_col_height = *heights.get(&(self.x - 1)).unwrap_or(&CONFIG.height);
        let right_col_height = *heights.get(&(self.x + 1)).unwrap_or(&CONFIG.height);

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

fn get_max_heights(
    peaks: Option<HashMap<u32, u32>>,
    settled_particles: &[Particle],
) -> HashMap<u32, u32> {
    let mut heights: HashMap<u32, u32> = peaks.unwrap_or_default();
    (0..CONFIG.width).for_each(|x| {
        let max = settled_particles
            .iter()
            .filter(|p| p.x == x)
            .map(|p| p.y)
            .min()
            .unwrap_or(CONFIG.height);

        let cur_height = *heights.get(&x).unwrap_or(&CONFIG.height);
        heights.insert(x, max.min(cur_height));
    });

    heights
}
