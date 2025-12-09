use std::collections::HashMap;

pub fn a(input: &str, num_connections: usize) -> u64 {
    let points = parse_points(input);
    let distances = calculate_distances(&points);

    #[derive(Debug)]
    struct ConnectedPoint {
        pos: Point,
        circuit: Option<usize>,
    }

    let mut points: Vec<_> = points
        .into_iter()
        .map(|p| ConnectedPoint {
            pos: p,
            circuit: None,
        })
        .collect();

    let mut next_circuit = 0;
    let mut circuits = HashMap::new();
    for distance in distances.into_iter().take(num_connections) {
        let [p1, p2] =
            unsafe { points.get_disjoint_unchecked_mut([distance.p1_idx, distance.p2_idx]) };
        match (p1.circuit, p2.circuit) {
            (None, None) => {
                let c = next_circuit;
                next_circuit += 1;
                p1.circuit = Some(c);
                p2.circuit = Some(c);
                circuits.insert(c, 2);
            }
            (Some(c), None) | (None, Some(c)) => {
                p1.circuit = Some(c);
                p2.circuit = Some(c);
                *circuits.get_mut(&c).unwrap() += 1;
            }
            (Some(c1), Some(c2)) if c1 == c2 => {} // already connected
            (Some(c1), Some(c2)) => {
                let n_c2 = circuits.remove(&c2).unwrap();
                *circuits.get_mut(&c1).unwrap() += n_c2;
                for p in points.iter_mut().filter(|p| p.circuit == Some(c2)) {
                    p.circuit = Some(c1);
                }
            }
        }
    }

    let mut sorted_circuits: Vec<_> = circuits.values().copied().collect();
    sorted_circuits.sort_unstable();
    sorted_circuits.iter().rev().take(3).product()
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut s = l.split(',');
            Point {
                x: s.next().unwrap().parse().unwrap(),
                y: s.next().unwrap().parse().unwrap(),
                z: s.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}
impl Point {
    fn distance(&self, other: &Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }
}
#[derive(Debug)]
struct Distance {
    p1_idx: usize,
    p2_idx: usize,
    distance: f32,
}

fn calculate_distances(points: &[Point]) -> Vec<Distance> {
    let mut distances: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(p1_idx, p1)| {
            points
                .iter()
                .enumerate()
                .skip(p1_idx + 1)
                .map(move |(p2_idx, p2)| Distance {
                    p1_idx,
                    p2_idx,
                    distance: p1.distance(p2),
                })
        })
        .collect();
    distances.sort_unstable_by(|a, b| a.distance.total_cmp(&b.distance));
    distances
}

pub fn b(input: &str) -> u64 {
    let points = parse_points(input);
    let num_points = points.len();
    let distances = calculate_distances(&points);

    #[derive(Debug)]
    struct ConnectedPoint {
        pos: Point,
        circuit: Option<usize>,
    }

    let mut points: Vec<_> = points
        .into_iter()
        .map(|p| ConnectedPoint {
            pos: p,
            circuit: None,
        })
        .collect();

    let mut next_circuit = 0;
    let mut circuits = HashMap::new();
    for distance in distances {
        let [p1, p2] =
            unsafe { points.get_disjoint_unchecked_mut([distance.p1_idx, distance.p2_idx]) };
        let x1 = p1.pos.x;
        let x2 = p2.pos.x;
        match (p1.circuit, p2.circuit) {
            (None, None) => {
                let c = next_circuit;
                next_circuit += 1;
                p1.circuit = Some(c);
                p2.circuit = Some(c);
                circuits.insert(c, 2);
            }
            (Some(c), None) | (None, Some(c)) => {
                p1.circuit = Some(c);
                p2.circuit = Some(c);
                *circuits.get_mut(&c).unwrap() += 1;
            }
            (Some(c1), Some(c2)) if c1 == c2 => {} // already connected
            (Some(c1), Some(c2)) => {
                let n_c2 = circuits.remove(&c2).unwrap();
                *circuits.get_mut(&c1).unwrap() += n_c2;
                for p in points.iter_mut().filter(|p| p.circuit == Some(c2)) {
                    p.circuit = Some(c1);
                }
            }
        }
        if circuits.len() == 1 && *circuits.values().next().unwrap() == num_points {
            return (x1 * x2) as u64;
        }
    }
    panic!()
}

#[cfg(test)]
const TEST: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

#[test]
fn test() {
    assert_eq!(a(TEST, 10), 40);
    assert_eq!(b(TEST), 25272);
}
