use super::vectors::Vec2D;

pub mod consts {
    pub use std::f64::consts::{FRAC_PI_2 as HALF_PI, PI, TAU};
}

pub struct CollisionRecord {
    pub collided: bool,
    pub distance: f64,
}

pub struct CollisionResponse {
    pub dir: Vec2D,
    pub pen: f64,
}

pub struct IntersectionResponse {
    pub point: Vec2D,
    pub normal: Vec2D,
}

pub mod numeric {
    pub fn get_sign(number: f64, inverse: bool) -> i8 {
        if inverse {
            return if number > 0.0 { -1 } else { 1 };
        } else {
            return if number > 0.0 { 1 } else { -1 };
        }
    }

    /// Works like regular modulo, but negative numbers cycle back around: hence,
    /// `-1 % 4` gives `3` and not `-1`
    /// ## Parameters
    /// - `a`: The dividend
    /// - `n`: The divisor
    pub fn abs_mod(a: f64, n: f64) -> f64 {
        if a >= 0.0 {
            a % n
        } else {
            (a % n + n) % n
        }
    }
    /// Interpolate between two values
    /// ## Parameters
    /// - `start`: The start value
    /// - `end`: The end value
    /// - `interp_factor`: The interpolation factor
    /// ## Returns
    /// A number corresponding to the linear interpolation
    /// between `a` and `b` at factor `interpFactor`
    pub fn lerp(start: f64, end: f64, interp_factor: f64) -> f64 {
        return start * (1.0 - interp_factor) + end * interp_factor;
    }
    /// Limit a number to given bounds
    /// ## Parameters
    /// - `value`: Number to limit
    /// - `min`: Lower bound
    /// - `max`: Upper bound
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        if value < max {
            if value > min {
                return value;
            }
            return min;
        }
        max
    }
    /// Uses linear interpolation in each range to
    /// remap a number from one range to another.
    pub fn remap(value: f64, min0: f64, max0: f64, min1: f64, max1: f64) -> f64 {
        self::lerp(
            min1,
            max1,
            self::clamp((value - min0) / (max0 - min0), 0.0, 1.0),
        )
    }
}

pub mod angle {
    use crate::typings::Orientation;

    use super::{super::vectors::Vec2D, consts::*, numeric};

    /// Draws a line between two points and returns that line's angle
    /// ## Parameters
    /// - `a`: The first point, used as the head of the vector
    /// - `b`: The second point, used as the tail of the vector
    /// ## Returns
    /// The angle, in radians, of the line going from b to a
    pub fn between_points(a: &Vec2D, b: &Vec2D) -> f64 {
        (a.y - b.y).atan2(a.x - b.x)
    }
    /// Normalize an angle to between -π and π
    /// ## Parameters
    /// - `radians`: The angle, in radians
    pub fn normalize(radians: f64) -> f64 {
        numeric::abs_mod(radians - PI, TAU) - PI
    }
    /// Find the smallest difference between two angles (in radians)
    pub fn minimize(start: f64, end: f64) -> f64 {
        numeric::abs_mod(end - start + PI, TAU) - PI
    }
    /// Degrees to radians
    pub fn deg_to_rad(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }
    /// Radians to degrees
    pub fn rad_to_deg(radians: f64) -> f64 {
        radians / PI * 180.0
    }

    pub fn orientation_to_rotation(orientation: Orientation) -> f64 {
        -normalize((orientation as u8 as f64) * HALF_PI)
    }
}

pub mod geometry {
    use super::Vec2D;
    use crate::typings::Orientation;

    pub struct Circle {
        pub center: Vec2D,
        pub radius: f64,
    }

    pub struct Rectangle {
        pub min: Vec2D,
        pub max: Vec2D,
    }

    impl Rectangle {
        /// Translates this rectangle by a position.
        /// Mutates the original object, returns mutable reference to self for chaining.
        pub fn translate(&mut self, pos: Vec2D) -> &mut Self {
            self.min = self.min + pos;
            self.max = self.max + pos;
            self
        }

        /// Scale a rectangle by a factor.
        /// Mutates the original object, returns mutable reference to self for chaining.
        pub fn scale(&mut self, scale: f64) -> &mut Self {
            self.min = self.min * scale;
            self.max = self.max * scale;
            self
        }

        /// Since Rectangle itself is orientation-agnostic and axis-aligned,
        /// rotating 90 degrees is the only rotation necessary.
        /// This function rotates the rectangle about its center.
        /// Mutates the original object, returns mutable reference to self for chaining.
        pub fn rotate_90(&mut self) -> &mut Self {
            let center: Vec2D = (self.min + self.max) * 0.5;
            let side1: f64 = center.x - self.min.x;
            let side2: f64 = center.y - self.min.y;
            self.min = Vec2D {
                x: center.x - side2,
                y: center.y - side1,
            };
            self.max = Vec2D {
                x: center.x + side2,
                y: center.y + side1,
            };
            self
        }

        /// Transform rectangle by position, scale, and orientation.
        /// Mutates the original object, returns mutable reference to self for chaining.
        pub fn transform(&mut self, pos: Vec2D, scale: f64, orientation: Orientation) -> &mut Self {
            self.translate(pos).scale(scale);
            if orientation as u8 % 2 == 1 {
                self.rotate_90();
            }
            self
        }
    }

    /// Calculate distance between two points
    /// ## Parameters
    /// - `a`: the first point
    /// - `b`: the second point
    pub fn distance(a: Vec2D, b: Vec2D) -> f64 {
        distance_squared(a, b).sqrt()
    }
    /// Calculate squared distance between two points
    /// ## Parameters
    /// - `a`: the first point
    /// - `b`: the second point
    pub fn distance_squared(a: Vec2D, b: Vec2D) -> f64 {
        (b.x - a.x).powi(2) + (b.y - a.y).powi(2)
    }
    /// Calculate area of a triangle whose vertices are the three points passed in
    /// ## Parameters
    /// - `a`: the first vertex
    /// - `b`: the second vertex
    /// - `c`: the third vertex
    pub fn signed_tri_area(a: Vec2D, b: Vec2D, c: Vec2D) -> f64 {
        (a.x - c.x) * (b.y - c.y) - (a.y - c.y) * (b.x - c.x)
    }
}

pub mod intersections {
    use super::numeric::{clamp, get_sign};
    use super::Vec2D;
    use super::CollisionResponse;

    /// Calculate the intersection between two circles
    /// ## Parameters
    /// - `center_a`: Center of the first circle
    /// - `radius_a`: Radius of the first circle
    /// - `center_b`: Center of the second circle
    /// - `radius_b`: Radius of the second circle
    /// ## Returns
    /// An `Option` containing a `CollisionResponse` if the circles intersect, otherwise `None`
    pub fn circles(center_a: Vec2D, radius_a: f64, center_b: Vec2D, radius_b: f64) -> Option<CollisionResponse> {
        let radius = radius_a + radius_b;
        let p1 = center_b - center_a;
        let dist_sqr = Vec2D::squared_length(p1);

        if dist_sqr < radius * radius {
            Some(CollisionResponse {
                dir: Vec2D::normalize(p1, None),
                pen: radius - f64::sqrt(dist_sqr),
            })
        } else {
            None
        }
    }

    pub fn rect_circle(min: Vec2D, max: Vec2D, pos: Vec2D, radius: f64) -> Option<CollisionResponse> {
        if min.x <=pos.x && pos.x <= max.x && min.y <=pos.y && pos.y <= max.y {
            let half_dimension: Vec2D = Vec2D::scale(max - min, 0.5);
            let p = pos - (min + half_dimension);
            let xp = f64::abs(p.x) - half_dimension.x - radius;
            let yp = f64::abs(p.y) - half_dimension.y - radius;

            if xp > yp {
                return Some(CollisionResponse {
                    dir: Vec2D::new(f64::from(get_sign(p.x, false)), 0.0),
                    pen: -xp
                });
            } else {
                return Some(CollisionResponse {
                    dir: Vec2D::new(0.0, f64::from(get_sign(p.y, false))),
                    pen: -yp
                });
            }
        }

        let dir = Vec2D::new(clamp(pos.x, min.x, max.x), clamp(pos.y, min.y, max.y)) - pos;
        let dist_sqrd = Vec2D::squared_length(dir);

        if dist_sqrd < radius * radius {
            return Some(CollisionResponse {
                dir: Vec2D::normalize(dir, None),
                pen: radius - f64::sqrt(dist_sqrd)
            });
        } else {
            return None;
        }
    }
}

pub mod collisions {
    use super::numeric;
    use super::Vec2D;

    /// Check for collision between two circles.
    ///
    /// Determines if two circles defined by their centers and radii collide with each other.
    ///
    /// ## Parameters
    /// - `center_a`: The center of the first circle
    /// - `radius_a`: The radius of the first circle
    /// - `center_b`: The center of the second circle
    /// - `radius_b`: The radius of the second circle
    ///
    /// ## Returns
    /// Returns `true` if the circles collide, `false` otherwise.
    pub fn check_circles(center_a: Vec2D, radius_a: f64, center_b: Vec2D, radius_b: f64) -> bool {
        let rad_sum = radius_a + radius_b;
        let center_x = center_a.x - center_b.x;
        let center_y = center_a.y - center_b.y;

        rad_sum * rad_sum > center_x * center_x + center_y * center_y
    }

    /// Check for collision between a rectangle and a circle.
    ///
    /// ## Parameters
    /// - `min`: The rectangle minimum position
    /// - `max`: The rectangle maximum position
    /// - `pos`: The center of the circle
    /// - `rad`: The radius of the circle
    ///
    /// ## Returns
    /// Returns `true` if the shapes collide, `false` otherwise.
    pub fn check_rects(min: Vec2D, max: Vec2D, pos: Vec2D, rad: f64) -> bool {
        let cpt = Vec2D {
            x: numeric::clamp(pos.x, min.x, max.x),
            y: numeric::clamp(pos.y, min.y, max.y),
        };

        let distance_x = pos.x - cpt.x;
        let distance_y = pos.y - cpt.y;
        let distance_squared = distance_x * distance_x + distance_y * distance_y;

        (distance_squared < rad * rad)
            || (pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y)
    }
}
