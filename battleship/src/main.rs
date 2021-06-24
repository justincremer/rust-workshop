struct Board<'a> {
    player: u8,
    grid: [char; 64], // 8 x 8
    ship_locations: &'a Locations,
}

struct Locations<'a> {
    carrier: &'a [u8; 5],    // 5
    battleship: &'a [u8; 4], // 4
    cruiser: &'a [u8; 3],    // 3
    submarine: &'a [u8; 3],  // 3
    destroyer: &'a [u8; 2],  // 2
}

impl<'a> Iterator for Locations<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl Board<'_> {
    fn new<'a>(player: u8, locations: &'a Locations) -> &'a Board {
        &'a Board {
            player: player,
            grid: ['#'; 64],
            ship_locations: locations,
        }
    }

    fn pick(&self, point: u8) {
        let mut hit = false;
        for l in self.ship_locations {
            for p in l {}
        }

        self.grid[point]
    }

    fn win(&self) -> bool {
        let mut acc = 0;
        for c in &self.grid {
            if *c == 'x' {
                acc += 1;
            }
        }
        acc == 17
    }
}

fn main() {
    println!("Hello, world!");
}
