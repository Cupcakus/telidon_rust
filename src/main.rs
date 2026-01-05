extern crate sdl2;

mod screen;

use crate::screen::Screen;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::fs::File;
use std::io::Read;

//const BAUD_RATE: u32 = 1200;

fn tick_serial_emulator(_dt: Duration, file: &mut File) {
    //Even though we are just opening files from the OS filesystem, I'd like to process the file at old school
    //serial/modem speeds to emulate the experience of seeing the graphics draw to the screen.
    //I could just read the whole file into a buffer and process it slowly, but that's dumb. So I will instead
    //figure out how to have rust let me lock and read the file one byte at a time.
    let mut buffer = [0; 1];
    match file.read(&mut buffer).unwrap() {
        1 => { println!("{:#04x}", buffer[0]); }
        _ => { 
            //We could close the file here, but I like the idea of another process appending the file we have open
            //in the future to "stream" more data to us.
            }
    }
}

pub fn main() {
  
    let mut screen = Screen::new(800, 600, "Telidon Terminal");
    let mut file = File::open("hello.txt").unwrap();
    
    screen.clear((0, 255, 255));
    screen.present();

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        screen.clear((i, 64, 255 - i));
        for event in screen.poll_events() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...   
        tick_serial_emulator(Duration::new(0, 1_000_000_000u32 / 60), &mut file);
        screen.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}