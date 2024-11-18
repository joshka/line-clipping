#![doc = include_str!("../README.md")]
pub mod cohen_sutherland;

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

/// A line segment in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
}

/// A rectangular region to clip lines against.
#[derive(Debug, Clone, Copy)]
pub struct Window {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}
