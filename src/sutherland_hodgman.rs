//! Implements the Sutherland-Hodgman polygon clipping algorithm.
//!
//! Return the clipped polygon.
//!
//! Reference: [Sutherland-Hodgman](https://en.wikipedia.org/wiki/Sutherland%E2%80%93Hodgman_algorithm)
//!
//! The Sutherland-Hodgman algorithm clips a polygon against a convex clipping window.
//! For each edge of the clipping window, the current polygon is clipped against
//! that edge, producing a new output polygon.
//!
//! For every edge of the clipping window:
//! 1. Start with an empty output polygon.
//! 2. For each edge of the subject polygon (formed by the previous and current
//!    vertices):
//!    1. If both vertices lie inside, push the current vertex.
//!    2. If the previous vertex is inside and the current one is outside,
//!       push the intersection point.
//!    3. If the previous vertex is outside and the current one is inside,
//!       push the intersection point, then the current vertex.
//!    4. If both vertices are outside, push nothing.
//! 3. Use the output polygon as the input polygon for the next clipping edge.
//!
//! After all clipping window edges have been processed, the remaining vertices
//! form the clipped polygon.
//!
//! # Examples
//!
//! ```rust
//! use line_clipping::{Point, Polygon, Window, sutherland_hodgman};
//!
//! let vertices: Vec<(f64, f64)> = vec![(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)];
//!
//! let vertices: Vec<Point> = vertices
//!     .into_iter()
//!     .map(|(x, y)| Point::new(x, y))
//!     .collect();
//! let polygon = Polygon::new(&vertices);
//!
//! let clipped = sutherland_hodgman::clip_polygon(&polygon, Window::new(0.0, 3.0, 0.0, 3.0));
//! // Clipped polygon vertices:
//! // [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)]
//! ```

use crate::{Point, Polygon, Window};
use alloc::vec::Vec;

/// Clips a polygon against a rectangular window using the Sutherland-Hodgman algorithm.
///
/// See the [module-level documentation](crate::sutherland_hodgman) for more details on the algorithm.
///
/// # Examples
///
/// ```rust
/// use line_clipping::{Point, Polygon, Window, sutherland_hodgman};
///
/// let vertices: Vec<(f64, f64)> = vec![(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)];
///
/// let vertices: Vec<Point> = vertices
///     .into_iter()
///     .map(|(x, y)| Point::new(x, y))
///     .collect();
/// let polygon = Polygon::new(&vertices);
///
/// let clipped = sutherland_hodgman::clip_polygon(&polygon, Window::new(0.0, 3.0, 0.0, 3.0));
/// // Clipped polygon vertices:
/// // [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)]
/// ```
#[must_use]
pub fn clip_polygon(vertices: &Polygon, window: Window) -> Polygon {
    let clipped = &vertices.vertices;
    let clipped = clip_left(clipped, window.x_min);
    let clipped = clip_right(&clipped, window.x_max);
    let clipped = clip_bottom(&clipped, window.y_min);
    Polygon::new(&clip_top(&clipped, window.y_max))
}

fn clip_top(clipped: &[Point], y_max: f64) -> Vec<Point> {
    clip_edge(
        clipped,
        |p| p.y <= y_max,
        |p1, p2| {
            let t = (y_max - p1.y) / (p2.y - p1.y);
            Point::new(p1.x + t * (p2.x - p1.x), y_max)
        },
    )
}

fn clip_bottom(clipped: &[Point], y_min: f64) -> Vec<Point> {
    clip_edge(
        clipped,
        |p| p.y >= y_min,
        |p1, p2| {
            let t = (y_min - p1.y) / (p2.y - p1.y);
            Point::new(p1.x + t * (p2.x - p1.x), y_min)
        },
    )
}

fn clip_right(clipped: &[Point], x_max: f64) -> Vec<Point> {
    clip_edge(
        clipped,
        |p| p.x <= x_max,
        |p1, p2| {
            let t = (x_max - p1.x) / (p2.x - p1.x);
            Point::new(x_max, p1.y + t * (p2.y - p1.y))
        },
    )
}

fn clip_left(clipped: &[Point], x_min: f64) -> Vec<Point> {
    clip_edge(
        clipped,
        |p| p.x >= x_min,
        |p1, p2| {
            let t = (x_min - p1.x) / (p2.x - p1.x);
            Point::new(x_min, p1.y + t * (p2.y - p1.y))
        },
    )
}

fn clip_edge<F, I>(vertices: &[Point], is_inside: F, get_intersection: I) -> Vec<Point>
where
    F: Fn(Point) -> bool,
    I: Fn(Point, Point) -> Point,
{
    let mut result = Vec::new();
    let len = vertices.len();
    for i in 0..len {
        let p1 = vertices[i];
        // % len to connect last and first vertices
        let p2 = vertices[(i + 1) % len];

        let p1_inside = is_inside(p1);
        let p2_inside = is_inside(p2);

        if p2_inside {
            if !p1_inside {
                result.push(get_intersection(p1, p2));
            }
            result.push(p2);
        } else if p1_inside {
            result.push(get_intersection(p1, p2));
        } else {
            // we don't care if none of the points is inside
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use alloc::vec::Vec;

    // The vertex order in the expected results matches the input order, but may
    // be cyclically shifted: for each edge, `clip_edge` pushes the second vertex
    // (`p2`), so over the loop (v0->v1, v1->v2, ..., v_{n-1}->v0) it emits
    // v1, v2, ..., v0, moving the original first vertex to the end.

    /// The clipping window used by every test case.
    ///
    /// ```plain
    ///        (-1, 1) ─────── (1, 1)
    ///           │               │
    ///           │      .        │
    ///           │               │
    ///        (-1,-1) ─────── (1,-1)
    /// ```
    const WINDOW: Window = Window::new(-1.0, 1.0, -1.0, 1.0);

    /// Builds a [`Polygon`] from a slice of `(x, y)` tuples.
    fn poly(points: &[(f64, f64)]) -> Polygon {
        let vertices: Vec<Point> = points.iter().map(|&(x, y)| Point::new(x, y)).collect();
        Polygon::new(&vertices)
    }

    /// No clipping should occur: the polygon is either entirely inside the window (returned
    /// unchanged) or entirely outside it (reduced to an empty polygon).
    #[rstest]
    #[case::inside(
        &[(-0.5, -0.5), (-0.5, 0.5), (0.5, 0.5), (0.5, -0.5)],
        &[(-0.5, -0.5), (-0.5, 0.5), (0.5, 0.5), (0.5, -0.5)]
    )]
    #[case::outside_left(
        &[(-3.0, -0.5), (-3.0, 0.5), (-2.0, 0.5), (-2.0, -0.5)],
        &[]
    )]
    #[case::outside_right(
        &[(2.0, -0.5), (2.0, 0.5), (3.0, 0.5), (3.0, -0.5)],
        &[]
    )]
    #[case::outside_top(
        &[(-0.5, 2.0), (-0.5, 3.0), (0.5, 3.0), (0.5, 2.0)],
        &[]
    )]
    #[case::outside_bottom(
        &[(-0.5, -3.0), (-0.5, -2.0), (0.5, -2.0), (0.5, -3.0)],
        &[]
    )]
    #[case::outside_top_right(
        &[(2.0, 2.0), (2.0, 3.0), (3.0, 3.0), (3.0, 2.0)],
        &[]
    )]
    fn no_clipping(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// The polygon is clipped by exactly one edge of the window: top, bottom, right or left.
    #[rstest]
    #[case::top(
        &[(-0.5, -0.5), (-0.5, 1.5), (0.5, 1.5), (0.5, -0.5)],
        &[(-0.5, -0.5), (-0.5, 1.0), (0.5, 1.0), (0.5, -0.5)]
    )]
    #[case::bottom(
        &[(-0.5, 0.5), (-0.5, -1.5), (0.5, -1.5), (0.5, 0.5)],
        &[(0.5, 0.5), (-0.5, 0.5), (-0.5, -1.0), (0.5, -1.0)]
    )]
    #[case::right(
        &[(-0.5, -0.5), (1.5, -0.5), (1.5, 0.5), (-0.5, 0.5)],
        &[(-0.5, -0.5), (1.0, -0.5), (1.0, 0.5), (-0.5, 0.5)]
    )]
    #[case::left(
        &[(0.5, -0.5), (-1.5, -0.5), (-1.5, 0.5), (0.5, 0.5)],
        &[(0.5, -0.5), (-1.0, -0.5), (-1.0, 0.5), (0.5, 0.5)]
    )]
    fn one_edge(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// The polygon is clipped by exactly two edges of the window. All six combinations are
    /// covered: top-bottom, left-right and each of the four corners.
    #[rstest]
    #[case::top_bottom(
        &[(-0.5, -1.5), (-0.5, 1.5), (0.5, 1.5), (0.5, -1.5)],
        &[(-0.5, -1.0), (-0.5, 1.0), (0.5, 1.0), (0.5, -1.0)]
    )]
    #[case::left_right(
        &[(-1.5, -0.5), (1.5, -0.5), (1.5, 0.5), (-1.5, 0.5)],
        &[(-1.0, 0.5), (-1.0, -0.5), (1.0, -0.5), (1.0, 0.5)]
    )]
    #[case::top_right(
        &[(0.5, 0.5), (0.5, 1.5), (1.5, 1.5), (1.5, 0.5)],
        &[(0.5, 0.5), (0.5, 1.0), (1.0, 1.0), (1.0, 0.5)]
    )]
    #[case::top_left(
        &[(-0.5, 0.5), (-1.5, 0.5), (-1.5, 1.5), (-0.5, 1.5)],
        &[(-0.5, 1.0), (-0.5, 0.5), (-1.0, 0.5), (-1.0, 1.0)]
    )]
    #[case::bottom_right(
        &[(0.5, -0.5), (0.5, -1.5), (1.5, -1.5), (1.5, -0.5)],
        &[(1.0, -0.5), (0.5, -0.5), (0.5, -1.0), (1.0, -1.0)]
    )]
    #[case::bottom_left(
        &[(-0.5, -0.5), (-0.5, -1.5), (-1.5, -1.5), (-1.5, -0.5)],
        &[(-1.0, -0.5), (-0.5, -0.5), (-0.5, -1.0), (-1.0, -1.0)]
    )]
    fn two_edges(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// The polygon is clipped by exactly three edges of the window. All four combinations are
    /// covered (each one leaving a different edge untouched).
    #[rstest]
    #[case::top_bottom_left(
        &[(-1.5, -1.5), (-1.5, 1.5), (0.5, 1.5), (0.5, -1.5)],
        &[(-1.0, -1.0), (-1.0, 1.0), (0.5, 1.0), (0.5, -1.0)]
    )]
    #[case::top_bottom_right(
        &[(1.5, -1.5), (1.5, 1.5), (-0.5, 1.5), (-0.5, -1.5)],
        &[(-0.5, 1.0), (-0.5, -1.0), (1.0, -1.0), (1.0, 1.0)]
    )]
    #[case::top_left_right(
        &[(-1.5, 1.5), (1.5, 1.5), (1.5, -0.5), (-1.5, -0.5)],
        &[(-1.0, -0.5), (-1.0, 1.0), (1.0, 1.0), (1.0, -0.5)]
    )]
    #[case::bottom_left_right(
        &[(-1.5, -1.5), (1.5, -1.5), (1.5, 0.5), (-1.5, 0.5)],
        &[(1.0, 0.5), (-1.0, 0.5), (-1.0, -1.0), (1.0, -1.0)]
    )]
    fn three_edges(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// The polygon surrounds the window and is clipped by all four edges. The result is the
    /// window itself.
    #[rstest]
    #[case::all(
        &[(-2.0, -2.0), (-2.0, 2.0), (2.0, 2.0), (2.0, -2.0)],
        &[(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)]
    )]
    fn all_edges(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// Non-axis-aligned (diagonal) subject polygons. Unlike the rectangle cases
    /// above, clipping a slanted edge exercises the actual intersection
    /// interpolation in `get_intersection`, and the vertex count changes as
    /// corners are cut off and new intersection points are inserted
    /// (3-gon -> 4-gon, 4-gon -> 5-gon, 4-gon -> 6-gon).
    #[rstest]
    #[case::triangle_to_quadrilateral(
        &[(0.0, 0.0), (-0.5, 2.0), (0.5, 0.0)],
        &[(-0.25, 1.0), (0.0, 1.0), (0.5, 0.0), (0.0, 0.0)]
    )]
    #[case::quadrilateral_to_pentagon(
        &[(-0.75, 0.5), (0.0, 1.5), (0.75, 0.5), (0.0, -0.5)],
        &[(-0.75, 0.5), (-0.375, 1.0), (0.375, 1.0), (0.75, 0.5), (0.0, -0.5)]
    )]
    #[case::quadrilateral_to_hexagon(
        &[(-0.5, 0.0), (0.0, 2.0), (0.5, 0.0), (0.0, -2.0)],
        &[(-0.25, -1.0), (-0.5, 0.0), (-0.25, 1.0), (0.25, 1.0), (0.5, 0.0), (0.25, -1.0)]
    )]
    fn diagonal_edges(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }
}
