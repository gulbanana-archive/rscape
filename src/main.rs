extern crate sdl2;
use std::thread;
use std::time::Duration;

struct Context {
    sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    events: sdl2::EventPump,
}

fn init() -> Result<Context, String> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let window = try!(video_ctx.window("Scape", 640, 480).position_centered().opengl().build().map_err(|e| e.to_string()));
    let events = try!(sdl_ctx.event_pump());
    
    Ok(Context {sdl: sdl_ctx, window: window, events: events})
}

fn main() {
    let mut context = init().unwrap();
    context.window.show();
    
    let mut should_quit = false;
    while !should_quit {
        for event in context.events.poll_iter() {
            should_quit = match event {
                sdl2::event::Event::Quit {..} => true,
                _ => false
            };
        }
        
        thread::sleep(Duration::from_millis(100));
    }
}