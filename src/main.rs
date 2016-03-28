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
use sdl2::render::Texture;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;
use sdl2_ttf::Font;
use sdl2::event::WindowEventId;

use errors::ScapeError;

fn main() {
    if let Err(e) = _main() {
        let _ = messagebox::show_simple_message_box(messagebox::MESSAGEBOX_ERROR,
                                                    "Error",
                                                    e.description(),
                                                    None);
    }
}

fn _main() -> Result<(), ScapeError> {
    let mut screen_width: u32 = 1280;
    let mut screen_height: u32 = 720;

    let sdl_ctx = try!(sdl2::init());
    let video_ctx = try!(sdl_ctx.video());
    let ttf_ctx = try!(sdl2_ttf::init().map_err(|e| e.to_string())); //because the actual error type is private

    let window = try!(video_ctx.window("Scape", screen_width, screen_height)
                               .position_centered()
                               .resizable()
                               .opengl()
                               .build());
    let mut events = try!(sdl_ctx.event_pump());
    let mut renderer = try!(window.renderer().build());

    let arial = try!(ttf_ctx.load_font(Path::new("C:\\Windows\\Fonts\\arial.ttf"), 64));

    let mut should_quit = false;
    let mut i = 0;
    while !should_quit {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    should_quit = true;
                }
                Event::Window { win_event_id: WindowEventId::Resized, data1, data2, .. } => {
                    screen_width = data1 as u32;
                    screen_height = data2 as u32;
                }
                _ => (),
            };
        }

        // prep the screen
        renderer.set_draw_color(Color::RGBA(195, 217, 255, 255));
        renderer.clear();

        // draw a number to a texture
        let mut dot = try!(render_text(&mut renderer, &arial, i));
        let TextureQuery { width: glyph_width, height: glyph_height, .. } = dot.query();

        if (screen_width / glyph_width) * glyph_width < screen_width ||
           (screen_height / glyph_height) * glyph_height < screen_height {
            screen_width = (screen_width / glyph_width) * glyph_width;
            screen_height = (screen_height / glyph_height) * glyph_height;
            try!(renderer.window_mut().unwrap().set_size(screen_width, screen_height));
        }

        for x in 0..screen_width / glyph_width {
            for y in 0..screen_height / glyph_height {
                let target = Rect::new((x * glyph_width) as i32,
                                       (y * glyph_height) as i32,
                                       glyph_width,
                                       glyph_height);
                renderer.copy(&mut dot, None, Some(target));
            }
        }
        renderer.present();

        thread::sleep(Duration::from_millis(10));
        i += 1;
        if i > 9 {
            i = 0;
        }
    }

    Ok(())
}

fn render_text(renderer: &mut Renderer, font: &Font, i: i32) -> Result<Texture, ScapeError> {
    let surface = try!(font.render(format!("{}", i).as_str())
                           .blended(Color::RGBA(255, 0, 0, 255))
                           .map_err(|e| e.to_string())); //more private error types :(
    let texture = try!(renderer.create_texture_from_surface(&surface));

    Ok(texture)
}
