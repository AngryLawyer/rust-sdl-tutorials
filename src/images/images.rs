extern mod std;
extern mod sdl;

struct Engine {
    surface: ~sdl::video::Surface,
    image: ~sdl::video::Surface,

    mut running: bool,
}

impl Engine {
    fn on_execute() {
        
        while (self.running) {
            //Handle the event poll 
            let mut polling = true;
            while polling {
                sdl::event::poll_event(|event| {
                    match event {
                        sdl::event::QuitEvent => { self.running = false; }
                        sdl::event::NoEvent => { polling = false; }
                        _ => {}
                    }
                });
            }
            self.on_loop();
            self.on_render();
        }
    }

    /*
     * Handles game logic, which is nothing at the moment
     */
    fn on_loop() {
    }

    fn on_render() {
        self.surface.blit_surface(self.image);
        self.surface.flip();
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

            let maybe_image = load_image(~"example.bmp");

            match maybe_image {
                result::Ok(image) => {
                    result::Ok(Engine {
                        running: true,
                        surface: surface,
                        image: image 
                    })
                },
                result::Err(message) => {
                    result::Err(fmt!("Unable to load bitmap: %s", message))
                }
            }

        },
        result::Err(message) => {
            result::Err(fmt!("Unable to create surface: %s", message))
        }
    }
}

fn load_image(filename: ~str) -> result::Result<~sdl::video::Surface, ~str> {
    match sdl::video::load_bmp(filename) {
        result::Ok(image) => {
            image.display_format()
        },
        result::Err(message) => {
            result::Err(message)
        }
    }
}

fn main() {
    match Engine() {
        result::Ok(engine) => engine.on_execute(),
        result::Err(message) => {
            io::println(message);
        }
    };
    sdl::sdl::quit();
}
