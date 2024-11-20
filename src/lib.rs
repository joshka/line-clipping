#![doc = include_str!("../README.md")]
pub mod cohen_sutherland;

/// A point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

/// A line segment in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    pub p1: Point,
    pub p2: Point,
}

impl LineSegment {
    /// Creates a new line segment.
    pub fn new(p1: Point, p2: Point) -> Self {
        LineSegment { p1, p2 }
    }
}

/// A rectangular region to clip lines against.
#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl Window {
    /// Creates a new window.
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Window {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}
