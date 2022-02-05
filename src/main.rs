/* Minimal example to showcase the Rust SDL2 bindings working with an emscripten target (asmjs,wasm32).
 * Install one or both of the following Rust target triples:
 *   rustup target add wasm32-unknown-emscripten
 *   rustup target add asmjs-unknown-emscripten
 *
 * Build:
 *   source emsdk/emsdk_env.sh 
 *   cd src/
 *   em++ -c gxx_personality_v0_stub.cpp
 *   cargo build --target=asmjs-unknown-emscripten --release
 *   cargo build --target=wasm32-unknown-emscripten --release
 *
 * Start a web server and run the example:
 *   emrun index-asmjs.html 
 *   (or emrun index-wasm.html)
 */
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::thread;

fn main() -> Result<(), String> {

    #[cfg(target_os = "emscripten")]
    let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?; 
    let (width,height) = (640,480);
    let window = video_subsystem
        .window("wasm_sdl_threads", width, height)
        .position_centered() .resizable() .build() .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop: loop {
        let event = event_pump.wait_event(); //blocking wait for events
        
        canvas.set_draw_color(Color::RGBA(230,230,230,255));
        canvas.clear();
                
        match event {
            Event::KeyDown {keycode: Some(Keycode::Escape),..} | Event::Quit { .. } 
                => { break 'mainloop; }
            Event::MouseMotion {x, y, .. } => {
                //draw a red line from the upper left corner to the current mouse position
                canvas.set_draw_color(Color::RGBA(255,0,0,255));
                canvas.draw_line(Point::new(0,0), Point::new(x,y)).unwrap();
                ()}
            _ => {
                println!("Event: {:?}",event); //Print out other events to the "console"
                ()}
        }
        canvas.present();
    };
    
    Ok(())
}
