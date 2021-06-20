use std::f64::consts::PI;

pub struct Rectangle {
    pub height: u32,
    pub width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        self.height * 2 + self.width * 2
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }

    pub fn display(&self) {
        println!(
            "{} {} {} {} {}",
            self.height,
            self.width,
            self.area(),
            self.perimeter(),
            self.is_square(),
        )
    }
}

pub struct Circle {
    pub radius: u32,
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    fn circumferance(&self) -> f64 {
        2 as f64 * PI * self.radius as f64
    }

    fn area(&self) -> f64 {
        let r = self.radius;
        PI * (r * r) as f64
    }

    pub fn display(&self) {
        println!(
            "{} {} {} {}",
            self.radius,
            self.diameter(),
            self.circumferance(),
            self.area(),
        )
    }
}

// struct Point {
//     x: f64,
//     y: f64,
// }

// pub struct Triangle {
//     points: [Point; 3],
// }

// impl Triangle {
//     fn perimeter(&self) -> f64 {
//         let p = self.points;
//         let sum = p.iter().sum();
//         sum
//     }

//     // fn area(&self) -> f64 {}

//     pub fn display(&self) {}
// }
