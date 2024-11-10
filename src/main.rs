use opengl_graphics::GlGraphics;
use piston_window::*;

use sand::{config::CONFIG, App};

fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new("Sand Simulation", [CONFIG.width, CONFIG.height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(mousepos) = e.mouse_cursor_args() {
            app.update_mouse_pos(mousepos);
        }
        if let Some(Button::Mouse(key)) = e.press_args() {
            if key == MouseButton::Left {
                app.press_mouse();
            }
        }
        if let Some(Button::Mouse(key)) = e.release_args() {
            if key == MouseButton::Left {
                app.release_mouse();
            }
        }
    }
}
