


// pub struct Hexgrid<T> {
//   data : std::Vec<T>,
//   width: u32,
//   height: u32,
// }


pub fn neighbours(&self, i : u32, j : u32) -> [T; 6] {
  // TODO:
  const odd : i32 = i % 2;
  let mut n : [T; 6];
  const indexes : [(i32, i32); 6] = [
    (-1, odd - 1),
    (-1, odd),
    (0, -1),
    (0, 1),
    (1, odd - 1),
    (1, odd)
  ];

  for k in 0..6 {
    let pair = indexes[k];
    let y = i + pair.first;
    let x = j + pair.second;
    let index = y * self.width + x;
    n[k] = self.data[index];
  }

  return n;
}

