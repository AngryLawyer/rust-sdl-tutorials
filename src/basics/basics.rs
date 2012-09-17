use sdl;

struct Engine {
    running:bool 
}

impl Engine {
    fn on_execute() -> int {
        if (self.on_init() == false) {
            return -1;
        }

        while (self.running) {

            self.on_loop();
            self.on_render();
        }

        self.on_cleanup();
        return 0;
    }

    fn on_init() -> bool {
        return false;
    }

    fn on_loop() {
    }

    fn on_render() {
    }

    fn on_cleanup() {
    }
}

fn Engine() -> Engine {
    return Engine {
        running: true 
    };
}

fn main() {
    let engine = Engine();
    engine.on_execute();
}
