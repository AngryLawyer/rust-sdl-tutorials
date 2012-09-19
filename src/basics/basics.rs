use sdl;

struct Engine {
    mut running: bool,
    mut surface: *sdl::video::Surface
}

impl Engine {
    fn on_execute() -> int {

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

        self.on_cleanup();
        return 0;
    }

    /*
     * Handles game logic, which is nothing at the moment
     */
    fn on_loop() {
    }

    /*
     * Handles rendering logic, which is also nothing!
     */
    fn on_render() {
    }

    /*
     * It's always good practice to release all the things we've aquired
     * before quitting out
     */
    fn on_cleanup() {
        sdl::video::free_surface(self.surface);
        sdl::quit();
    }
}

/*
 * Construct our Engine, and fill it with the surface we're going to need
 */
fn Engine() -> result::Result<Engine, ~str> {

    if sdl::init(~[sdl::InitVideo]) < 0 {
        return result::Err(#fmt("Unable to initialize SDL: %s", sdl::get_error()));
    }

    let surface = sdl::video::set_video_mode(640, 480, 31, ~[sdl::video::HWSurface], ~[sdl::video::DoubleBuf]);

    if surface == ptr::null() {
        return result::Err(#fmt("Unable to create surface: %s", sdl::get_error()));
    }

    return result::Ok(Engine {
        running: true,
        surface: surface
    });
}

fn main() {
    let maybe_engine = Engine();
    if result::is_ok(maybe_engine) {
        result::unwrap(maybe_engine).on_execute();
    } else {
        io::println(result::unwrap_err(maybe_engine));
    }
}
