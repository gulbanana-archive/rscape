extern crate sdl2;
extern crate sdl2_ttf;
mod errors;

use std::thread;
use std::time::Duration;
use std::error::Error;
use std::path::Path;

use sdl2::messagebox;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2_ttf::Font;

use errors::ScapeError;

static SCREEN_WIDTH : u32 = 1280;
static SCREEN_HEIGHT : u32 = 720;

fn main() {
    if let Err(e) = _main() {
        let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR, "Error", e.description(), None); 
    }
}

fn _main() -> Result<(), ScapeError> {
    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let ttf_ctx = try!(sdl2_ttf::init().map_err(|e| e.to_string())); //because the actual error type is private  
    
    let window = try!(video_ctx.window("Scape", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build());
    let mut events = try!(sdl_ctx.event_pump());
    let mut renderer = try!(window.renderer().build());
    
    let arial128 = try!(ttf_ctx.load_font(Path::new("C:\\Windows\\Fonts\\arial.ttf"), 128));
    
    let mut should_quit = false;
    let mut i = 0;
    while !should_quit {
        for event in events.poll_iter() {
            should_quit = match event {
                Event::Quit {..} => true,
                _ => false
            };
        }
        
        if let Err(e) = frame(&mut renderer, &arial128, i) {
            let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR, "Error", e.description(), None); //should be Some but there's a library bug
            should_quit = true;
        }
        i += 1;
        
        thread::sleep(Duration::from_millis(100));
    }
    
    Ok(())
}

fn frame(renderer: &mut Renderer, font: &Font, i: i32) -> Result<(), ScapeError> {
    let surface = try!(font.render(format!("{}", i).as_str()).blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())); //more private error types :(
    let mut texture = try!(renderer.create_texture_from_surface(&surface));
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new((SCREEN_WIDTH / 2 - width / 2) as i32, 
                           (SCREEN_HEIGHT / 2 - height / 2) as i32,
                           width, 
                           height);

    renderer.set_draw_color(Color::RGBA(195, 217, 255, 255));
    renderer.clear();
    renderer.copy(&mut texture, None, Some(target));
    renderer.present();
    
    Ok(())
}
