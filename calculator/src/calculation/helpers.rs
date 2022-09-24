use crate::shared::calculation_precision::UnsignedValuePrecision;

// pub fn lcm(a: RationalNumberPartDepth, b: RationalNumberPartDepth) -> RationalNumberPartDepth {
//     let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };
//     let mut rem: RationalNumberPartDepth = x % y;

//     while rem != 0 {
//         x = y;
//         y = rem;
//         rem = x % y;
//     }

//     let lcm = a * b / y;
//     lcm
// }

pub fn hcf(mut a: UnsignedValuePrecision, mut b: UnsignedValuePrecision) -> UnsignedValuePrecision {
    while a != b && a > 0 && b > 0 {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    if a == 0 {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hcf_works() {
        assert_eq!(hcf(5, 7), 1);
        assert_eq!(hcf(50, 60), 10);
    }
}
