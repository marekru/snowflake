

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;

mod frag_shader;
mod game_of_life;
mod hexgrid;

use frag_shader::frag::{PixelData, PixelShader};
use game_of_life::gol;

use crate::frag_shader::frag::FragmentCompute;

fn main() -> Result<(), String> {

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  /* Create the window and renderer */
  const WIDTH : u32 = 400;
  const HEIGHT : u32 = 400;
  const PIXEL_FORMAT : PixelFormatEnum = PixelFormatEnum::RGB24;
  const BPP : usize = 3; // TODO: from pixel format

  let window = video_subsystem
        .window("snowflake", WIDTH, HEIGHT)
        .position_centered()
        //.opengl()
        .build()
        .unwrap();

  let mut canvas = window
          .into_canvas()
          .build()
          .map_err(|e| e.to_string())?;

  let texture_creator = canvas.texture_creator();
  
  let mut texture = texture_creator
            .create_texture_streaming(PIXEL_FORMAT, WIDTH, HEIGHT)
            .map_err(|e| e.to_string())?;

  // let mut texture_swap = texture_creator
  //         .create_texture_streaming(PIXEl_FORMAT, WIDTH, HEIGHT)
  //         .map_err(|e| e.to_string())?;


  let frag = PixelShader{ _compute : gol::game_of_life_step };

  // TODO: shader init
  texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
    let w = WIDTH as usize;
    let h = HEIGHT as usize;
    gol::game_of_life_init(w, h, buffer, pitch);
  })?;

  let mut event_pump = sdl_context.event_pump()?;

  canvas.clear();
  canvas.copy(&texture, None, Some(Rect::new(0, 0, WIDTH, HEIGHT)))?;
  canvas.present();

  let mut time : usize = 0;
  'running: loop {
      for event in event_pump.poll_iter() {
          match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => break 'running,
              _ => {}
          }
      }

    let pixels = canvas.read_pixels(None, PIXEL_FORMAT).unwrap();
    
    let update_texture = |buffer: &mut [u8], pitch: usize| {
      let w = WIDTH as usize;
      let h = HEIGHT as usize;
      // TODO: 0..h
      for y in 1..h-1 {
        for x in 1..w-1 {
          let offset = y * pitch + x * BPP;

          let pixel = PixelData {
            x : x,
            y : y,
            width: w,
            height: h,
            pitch: pitch,
            bpp: BPP,
            data: &pixels
          };

          let color = frag.compute(&pixel);

          // TODO: method in utils
          for i in 0..pixel.bpp {
            let mask = 0xff << i * 8;
            buffer[offset + i] = ((color & mask) >> (i * 8)) as u8;
          }
        }
      }
    };

    texture.with_lock(None, update_texture);

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(0, 0, WIDTH, HEIGHT)))?;
    canvas.present();
    time += 1;
  }

  Ok(())

}
