struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let input = 361527;

    let coords = vec![
        Coord { x: 1, y: 0},
        Coord { x: 0, y: 1},
        Coord { x: -1, y: 0},
        Coord { x: 0, y: -1},
    ];

    let mut x = 0;
    let mut y = 0;
    let mut counter = 1;
    let mut n = 1;
    let mut step = 0;
    let mut index = 0;

    'outer: loop {
        let coord = &coords[index];

        for _i in 0..n {
            x += coord.x;
            y += coord.y;
            counter += 1;
            if counter == input {
                break 'outer;
            }
        }

        if step == 1 {
            step = 0;
            n += 1;
        } else {
            step += 1;
        }

        index = (index + 1) % 4
    }

    println!("{}", x.abs() + y.abs());
}
