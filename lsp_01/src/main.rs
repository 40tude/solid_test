// cargo run -p ex_01_lsp

// =========================
// Rectangle/Square problem
// =========================

// =========================
// Abstractions
// =========================

pub trait Shape {
    fn set_width(&mut self, width: f64);
    fn set_height(&mut self, height: f64);
    fn area(&self) -> f64;
}

// =========================
// Concrete shapes
// =========================

// Rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Square
// A square is a rectangle, right? Mathematically yes. In software? Trouble.
pub struct Square {
    side: f64,
}

impl Shape for Square {
    fn set_width(&mut self, width: f64) {
        self.side = width; // Setting width changes the square's side
    }

    fn set_height(&mut self, height: f64) {
        self.side = height; // Setting height ALSO changes the square's side
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// =========================
// Usage
// =========================

fn main() {
    let mut my_square = Square { side: 20.0 };
    let area = my_square.area();
    println!("Expected area: 400, Got: {}", area);

    my_square.set_width(10.0);
    my_square.set_height(13.0);
    let area = my_square.area();

    // We expect: width=10, height=13, area=130
    // With Rectangle: CORRECT (10 * 13 = 130)
    // With Square: WRONG! (13 * 13 = 169)
    // The last set_height overwrote the width
    println!("Expected area: 130, Got: {}", area);
}
