extern crate sdl2;
use std::thread;
use std::time::Duration;
use sdl2::messagebox;
use sdl2::messagebox::MessageBoxFlag;
use sdl2::event::Event;

fn _main() -> Result<(), String> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let mut window = try!(video_ctx.window("Scape", 640, 480).position_centered().opengl().build().map_err(|e| e.to_string()));
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
        
        thread::sleep(Duration::from_millis(100));
    }
    
    Ok(())
}

fn main() {
    if let Err(e) = _main() {
        let _ = messagebox::show_simple_message_box(MessageBoxFlag::empty(), "Error", &e, Option::None); 
    }
}