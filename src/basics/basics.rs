extern mod std;
extern mod sdl;

use sdl;

struct Engine {
    mut running: bool,
    surface: ~sdl::video::Surface,
}

impl Engine {
    fn on_execute(&self) {
        
        while (self.running) {
            //Handle the event poll 
            let mut polling = true;
            while polling {
                match sdl::event::poll_event() {
                    sdl::event::QuitEvent => { self.running = false; }
                    sdl::event::NoEvent => { polling = false; }
                    _ => {}
                };
            }
        }
    }

    /*
     * Handles game logic, which is nothing at the moment
     */
    fn on_loop(&self) {
    }

    fn on_render(&self) {
    }
}

/*
 * Construct our Engine, and fill it with the surface we're going to need
 */
fn Engine() -> result::Result<Engine, ~str> {

    if sdl::sdl::init(~[sdl::sdl::InitVideo]) == false {
        return result::Err(fmt!("Unable to initialize SDL: %s", sdl::sdl::get_error()));
    }

    let maybe_surface = sdl::video::set_video_mode(640, 480, 31, ~[sdl::video::HWSurface], ~[sdl::video::DoubleBuf]);

    match maybe_surface {
        result::Ok(surface) => {
            result::Ok(Engine {
                running: true,
                surface: surface
            })
        },
        result::Err(message) => {
            result::Err(fmt!("Unable to create surface: %s", message))
        }
    }
}

fn main() {
    do sdl::start::start {
        match Engine() {
            result::Ok(engine) => {
                engine.on_execute();
                sdl::sdl::quit();
            },
            result::Err(message) => {
                io::println(message);
            }
        };
    }
}
