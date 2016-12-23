// Generates a set of points on the map

use std::collections::*;
use std::cmp::*;

use fourthrail::*;

fn is_on_map(coord: &Coord) -> bool {
    let (r, c) = *coord;
    r >= 0 && r < MAP_HEIGHT && c >= 0 && c < MAP_WIDTH
}

pub fn circle(centre: &Coord, rad: i32) -> Coords {
    let (r0, c0) = *centre;
    let mut result: Coords = BTreeSet::new();
    for r in max((r0 - rad), 0)..min((r0 + 1 + rad), MAP_HEIGHT) {
        for c in max((c0 - rad), 0)..min((c0 + 1 + rad), MAP_WIDTH) {
            let dr = ((r - r0) as f64).abs() - 0.4;
            let dc = ((c - c0) as f64).abs() - 0.4;
            if (dr * dr) + (dc * dc) <= (rad * rad) as f64 {
                result.insert((r, c));
            }
        }
    }

    result
}

pub fn line(start: &Coord, end: &Coord) -> Coords {
    let (mut r, mut c) = *start;
    let (r1, c1) = *end;
    let mut result: Coords = BTreeSet::new();

    let dr = r1 - r;
    let dc = c1 - c;
    let mut e = 0.0;

    result.insert((r, c));
    if dr.abs() > dc.abs() {
        let es = (dc as f64 / dr as f64).abs();
        while r != r1 {
            e += es;
            if e > 0.5 {
                c += if dc > 0 {1} else {-1};
                e -= 1.0;
            }
            r += if dr > 0 {1} else {-1};
            result.insert((r, c));
        }
    } else {
        let es = (dr as f64 / dc as f64).abs();
        while c != c1 {
            e += es;
            if e > 0.5 {
                r += if dr > 0 {1} else {-1};
                e -= 1.0;
            }
            c += if dc > 0 {1} else {-1};
            result.insert((r, c));
        }
    }

    result
}
