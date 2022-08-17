


//#[path = "frag_shader.rs"] mod frag_shader;
//use crate::frag_shader::frag::{PixelData};
// TODO: do something about this crate::frag_shader::frag::

pub mod gol {
  use rand::Rng;

  type Pattern<'a> = (usize, usize, &'a[u8]);


  const GLIDER_DATA : [u8; 9] = [0, 1, 0,
                                 0, 0, 1,
                                 1, 1, 1];

  const GLIDER_PATTERN : Pattern = (3, 3, &GLIDER_DATA);

  const EMPTY : u32 = 0xffffffff;
  const FULL : u32 = 0;

   fn set_value(data : &mut [u8], index : usize, bpp : usize, value : u32) {
    for i in 0..bpp {
      let mask = 0xff << i * 8;
      data[index + i] = ((value & mask) >> (i * 8)) as u8;
    }
  }

  fn place_pattern(x : usize, y : usize, width : usize, height : usize, data : &mut [u8], pitch : usize, pattern : Pattern) {
    let bpp = pitch / width;
    for dy in 0..pattern.0 {
      for dx in 0..pattern.1 {
        let xx = (x + dx) % width;
        let yy = (y + dy) % height;
    
        let index = yy * pitch + xx * bpp;
        if pattern.2[dy * pattern.0 + dx] > 0 {
          set_value(data, index, bpp, FULL);
        }
      }
    }
  }

  pub fn game_of_life_init(width : usize, height : usize, data : &mut [u8], pitch : usize) {
    let bpp = pitch / width;
    for y in 0..height {
      for x in 0..width {
        let index = y * pitch + x * bpp;
        set_value(data, index, bpp, EMPTY);
      }
    }

    let mut rng = rand::thread_rng();
    let cnt = rng.gen_range(0..1000);
    for _i in 0..cnt {
      let px = rng.gen_range(0..width);
      let py = rng.gen_range(0..height);
      place_pattern(px, py, width, height, data, pitch, GLIDER_PATTERN);
    }
  }

  fn count_neighbours(data : &crate::frag_shader::frag::PixelData) -> u32 {
    let x = data.x;
    let y = data.y;
    let px_index = data.index();
    let mut result : u32 = 0;
    for dy in 0..3 {
      for dx in 0..3 {
        let xx = x + dx;
        let yy = y + dy;

        // if xx == 0 {
        //   xx = data.width;
        // }

        // if yy == 0 {
        //   yy = data.height;
        // }
  
        //let index = data.to_index_safe(xx - 1, yy - 1);
        let index = data.to_index(xx - 1, yy - 1);
        let val = data.value(index);
        if (index != px_index) && (val == FULL) {
          result += 1;
        }
      }
    }
    result
  }

  pub fn game_of_life_step(data : &crate::frag_shader::frag::PixelData) -> u32 {
    let index = data.index();
    let neighbours = count_neighbours(data);
    if (neighbours == 2) && (data.value(index) == FULL) {
      return FULL;
    }
    else if neighbours == 3 {
      return FULL;
    }

    EMPTY
  }
}
