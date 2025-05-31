use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }
}

impl std::ops::Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn main() {
  let p1 = Point::new(10, 20);
  let p2 = Point::new(30, 40);
  let p3 = p1 + p2;
  // p3 is a Point with x = 40 and y = 60
  println!("Point 3: {}", p3);
}
