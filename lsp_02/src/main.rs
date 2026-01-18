// cargo run -p ex_02_lsp

// =========================
// Rectangle/Square solution
// =========================

// =========================
// Abstractions
// =========================

// Immutable shapes with clear contracts
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// =========================
// Concrete shapes
// =========================

// Rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Square
pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

// =========================
// Usage
// =========================

fn main() {
    let my_square = Square { side: 20.0 };
    println!(
        "Area: {}, Perimeter: {}",
        my_square.area(),
        my_square.perimeter()
    );

    let my_rect = Rectangle {
        width: 6.0,
        height: 7.0,
    };
    println!(
        "Area: {}, Perimeter: {}",
        my_rect.area(),
        my_rect.perimeter()
    );
}
