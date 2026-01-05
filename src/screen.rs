//SDL2 implementation of a Telidon terminal screen.
extern crate sdl2;
    pub struct Screen {
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        event_pump: sdl2::EventPump
    }

    impl Screen {
        pub fn new(width: u32, height: u32, title: &str) -> Screen {
            let context = sdl2::init().unwrap();
            let video_subsystem = context.video().unwrap();
            let window = video_subsystem.window(title, width, height)
                .position_centered()
                .build()
                .unwrap();
            let canvas = window.into_canvas().build().unwrap();
            let event_pump = context.event_pump().unwrap();
            Screen { canvas, event_pump: event_pump }
        }

        pub fn present(&mut self) {
            self.canvas.present();
        }

        pub fn clear(&mut self, color: (u8, u8, u8)) {
            self.canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
            self.canvas.clear();
        }

        pub fn poll_events(&mut self) -> Vec<sdl2::event::Event> {
            self.event_pump.poll_iter().collect()
        }
    }

