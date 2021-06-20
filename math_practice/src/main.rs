mod numbers;
mod shapes;

fn display_shapes(rectangles: [shapes::Rectangle; 5], circles: [shapes::Circle; 5]) {
    println!("\nRectangles\n----------");
    for r in &rectangles {
        r.display();
    }

    println!("\nCircles\n-------");
    for c in &circles {
        c.display();
    }
}

fn init() -> ([shapes::Rectangle; 5], [shapes::Circle; 5]) {
    let rectangles: [shapes::Rectangle; 5] = [
        shapes::Rectangle {
            height: 10,
            width: 20,
        },
        shapes::Rectangle {
            height: 10,
            width: 10,
        },
        shapes::Rectangle {
            height: 15,
            width: 5,
        },
        shapes::Rectangle {
            height: 7,
            width: 9,
        },
        shapes::Rectangle {
            height: 13,
            width: 13,
        },
    ];

    let circles: [shapes::Circle; 5] = [
        shapes::Circle { radius: 20 },
        shapes::Circle { radius: 5 },
        shapes::Circle { radius: 10 },
        shapes::Circle { radius: 25 },
        shapes::Circle { radius: 15 },
    ];

    (rectangles, circles)
}

fn main() {
    let shape_lists = init();
    display_shapes(shape_lists.0, shape_lists.1);

    println!("\nFactorials\n----------");
    for i in 1..10 {
        println!("{}", numbers::fac(i));
    }
}
