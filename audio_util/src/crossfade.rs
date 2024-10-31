/**
converts from a linear crossfade between 0 and 1 to
an equal power crossfade between 0 and 1
 */
pub fn constant_power(linear_fade_strength: f32) -> f32 {
    let linear_fade_strength = linear_fade_strength.clamp(0., 1.);
    return f32::sqrt(linear_fade_strength);
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{approx_eq, assert_approx_eq};

    #[test]
    fn test_equal_power() {
        // center
        assert_approx_eq!(f32, constant_power(0.5), 0.70710677);
        // left
        assert_approx_eq!(f32, constant_power(0.0), 0.0);
        // right
        assert_approx_eq!(f32, constant_power(1.0), 1.0); 
        //value too high
        assert!(approx_eq!(f32, constant_power(100.0), 1.0));
        // value too low
        assert!(approx_eq!(f32, constant_power(-100.0), 0.0))
    }
}
