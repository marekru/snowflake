
pub mod frag {
  // TODO: template trait
  pub trait FragmentCompute<FragmentData> {
    fn compute(&self, data : &FragmentData) -> u32;
  }

  pub struct PixelData<'a> {
    pub x : usize,
    pub y : usize,
    pub width : usize,
    pub height : usize,
    pub pitch: usize,
    pub bpp: usize,
    pub data: &'a dyn std::ops::Index<usize, Output = u8>
  }

  pub struct PixelShader {
    pub _compute : fn(&PixelData) -> u32
  }

  impl FragmentCompute<PixelData<'_>> for PixelShader {
    fn compute(&self, data : &PixelData) -> u32 {
      return (self._compute)(data);
    }
  }

  impl PixelData<'_> {
    // TODO: fn new() -> PixelShader


    // TODO: these methods shouldnt be here!!!!!!

    pub fn to_index(&self, x : usize, y : usize) -> usize {
      y * self.pitch + x * self.bpp
    }

    pub fn to_index_safe(&self, x : usize, y : usize) -> usize {
      self.to_index(x % self.width, y % self.height)
    }

    pub fn index(&self) -> usize {
      self.to_index(self.x, self.y)
    }

    pub fn value(&self, index : usize) -> u32 {
      let mut result : u32 = 0;
      for i in 0..self.bpp {
        let shift = i * 8;
        result |= (self.data[index + i] as u32) << shift;
      }
      result
    }
  }

}