
pub trait Movable2D {
  fn move_right (&mut self) -> Self;
  fn move_left (&mut self) -> Self;
  fn move_up (&mut self) -> Self;
  fn move_down (&mut self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
  pub x: i16,
  pub y: i16,
}

impl Vec2D {
  pub fn from_x_y(x: i16, y: i16) -> Self {
    Vec2D { x, y }
  }

  pub fn zero() -> Self {
    Vec2D { x: 0, y: 0 }
  }
}

impl Movable2D for Vec2D {
  fn move_right (&mut self) -> Self {
    self.x += 1;
    *self
  }

  fn move_left (&mut self) -> Self {
    self.x -= 1;
    *self
  }

  fn move_up (&mut self) -> Self {
    self.y += 1;
    *self
  }

  fn move_down (&mut self) -> Self {
    self.y -= 1;
    *self
  }
}

impl PartialEq for Vec2D {
  fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
  }
}
