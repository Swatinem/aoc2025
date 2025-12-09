pub fn a(input: &str) -> i64 {
    let points: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    points
        .iter()
        .enumerate()
        .flat_map(|(p1_idx, p1)| {
            points
                .iter()
                .skip(p1_idx + 1)
                .map(|p2| (p1.0 + 1 - p2.0).abs() * (p1.1 + 1 - p2.1).abs())
        })
        .max()
        .unwrap()
}

pub fn b(input: &str) -> i64 {
    let mut points: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    // make it wrap around
    points.insert(0, points.last().copied().unwrap());

    #[derive(Debug)]
    struct Rect {
        xmin: i64,
        xmax: i64,
        ymin: i64,
        ymax: i64,
    }

    // this is <https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm>
    let outside_box = |rect: &Rect, (x0, y0), (x1, y1)| {
        const INSIDE: u8 = 0b0000;
        const LEFT: u8 = 0b0001;
        const RIGHT: u8 = 0b0010;
        const BOTTOM: u8 = 0b0100;
        const TOP: u8 = 0b1000;
        let compute_outcode = |x, y| {
            let mut code = INSIDE; // initialised as being inside of clip window

            if x <= rect.xmin {
                // to the left of clip window
                code |= LEFT;
            } else if x >= rect.xmax {
                // to the right of clip window
                code |= RIGHT;
            }
            if y <= rect.ymin {
                // below the clip window
                code |= BOTTOM;
            } else if y >= rect.ymax {
                // above the clip window
                code |= TOP;
            }

            return code;
        };

        let outcode0 = compute_outcode(x0, y0);
        let outcode1 = compute_outcode(x1, y1);

        // bitwise AND is not 0: both points share an outside zone (LEFT, RIGHT, TOP,
        // or BOTTOM), so both must be outside window; exit loop (accept is false)
        (outcode0 & outcode1) > 0
    };
    let intersects_polygon = |rect: &Rect| {
        for line in points.windows(2) {
            let p1 = line[0];
            let p2 = line[1];
            if !outside_box(rect, p1, p2) {
                return true;
            }
        }
        return false;
    };

    let pairs = points
        .iter()
        .enumerate()
        .skip(1) // ignore wrap around
        .flat_map(|(p1_idx, p1)| points.iter().skip(p1_idx + 1).map(|p2| (*p1, *p2)));

    let mut max_area = 0;
    for (p1, p2) in pairs {
        let xmin = p1.0.min(p2.0);
        let ymin = p1.1.min(p2.1);
        let xmax = p1.0.max(p2.0);
        let ymax = p1.1.max(p2.1);
        let rect = Rect {
            xmin,
            xmax,
            ymin,
            ymax,
        };

        if !intersects_polygon(&rect) {
            let area = (xmax - xmin + 1) * (ymax - ymin + 1);
            max_area = max_area.max(area);
        }
    }

    max_area
}

#[cfg(test)]
const TEST: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

#[test]
fn test() {
    assert_eq!(a(TEST), 50);
    assert_eq!(b(TEST), 24);
}
