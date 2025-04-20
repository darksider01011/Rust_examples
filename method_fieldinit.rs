#[derive(Debug)]
struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
  fn area(&self) -> u32 {
     self.width * self.height
  }

  fn masa(&self) -> u32 {
     (self.width + self.height) * 2
  }
}

fn main() {
  fn build(width: u32, height: u32) -> Rect {
    Rect {
     width,
     height,}
      }
   println!("rect1 = {}", build(4,5).masa());
}
