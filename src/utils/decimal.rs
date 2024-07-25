// TODO if f128 ever gets real support, add that in

/// Serializer for floating-point numbers that writes and reads its data
/// based on the IEEE-754 schema. Numbers are represented as a binary chain
/// of a certain length, and this chain is divided into three parts: one bit
/// is dedicated to the sign of the number, a certain amount are dedicated
/// to the *exponent*, and the rest to the *mantissa* (or *significand*). The
/// idea is to express numbers in a sort of scientific notation, written in
/// base 2 instead of 10; thus, numbers can be written as
/// `sign * significand * 2 ** exponent`; 3 can be written as `1.5 * 2 ** 1`,
/// -0.5 can be written as `-1 * 2 ** -1`, and so on. Javascript's numbers are
/// all 64-bit IEEE-754 floating-point numbers; thus, generating a serializer
/// using more than 64 bits will not yield any gain. Neither `-NaN` nor `-0` can be
/// serialized or deserialized correctly because Javascript will convert both to
/// their unsigned versions.
///
/// Note that the choice of exponent width has an impact on the range of numbers
/// that can be represented, as well as the accuracy thereof; the number of
/// binary chains possible does not change, but their mappings do. Larger exponent
/// widths provide logarithmic-like scaling; encodings will tend to bunch up around
/// 0, and will spread out to very large numbers; however, in doing so, precision
/// is very quickly lost. (as an extreme example, an 8-bit float with 6 exponent
/// bits cannot represent the number `5`, but can represent both `4.656612873077393e-10`
/// and `3221225472`) Smaller exponent widths lead to a very small interval
/// of expressible values, and to more binary chains giving `NaN`.
///
/// Also note that this serializer can be *unsigned*, which effectively restricts the
/// representable range to only positive values, but doubles its precision in that range.
/// Passing a negative floating point value to an unsigned floating point serializer
/// simply results in the sign being _ignored_; no underflow is performed, meaning that
/// an unsigned serializer will treat `-3.2` as if it were `3.2`. Such serializers obviously
/// have no sign bit.
///
/// Lastly, take note of the language's capacities with regards to expressing floating-point
/// numbers, because they will dictate what encoding schemas are viable—for example, a 32-bit
/// float with 11 exponent bits (as opposed to the normal 8) will have its maximum value not
/// be expressible with a regular IEEE double-precision float.
///
/// Space complexity is simply what the user of this class indicates it to be via the
/// bit count.
#[derive(Copy, Clone, Debug)]
pub struct DecimalSerializer {
    // given by user
    /// How many bits this floating-point number occupies
    bits: u8,

    /// How many bits are dedicated to the exponent
    exponent_bits: u8,

    /// Whether this serializer accepts negative values or not. Passing a negative value
    /// to an unsigned serializer simply serializes it as if it had been positive; no
    /// underflow is performed
    signed: bool,

    // calculated
    /// How many bits are dedicated to the mantissa (or significand)
    mantissa_width: u8,

    /// Bit field that can be used as a mask to extract a number's sign. Since
    /// the sign bit is always the left-most bit, this is always equal to
    /// `2 ** (this.bits - 1)`. In unsigned mode, it's equal to 0
    sign_mask: u128,

    /// Bit field that can be used as a mask to extract a number's exponent
    exponent_mask: u128,

    /// Bit field that can be used as a mask to extract a number's mantissa
    mantissa_mask: u128,

    /// The value below which numbers are considered "subnormal"
    subnormal_threshold: i128,

    /// IEEE-754 exponents are signed integers, divided evenly between positive
    /// and negative exponents; this bias is added during encoding and is
    /// subtracted during decoding, and allows to reach the negative exponents
    /// without having to resort to two's complement
    exponent_bias: u128,

    /// Numbers which force the exponent to its minimal value are called "subnormal";
    /// this constant holds the value of 2 raised to said exponent's minimal value
    subnormal_coeff: f64,

    /// IEEE-754 states that an exponent filled with 1's is to be treated as an
    /// indicator of a special value: if the mantissa is 0, this value is ±Infinity
    /// (depending on the sign bit); otherwise, it is ±NaN (once again depending on
    /// the sign bit). This constant holds the value of said exponent, with no shifting
    /// applied (equal to `(2 ** this.exponentBits) - 1`).
    special_exp: u128,

    /// Apart from subnormal numbers, all other numbers have a hidden leading 1 in
    /// their mantissa, which allows for the encoding of exactly twice as many values
    /// compared to if the hidden 1 was encoded explicitly. This constant "holds" said
    /// bit (more precisely, it is the representation of where this hidden bit would be
    /// were it to be encoded along with the rest, ignoring spacing constraints).
    hidden_bit: u128,

    /// The value above which all numbers will be encoded as `Infinity`. Note that this
    /// is not necessarily the maximum finite value representable by this encoding scheme;
    /// it simply guarantees that any value *above it* will result in `Infinity`.
    max_value: f64,

    /// Whether this serializer accepts negative values or not. Passing a negative value
    /// to an unsigned serializer simply serializes it as if it had been positive; no
    /// underflow is performed
    min_value: f64,
}

impl DecimalSerializer {
    /// Creates a new signed DecimalSerializer
    pub fn new(bits: u8, exponent_bits: u8) -> DecimalSerializer {
        Self::new_sign(bits, exponent_bits, true)
    }

    /// Creates a new unsigned DecimalSerializer
    pub fn new_unsigned(bits: u8, exponent_bits: u8) -> DecimalSerializer {
        Self::new_sign(bits, exponent_bits, false)
    }

    fn new_sign(bits: u8, exponent_bits: u8, signed: bool) -> DecimalSerializer {
        assert!(
            bits <= 128,
            "Number width cannot exceed 128 bits (although surpassing 64 isn't recommended)"
        );

        assert!(
            exponent_bits < bits,
            "Given exponent width ({}) cannot be greater than the number's width ({})",
            exponent_bits,
            bits
        );

        /*
            note: quantities may not be related in the way
            their variable names might imply; a choice was
            made to prioritize avoiding unneeded recalculations
            by extracting some expressions to variables and
            reusing already-calculated quantities

            for example, the mantissa's mask isn't really defined
            as being "the hidden bit minus 1"—such a "hidden bit"
            quantity doesn't even really exist, and is but a
            convenient way to avoid recalculating 2 ** mantissaBits
            continuously
        */

        let mantissa_width = bits - exponent_bits - (signed as u8);
        let special_exp = 2_u128.pow(exponent_bits as u32) - 1;
        let hidden_bit = 2_u128.pow(mantissa_width as u32);
        let one_shifted_ebm1 = 2_u32.pow((exponent_bits - 1) as u32);
        let exponent_bias = (one_shifted_ebm1 - 1) as u128;
        let subnormal_exp = 1 - exponent_bias as i32;

        DecimalSerializer {
            bits,
            exponent_bits,
            signed,

            mantissa_width,
            sign_mask: if signed {
                2_u128.pow((bits - 1) as u32)
            } else {
                0
            },
            exponent_mask: special_exp * hidden_bit,
            mantissa_mask: hidden_bit - 1,
            subnormal_threshold: -(exponent_bias as i128),
            exponent_bias,
            subnormal_coeff: 2_f64.powi(subnormal_exp),
            special_exp,
            hidden_bit,
            max_value: 2_f64.powi(one_shifted_ebm1 as i32),
            min_value: 2_f64.powi(subnormal_exp - mantissa_width as i32),
        }
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    /// Converts a given floating point value to its binary representation in
    /// accordance to this encoder's configuration
    /// - `(param)` `value`: The value to encode. `±Infinity` and `±NaN` are accepted (
    ///              although Javascript cannot produce `-NaN`)
    /// - `(returns)`: An integer whose binary representation represents, to the best
    /// of the encoder's ability, the given binary value
    pub fn encode_ieee<T: Into<f64>>(&self, value: T) -> u128 {
        let val: f64 = value.into();
        let is_nan = val.is_nan();
        let whole_bits = val.log2().floor() as i32;
        let is_subnormal = whole_bits as i128 <= self.subnormal_threshold;

        // builtins don't handle NaN nor subnormals correctly (wtf?), so we bail out for those
        let can_use_builtin = !is_nan && !is_subnormal && self.signed;

        match (self.bits, self.exponent_bits) {
            // shortcut for builtins
            (32, 8) if can_use_builtin => f32::to_bits(val as f32) as u128,
            (64, 11) if can_use_builtin => f64::to_bits(val) as u128,

            // default algorithm
            _ => {
                let abs = val.abs();
                if abs < self.min_value {
                    return 0;
                }

                let sign = if self.signed && val < 0.0 {
                    self.sign_mask
                } else {
                    0
                };

                if is_nan || abs > self.max_value {
                    return self.exponent_mask | (is_nan as u128) | sign;
                }

                let exponent = if is_subnormal {
                    0
                } else {
                    (whole_bits as i128 + self.exponent_bias as i128) as u128 * self.hidden_bit
                };
                let mantissa = Self::to_inverse_binary(
                    abs / (if is_subnormal {
                        self.subnormal_coeff
                    } else {
                        2_f64.powi(whole_bits)
                    }),
                    self.mantissa_width,
                );

                sign + exponent + mantissa
            }
        }
    }

    /// Converts the IEEE-754 representation of a floating-point number
    /// into said float, according to this encoder's configuration
    /// `(param)` `value`: The binary representation of the number to decode
    /// `returns` The floating point value associated with the encoding, may
    /// be `±Infinity` or `±NaN` (although Javascript cannot represent `-NaN`)
    pub fn decode_ieee<T: Into<u128>>(&self, value: T) -> f64 {
        // it might not even be worth doing this check to use the builtins…
        let val: u128 = value.into();

        let exponent = (val & self.exponent_mask) >> self.mantissa_width;
        let raw_value = (val & self.mantissa_mask) as f64 / self.hidden_bit as f64;

        // again, the builtins don't handle NaN nor subnormals for some reason
        let is_special_exp = exponent == self.special_exp;
        let zero_mantissa = raw_value == 0.0;
        let is_subnormal = exponent == 0;
        let can_use_builtins = (!is_special_exp || !zero_mantissa) && !is_subnormal && self.signed;

        match (self.bits, self.exponent_bits) {
            // shortcut for builtins
            (32, 8) if can_use_builtins => f32::from_bits((val & 0xFFFFFFFF) as u32) as f64,
            (64, 11) if can_use_builtins => f64::from_bits((val & 0xFFFFFFFFFFFFFFFF) as u64),

            // default algorithm
            _ => {
                let sign = if self.signed && val & self.sign_mask != 0 {
                    -1
                } else {
                    1
                };

                sign as f64
                    * match true {
                        _ if is_subnormal => self.subnormal_coeff * raw_value,
                        _ if is_special_exp => {
                            if zero_mantissa {
                                f64::INFINITY
                            } else {
                                f64::NAN
                            }
                        }
                        _ => {
                            2_f64.powi(exponent as i32 - self.exponent_bias as i32)
                                * (raw_value + 1.0)
                        }
                    }
            }
        }
    }

    /// Returns the binary expansion of a number less than 1, with the
    /// returned interpretation being an unsigned integer.
    ///
    /// For example, 0.75 can be written as `0.11` in binary; if one
    /// were to call this method as `_toInverseBinary(0.75, 3)`, they
    /// would receive `110` as a result, which is 6
    ///
    /// - `(param)` `value`: The value to convert
    /// - `(param)` `bits`:  How many bits the conversion is allowed to span
    /// - `returns`: An unsigned integer whose binary form is the result of
    /// removing the decimal point from the given value's binary form,
    /// approximated to the granularity specified by `bits`.
    fn to_inverse_binary(mut value: f64, mut bits: u8) -> u128 {
        let mut res = 0;

        const LAST_BIT_ROUND_THRESHOLD: f64 = 2_f64 / 3_f64;

        while bits > 0 {
            bits -= 1;
            value = (value % 1.0) * 2.0;

            if value >= 1.0 || (bits == 0 && value >= LAST_BIT_ROUND_THRESHOLD) {
                res |= 2_u128.pow(bits as u32);
            }
        }

        res
    }
}