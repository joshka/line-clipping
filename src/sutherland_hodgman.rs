//! Implements the Sutherland-Hodgman polygon clipping algorithm.
//!
//! Returns the portion of a polygon inside a rectangular clipping window.
//!
//! Reference: Sutherland and Hodgman,
//! [*Reentrant Polygon Clipping*](https://doi.org/10.1145/360767.360802).
//!
//! The Sutherland-Hodgman algorithm clips a polygon against a convex clipping window. For each edge
//! of the clipping window, the current polygon is clipped against that edge, producing a new output
//! polygon.
//!
//! For every edge of the clipping window:
//! 1. Start with an empty output polygon.
//! 2. For each edge of the subject polygon (formed by the previous and current vertices):
//!    1. If both vertices lie inside, push the current vertex.
//!    2. If the previous vertex is inside and the current one is outside, push the intersection
//!       point.
//!    3. If the previous vertex is outside and the current one is inside, push the intersection
//!       point, then the current vertex.
//!    4. If both vertices are outside, push nothing.
//! 3. Use the output polygon as the input polygon for the next clipping edge.
//!
//! After all clipping window edges have been processed, the remaining vertices
//! form the clipped polygon.
//!
//! # Input and output
//!
//! Input vertices must occur consecutively around the polygon boundary. Clockwise and
//! counter-clockwise orderings are both supported, and the closing vertex must not repeat the first
//! vertex. The output preserves the input traversal direction. When no clipping is needed, it also
//! preserves the input's starting vertex rather than cyclically rotating the sequence.
//!
//! Concave and self-intersecting inputs can produce disconnected visible regions. This function
//! returns a single [`Polygon`], so those regions may be joined by overlapping edges along the
//! clipping-window boundary. The result is not necessarily a simple polygon and can produce bridge
//! artifacts when its outline is rendered. Holes and multiple boundary rings are not represented.
//!
//! # Examples
//!
//! ```
//! use line_clipping::sutherland_hodgman::clip_polygon;
//! use line_clipping::{Point, Polygon, Window};
//!
//! let polygon = Polygon::new(&[
//!     Point::new(-1.0, -1.0),
//!     Point::new(-1.0, 1.0),
//!     Point::new(1.0, 1.0),
//!     Point::new(1.0, -1.0),
//! ]);
//!
//! let clipped = clip_polygon(&polygon, Window::new(0.0, 3.0, 0.0, 3.0));
//!
//! assert_eq!(
//!     clipped.vertices.as_slice(),
//!     &[
//!         Point::new(0.0, 0.0),
//!         Point::new(0.0, 1.0),
//!         Point::new(1.0, 1.0),
//!         Point::new(1.0, 0.0),
//!     ]
//! );
//! ```
//!
//! For concave or self-intersecting input, clipping can produce disconnected regions. This function
//! returns a single vertex sequence, so such regions may be joined by overlapping edges along the
//! clipping-window boundary. The result is not necessarily a simple polygon and may produce bridge
//! artifacts when its outline is rendered.
//!
//! ```text
//! Input polygon:                 After clipping:
//! ┌───────────────┐
//! │               │
//! │  ┌─────────┐  │                   bridge
//! │  │         │  │    ──────>   ┌──┐=========┌──┐
//! │  │         │  │              │  │         │  │
//! └──┘         └──┘              └──┘         └──┘
//! ```

use alloc::vec::Vec;

use crate::{Point, Polygon, Window};

/// Clips a polygon against a rectangular window using the Sutherland-Hodgman algorithm.
///
/// Input vertices must occur consecutively around the polygon boundary, in either clockwise or
/// counter-clockwise order. See the [module-level documentation](crate::sutherland_hodgman) for
/// details about disconnected results and degenerate boundary edges.
///
/// # Examples
///
/// ```
/// use line_clipping::sutherland_hodgman::clip_polygon;
/// use line_clipping::{Point, Polygon, Window};
///
/// let polygon = Polygon::new(&[
///     Point::new(-1.0, -1.0),
///     Point::new(-1.0, 1.0),
///     Point::new(1.0, 1.0),
///     Point::new(1.0, -1.0),
/// ]);
///
/// let clipped = clip_polygon(&polygon, Window::new(0.0, 3.0, 0.0, 3.0));
///
/// assert_eq!(
///     clipped.vertices.as_slice(),
///     &[
///         Point::new(0.0, 0.0),
///         Point::new(0.0, 1.0),
///         Point::new(1.0, 1.0),
///         Point::new(1.0, 0.0),
///     ]
/// );
/// ```
#[must_use]
pub fn clip_polygon(polygon: &Polygon, window: Window) -> Polygon {
    let clipped = &polygon.vertices;
    let clipped = clip_left(clipped, window.x_min);
    let clipped = clip_right(&clipped, window.x_max);
    let clipped = clip_bottom(&clipped, window.y_min);
    clip_top(&clipped, window.y_max).into()
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

fn clip_edge<F, I>(vertices: &[Point], is_inside: F, get_intersection: I) -> Vec<Point>
where
    F: Fn(Point) -> bool,
    I: Fn(Point, Point) -> Point,
{
    // Begin with the closing edge from the last vertex to the first, then emit each current vertex
    // in input order. An equivalent current-to-next formulation emits the next vertex first and
    // cyclically rotates the sequence once for every clipping boundary.
    let Some(&last) = vertices.last() else {
        return Vec::new();
    };

    let mut result = Vec::with_capacity(vertices.len());
    let mut previous = last;
    for &current in vertices {
        let previous_inside = is_inside(previous);
        let current_inside = is_inside(current);

        match (previous_inside, current_inside) {
            (true, true) => result.push(current),
            (true, false) => result.push(get_intersection(previous, current)),
            (false, true) => {
                result.push(get_intersection(previous, current));
                result.push(current);
            }
            (false, false) => {}
        }

        previous = current;
    }
    result
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use rstest::rstest;

    use super::*;

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
        vertices.into()
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
    #[case::inside_triangle(
        &[(-0.5, -0.5), (0.0, 0.5), (0.5, -0.5)],
        &[(-0.5, -0.5), (0.0, 0.5), (0.5, -0.5)]
    )]
    #[case::empty(&[], &[])]
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
        &[(-0.5, 0.5), (-0.5, -1.0), (0.5, -1.0), (0.5, 0.5)]
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
        &[(-1.0, -0.5), (1.0, -0.5), (1.0, 0.5), (-1.0, 0.5)]
    )]
    #[case::top_right(
        &[(0.5, 0.5), (0.5, 1.5), (1.5, 1.5), (1.5, 0.5)],
        &[(1.0, 1.0), (1.0, 0.5), (0.5, 0.5), (0.5, 1.0)]
    )]
    #[case::top_left(
        &[(-0.5, 0.5), (-1.5, 0.5), (-1.5, 1.5), (-0.5, 1.5)],
        &[(-0.5, 1.0), (-0.5, 0.5), (-1.0, 0.5), (-1.0, 1.0)]
    )]
    #[case::bottom_right(
        &[(0.5, -0.5), (0.5, -1.5), (1.5, -1.5), (1.5, -0.5)],
        &[(1.0, -1.0), (1.0, -0.5), (0.5, -0.5), (0.5, -1.0)]
    )]
    #[case::bottom_left(
        &[(-0.5, -0.5), (-0.5, -1.5), (-1.5, -1.5), (-1.5, -0.5)],
        &[(-1.0, -1.0), (-1.0, -0.5), (-0.5, -0.5), (-0.5, -1.0)]
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
        &[(1.0, -1.0), (1.0, 1.0), (-0.5, 1.0), (-0.5, -1.0)]
    )]
    #[case::top_left_right(
        &[(-1.5, 1.5), (1.5, 1.5), (1.5, -0.5), (-1.5, -0.5)],
        &[(-1.0, 1.0), (1.0, 1.0), (1.0, -0.5), (-1.0, -0.5)]
    )]
    #[case::bottom_left_right(
        &[(-1.5, -1.5), (1.5, -1.5), (1.5, 0.5), (-1.5, 0.5)],
        &[(-1.0, -1.0), (1.0, -1.0), (1.0, 0.5), (-1.0, 0.5)]
    )]
    fn three_edges(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// The polygon surrounds the window and is clipped by all four edges. The result is the
    /// window itself.
    #[rstest]
    #[case::all(
        &[(-2.0, -2.0), (-2.0, 2.0), (2.0, 2.0), (2.0, -2.0)],
        &[(1.0, 1.0), (1.0, -1.0), (-1.0, -1.0), (-1.0, 1.0)]
    )]
    fn all_edges(#[case] input: &[(f64, f64)], #[case] expected: &[(f64, f64)]) {
        assert_eq!(clip_polygon(&poly(input), WINDOW), poly(expected));
    }

    /// Clipping away the base of a concave U leaves two disconnected regions. Because the
    /// algorithm returns one vertex sequence, it joins the regions with overlapping edges along
    /// the bottom clipping boundary.
    #[test]
    fn concave_u_joins_disconnected_regions() {
        let input = poly(&[
            (-0.75, 0.75),
            (-0.75, -2.0),
            (0.75, -2.0),
            (0.75, 0.75),
            (0.25, 0.75),
            (0.25, -1.5),
            (-0.25, -1.5),
            (-0.25, 0.75),
        ]);
        let expected = poly(&[
            (-0.75, 0.75),
            (-0.75, -1.0),
            (0.75, -1.0),
            (0.75, 0.75),
            (0.25, 0.75),
            (0.25, -1.0),
            (-0.25, -1.0),
            (-0.25, 0.75),
        ]);

        assert_eq!(clip_polygon(&input, WINDOW), expected);
    }

    /// Non-axis-aligned (diagonal) subject polygons. Unlike the rectangle cases
    /// above, clipping a slanted edge exercises the actual intersection
    /// interpolation in `get_intersection`, and the vertex count changes as
    /// corners are cut off and new intersection points are inserted
    /// (3-gon -> 4-gon, 4-gon -> 5-gon, 4-gon -> 6-gon).
    #[rstest]
    #[case::triangle_to_quadrilateral(
        &[(0.0, 0.0), (-0.5, 2.0), (0.5, 0.0)],
        &[(0.0, 0.0), (-0.25, 1.0), (0.0, 1.0), (0.5, 0.0)]
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
