extern crate sdl2;

fn create_window() -> Result<sdl2::video::Window, String> {
    let ctx = try!(sdl2::init());
    let video_ctx = try!(ctx.video());
    let window = video_ctx.window("Scape", 640, 480).position_centered().opengl().build();
    
    return window.map_err(|e| e.to_string());
}

fn main() {
    create_window().unwrap().show();
}