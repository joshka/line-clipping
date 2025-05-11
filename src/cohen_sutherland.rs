use bitflags::bitflags;

use crate::{LineSegment, Point, Window};

/// Implements the Cohen-Sutherland line clipping algorithm.
///
/// Returns the clipped line if the original line intersects the clipping window, or `None` if the
/// original line is completely outside the clipping window.
///
/// Reference: [Cohen-Sutherland algorithm](https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm)
///
/// The Cohen-Sutherland algorithm is a line clipping algorithm that divides the 2D plane into 9
/// regions and then determines the region in which the line lies. If the line lies completely
/// outside the clipping window, it is rejected. If the line lies completely inside the clipping
/// window, it is accepted. If the line lies partially inside the clipping window, it is clipped.
///
/// The regions are defined as follows:
///
/// 1001 | 1000 | 1010
/// -----|------|-----
/// 0001 | 0000 | 0010
/// -----|------|-----
/// 0101 | 0100 | 0110
///
/// The algorithm works as follows:
///
/// 1. Determine the region in which the line's starting point lies.
/// 2. Determine the region in which the line's ending point lies.
/// 3. If both points lie in region 0000, the line is completely inside the clipping window and
///    should be accepted.
/// 4. If both points lie in the same region that is not 0000, the line is completely outside the
///    clipping window and should be rejected.
/// 5. If the points lie in different regions, the line is partially inside the clipping window and
///    should be clipped.
/// 6. Clip the line using the Cohen-Sutherland algorithm.
/// 7. Repeat the process for the clipped line.
///
/// The Cohen-Sutherland algorithm is commonly used in computer graphics to clip lines against a
/// rectangular window.
///
/// # Examples
///
/// ```
/// use line_clipping::cohen_sutherland::clip_line;
/// use line_clipping::{LineSegment, Point, Window};
///
/// let line = clip_line(
///     LineSegment::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0)),
///     Window::new(1.0, 9.0, 1.0, 9.0),
/// );
///
/// assert_eq!(
///     line,
///     Some(LineSegment::new(Point::new(1.0, 1.0), Point::new(9.0, 9.0)))
/// );
/// ```
pub fn clip_line(mut line: LineSegment, window: Window) -> Option<LineSegment> {
    let mut region_1 = Region::from_point(line.p1, window);
    let mut region_2 = Region::from_point(line.p2, window);

    while region_1.is_outside() || region_2.is_outside() {
        if region_1.intersects(region_2) {
            // The line is completely outside the clipping window.
            return None;
        }
        if region_1.is_outside() {
            line.p1 = calculate_intersection(line.p1, line.p2, region_1, window);
            region_1 = Region::from_point(line.p1, window);
        } else {
            line.p2 = calculate_intersection(line.p2, line.p1, region_2, window);
            region_2 = Region::from_point(line.p2, window);
        };
    }

    Some(line)
}

fn calculate_intersection(p1: Point, p2: Point, region: Region, window: Window) -> Point {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    if region.contains(Region::LEFT) {
        let y = p1.y + (window.x_min - p1.x) * dy / dx;
        Point::new(window.x_min, y)
    } else if region.contains(Region::RIGHT) {
        let y = p1.y + (window.x_max - p1.x) * dy / dx;
        Point::new(window.x_max, y)
    } else if region.contains(Region::BOTTOM) {
        let x = p1.x + (window.y_min - p1.y) * dx / dy;
        Point::new(x, window.y_min)
    } else if region.contains(Region::TOP) {
        let x = p1.x + (window.y_max - p1.y) * dx / dy;
        Point::new(x, window.y_max)
    } else {
        p1
    }
}

bitflags! {
    /// Represents the regions in the Cohen-Sutherland algorithm.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Region: u8 {
        const LEFT = 0b0001;
        const RIGHT = 0b0010;
        const BOTTOM = 0b0100;
        const TOP = 0b1000;
    }
}

impl Region {
    const fn is_outside(self) -> bool {
        !self.is_empty()
    }

    /// Determines the region in which a point lies.
    fn from_point(point: Point, window: Window) -> Self {
        let mut region = Region::empty();
        if point.x < window.x_min {
            region |= Region::LEFT;
        } else if point.x > window.x_max {
            region |= Region::RIGHT;
        }
        if point.y < window.y_min {
            region |= Region::BOTTOM;
        } else if point.y > window.y_max {
            region |= Region::TOP;
        }
        region
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completely_inside() {
        let line = LineSegment::new(Point::new(2.0, 2.0), Point::new(8.0, 8.0));
        let window = Window::new(1.0, 9.0, 1.0, 9.0);
        let expected = LineSegment::new(Point::new(2.0, 2.0), Point::new(8.0, 8.0));
        assert_eq!(clip_line(line, window), Some(expected));
    }

    #[test]
    fn completely_outside() {
        let line = LineSegment::new(Point::new(-1.0, -1.0), Point::new(-5.0, -5.0));
        let window = Window::new(1.0, 9.0, 1.0, 9.0);
        assert_eq!(clip_line(line, window), None);
    }

    #[test]
    fn partially_inside() {
        let line = LineSegment::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0));
        let window = Window::new(1.0, 9.0, 1.0, 9.0);
        let expected = LineSegment::new(Point::new(1.0, 1.0), Point::new(9.0, 9.0));
        assert_eq!(clip_line(line, window), Some(expected));
    }

    #[test]
    fn vertical() {
        let line = LineSegment::new(Point::new(5.0, 0.0), Point::new(5.0, 10.0));
        let window = Window::new(1.0, 9.0, 1.0, 9.0);
        let expected = LineSegment::new(Point::new(5.0, 1.0), Point::new(5.0, 9.0));
        assert_eq!(clip_line(line, window), Some(expected));
    }

    #[test]
    fn horizontal() {
        let line = LineSegment::new(Point::new(0.0, 5.0), Point::new(10.0, 5.0));
        let window = Window::new(1.0, 9.0, 1.0, 9.0);
        let expected = LineSegment::new(Point::new(1.0, 5.0), Point::new(9.0, 5.0));
        assert_eq!(clip_line(line, window), Some(expected));
    }

    #[test]
    fn diagonal() {
        let line = LineSegment::new(Point::new(-5.0, -5.0), Point::new(15.0, 15.0));
        let window = Window::new(1.0, 9.0, 1.0, 9.0);
        let expected = LineSegment::new(Point::new(1.0, 1.0), Point::new(9.0, 9.0));
        assert_eq!(clip_line(line, window), Some(expected));
    }
}
