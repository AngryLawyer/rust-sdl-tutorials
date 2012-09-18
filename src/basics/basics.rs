use sdl;

struct Engine {
    mut running: bool,
    mut surface: *sdl::video::Surface
}

impl Engine {
    fn on_execute() -> int {

        while (self.running) {
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

    fn on_loop() {
    }

    fn on_render() {
    }

    fn on_cleanup() {
        sdl::quit();
    }
}

fn Engine() -> option::Option<Engine> {
    //This guy does initialization these days
    //Try to create the surface
    if sdl::init(~[sdl::InitEverything]) < 0 {
        return option::None;
    }

    let surface = sdl::video::set_video_mode(640, 480, 31, ~[sdl::video::HWSurface], ~[sdl::video::DoubleBuf]);

    if surface == ptr::null() {
        return option::None;
    }

    return option::Some(Engine {
        running: true,
        surface: surface
    });
}

fn main() {
    let maybe_engine = Engine();
    if option::is_some(maybe_engine) {
        option::unwrap(maybe_engine).on_execute();
    } else {
        io::println("Error - couldn't initialize the Engine!");
    }
}
