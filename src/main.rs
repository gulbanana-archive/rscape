extern crate sdl2;
extern crate sdl2_ttf;
mod errors;

use std::thread;
use std::time::Duration;
use std::error::Error;

use sdl2::messagebox;
use sdl2::event::Event;

use errors::ScapeError;

fn main() {
    if let Err(e) = _main() {
        let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR, "Error", e.description(), Option::None); 
    }
}

fn _main() -> Result<(), ScapeError> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let ttf_ctx = try!(sdl2_ttf::init().map_err(|e| e.to_string())); //because the actual error type is private
    
    let mut window = try!(video_ctx.window("Scape", 1280, 720).position_centered().opengl().build());
    let mut events = try!(sdl_ctx.event_pump());
    
    window.show();
    
    let mut should_quit = false;
    while !should_quit {
        for event in events.poll_iter() {
            should_quit = match event {
                Event::Quit {..} => true,
                _ => false
            };
        }
        
        if let Err(e) = frame() {
            let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR, "Error", e.description(), Option::Some(&window)); 
            should_quit = true;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    Ok(())
}

fn frame() -> Result<(), ScapeError> {
    Err("no game yet".into())
}