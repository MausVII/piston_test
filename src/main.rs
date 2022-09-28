use graphics::Graphics;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::{WindowSettings, RenderArgs, UpdateArgs, Events, EventSettings, RenderEvent, UpdateEvent};
use piston_window::PistonWindow;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

struct App {
    pub(crate) gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |context, gfx| {
            gfx.clear_color(BLACK);
        })
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}

fn run_loop(app: &mut App, wnd: &mut PistonWindow) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(wnd) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new("Piston Test", [800, 640])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    
    let gl = GlGraphics::new(opengl);
    let mut app = App { gl };

    run_loop(&mut app, &mut window);
}
