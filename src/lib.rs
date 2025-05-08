#![no_std]
//! A rust crate to implement several line clipping algorithms. See the
//! [documentation](https://docs.rs/line_clipping) for more information. The choice of algorithms is
//! based on the following article which contains a good summary of the options:
//!
//! Matthes D, Drakopoulos V. [Line Clipping in 2D: Overview, Techniques and
//! Algorithms](https://pmc.ncbi.nlm.nih.gov/articles/PMC9605407/). J Imaging. 2022 Oct
//! 17;8(10):286. doi: 10.3390/jimaging8100286. PMID: 36286380; PMCID: PMC9605407.
//!
//! Supports:
//!
//! - [x] Cohen-Sutherland
//!
//! TODO
//!
//! - [ ] Cyrus-Beck
//! - [ ] Liang-Barsky
//! - [ ] Nicholl-Lee-Nicholl
//! - [ ] More comprehensive testing
//!
//! # Installation
//!
//! ```shell
//! cargo add line-clipping
//! ```
//!
//! # Usage
//!
//! ```rust
//! use line_clipping::{
//!     LineSegment, Point, Window,
//!     cohen_sutherland::clip_line,
//! }
//!
//! let line = clip_line(
//!     LineSegment::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0)),
//!     Window::new(1.0, 9.0, 1.0, 9.0),
//! );
//! ```
//!
//! # License
//!
//! Copyright (c) 2024 Josh McKinney
//!
//! This project is licensed under either of
//!
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <http://www.apache.org/licenses/LICENSE-2.0>)
//!
//! at your option.
//!
//! # Contribution
//!
//! Contributions are welcome! Please open an issue or submit a pull request.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
//! the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
//! any additional terms or conditions.
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
