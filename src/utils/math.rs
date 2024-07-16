pub use std::f32::consts::{FRAC_PI_2 as HALF_PI, PI, TAU};

pub mod numeric {
    /// Works like regular modulo, but negative numbers cycle back around: hence,
    /// `-1 % 4` gives `3` and not `-1`
    /// ## Parameters
    /// - `a`: The dividend
    /// - `n`: The divisor
    pub fn abs_mod(a: f32, n: f32) -> f32 {
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
    pub fn lerp(start: f32, end: f32, interp_factor: f32) -> f32 {
        return start * (1.0 - interp_factor) + end * interp_factor;
    }
    /// Limit a number to given bounds
    /// ## Parameters
    /// - `value`: Number to limit
    /// - `min`: Lower bound
    /// - `max`: Upper bound
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
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
    pub fn remap(value: f32, min0: f32, max0: f32, min1: f32, max1: f32) -> f32 {
        self::lerp(
            min1,
            max1,
            self::clamp((value - min0) / (max0 - min0), 0.0, 1.0),
        )
    }
}

pub mod angle {
    use super::{*, super::vectors::Vec2D, numeric};

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
    pub fn normalize(radians: f32) -> f32 {
        numeric::abs_mod(radians - PI, TAU) - PI
    }
    /// Find the smallest difference between two angles (in radians)
    pub fn minimize(start: f32, end: f32) -> f32 {
        numeric::abs_mod(end - start + PI, TAU) - PI
    }
    /// Degrees to radians
    pub fn degToRad(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }
    /// Radians to degrees
    pub fn radToDeg(radians: f32) -> f32 {
        radians / PI * 180.0
    }
}

pub mod Geometry {

}

pub mod Collision {

}
