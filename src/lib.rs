use opengl_graphics::GlGraphics;
use particle::ParticleSet;
use piston_window::*;

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
            self.particle_set.add_particle(self.mouse_x, self.mouse_y);
            self.particle_set
                .add_particle(self.mouse_x + 1.0, self.mouse_y - 2.0);
            self.particle_set
                .add_particle(self.mouse_x - 2.0, self.mouse_y - 4.0);
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
}
