use plotters::prelude::*;
use std::{env, process::exit};
use tsp_rs::{Metrizable, Tour};

#[derive(Clone)]
pub(crate) struct Point {
    pub x: f64,
    pub y: f64,
}

impl Metrizable for Point {
    fn cost(&self, other: &Point) -> f64 {
        return ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt();
    }
}

fn main() {
    // points width height x1,y1;x2,y2;x3,y3
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    let points = if let [_, points] = &args[..] {
        points
    } else {
        println!("Usage: pathfind x1,y1;x2,y2...");
        exit(0);
    };

    if points.is_empty() {
        panic!("");
    }

    let PointScene {
        width,
        height,
        points,
    } = parse_points(points);

    let root_area = BitMapBackend::new(
        "output.png",
        (width.try_into().unwrap(), height.try_into().unwrap()),
    )
    .into_drawing_area();
    root_area.fill(&BLACK).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .build_cartesian_2d(0..width, 0..height)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        points.iter().map(|(x, y): &(f64, f64)| {
            Circle::new((x.round() as i32, y.round() as i32), 3, &WHITE)
        }),
    )
    .unwrap();

    let mut tour = Tour::from(
        &points
            .iter()
            .map(|point| Point {
                x: point.0,
                y: point.1,
            })
            .collect(),
    );

    tour.optimize_kopt(std::time::Duration::from_secs(10));

    ctx.draw_series(
        tour.path
            .iter()
            .map(|Point { x, y }| Circle::new((x.round() as i32, y.round() as i32), 3, &WHITE)),
    )
    .unwrap();

    ctx.draw_series(LineSeries::new(
        tour.path
            .iter()
            .map(|Point { x, y }| (x.round() as i32, y.round() as i32)),
        &WHITE,
    ))
    .unwrap();

    println!("{}", tour.path.len());

    root_area.present().unwrap();
}

struct PointScene {
    points: Vec<(f64, f64)>,
    width: i32,
    height: i32,
}
fn parse_points(points: &String) -> PointScene {
    let points = points.split(";");
    let points = points
        .map(|point| {
            let (x, y) = point.split_once(",").ok_or("Uhoh").unwrap();
            let (x, y) = (x.parse::<f64>().unwrap(), y.parse::<f64>().unwrap());
            (x, y)
        })
        .into_iter();

    let width = points.clone().map(|point| point.0 as i32).max().unwrap();
    let height = points.clone().map(|point| point.1 as i32).max().unwrap();

    PointScene {
        points: points.collect(),
        width,
        height,
    }
}
