use geo::convex_hull::ConvexHull;
use geo::{line_string, polygon};

fn main() {
    println!("Hello, world!");

    // An L shape
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];
    println!("{:?}", poly);
    println!("{:?}", poly.exterior());
    println!("{:?}", poly.exterior());
}
