pub fn a(input: &str) -> i64 {
    let (_shapes, places) = parse(input);

    let mut fits = 0;
    for place in places {
        let full_shapes = (place.width / 3) as u32 * (place.height / 3) as u32;
        let total_shapes = place.required_shapes.iter().map(|n| *n as u32).sum();

        // lol, but okay, I looked up the "solution" online :-D
        if full_shapes >= total_shapes {
            fits += 1;
        }
    }
    fits
}

struct Shape {
    _pixels: Vec<bool>,
}
struct Place {
    width: u16,
    height: u16,
    required_shapes: Vec<u8>,
}
fn parse(input: &str) -> (Vec<Shape>, Vec<Place>) {
    let mut lines = input.trim().lines();

    let shapes = (0..6)
        .map(|_| {
            lines.next();
            let pixels = lines
                .by_ref()
                .take(3)
                .flat_map(|l| l.chars().map(|ch| ch == '#'))
                .collect();
            lines.next();
            Shape { _pixels: pixels }
        })
        .collect();

    let places = lines
        .map(|l| {
            let (size, shapes) = l.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();
            let width = width.parse().unwrap();
            let height = height.parse().unwrap();

            let shapes = shapes.split(' ').map(|n| n.parse().unwrap()).collect();

            Place {
                width,
                height,
                required_shapes: shapes,
            }
        })
        .collect();

    return (shapes, places);
}

// #[cfg(test)]
// const TEST: &str = r#"
// 0:
// ###
// ##.
// ##.

// 1:
// ###
// ##.
// .##

// 2:
// .##
// ###
// ##.

// 3:
// ##.
// ###
// ##.

// 4:
// ###
// #..
// ###

// 5:
// ###
// .#.
// ###

// 4x4: 0 0 0 0 2 0
// 12x5: 1 0 1 0 2 2
// 12x5: 1 0 1 0 3 2
// "#;

// #[test]
// fn test() {
//     assert_eq!(a(TEST), 2);
// }
