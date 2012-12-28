extern mod std;
extern mod sdl;

struct Engine {
    surface: ~sdl::video::Surface,
    image: ~sdl::video::Surface,

    mut running: bool,
}

impl Engine {
    fn on_execute() {
        
        match self.generate_tile_map(self.image) {
            result::Ok(tilemap) => {
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
                    self.on_render(tilemap);
                }
            },
            result::Err(message) => {
                io::println(message);
            }
        }
    }

    /*
     * Handles game logic, which is nothing at the moment
     */
    fn on_loop() {
    }

    fn on_render(tilemap: &sdl::video::Surface) {
        self.surface.blit_surface(tilemap);
        self.surface.flip();
    }

    fn generate_tile_map(source: &sdl::video::Surface) -> result::Result<~sdl::video::Surface, ~str> {
        match sdl::video::create_rgb_surface(~[sdl::video::HWSurface], 640, 480, 32, 0, 0, 0, 0) {
            result::Ok(plotter) => {
                let max_width = 640 / 16;
                let max_height = 480 / 16;

                let mut x = 0;

                let rng = rand::Rng();

                while (x < max_width) {

                    let mut y = 0;

                    while (y < max_height) {
                        let source_rect = &{
                            x: (rng.gen_int_range(0, 256/16) as i16) * 16,
                            y: (rng.gen_int_range(0, 128/16) as i16) * 16,
                            w: 16,
                            h: 16,
                        };
                        let dest_rect = &{
                            x: x * 16,
                            y: y * 16,
                            w: 16,
                            h: 16,
                        };

                        plotter.blit_surface_rect(source, source_rect, dest_rect);
                        y += 1;
                    }
                    x += 1;
                }

                result::Ok(plotter)
            },
            result::Err(err) => {
                result::Err(err)
            }
        }
    }
}

/*
 * Construct our Engine, and fill it with the surface we're going to need
 */
fn Engine() -> result::Result<Engine, ~str> {

    if sdl::sdl::init(~[sdl::sdl::InitVideo]) == false {
        return result::Err(fmt!("Unable to initialize SDL: %s", sdl::sdl::get_error()));
    }

    let maybe_surface = sdl::video::set_video_mode(640, 480, 32, ~[sdl::video::HWSurface], ~[sdl::video::DoubleBuf]);

    match maybe_surface {
        result::Ok(surface) => {

            let maybe_image = load_image(~"tileset.bmp");

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
