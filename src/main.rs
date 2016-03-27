extern crate sdl2;
use std::thread;
use std::time::Duration;
use sdl2::messagebox;
use sdl2::messagebox::MessageBoxFlag;
use sdl2::event::Event;
use sdl2::video::Window;

fn main() {
    if let Err(e) = _main() {
        let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR, "Error", &e, Option::None); 
    }
}

fn _main() -> Result<(), String> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let mut window = try!(video_ctx.window("Scape", 1280, 720).position_centered().opengl().build().map_err(|e| e.to_string()));
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
        
        if let Err(e) = frame(&window) {
            let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR, "Error", &e, Option::Some(&window)); 
            should_quit = true;
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    Ok(())
}

fn frame(window: &Window) -> Result<(), String> {
    Err("no game yet".into())
}