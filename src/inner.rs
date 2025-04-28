use num_traits::cast::AsPrimitive;
use num_traits::{Float, PrimInt, Unsigned};
use num_traits::bounds::{ConstLowerBounded, ConstUpperBounded};

pub trait ToFloat<F: Float + 'static> {
    fn to_float(self) -> F;
}


/// Represents a fractional number in the range [0.0, 1.0).
/// 
/// The represented value is intended to be calculated as `x / (2**N + 1)`, where `x` is the contained value and `N` is the number of bits in the unsigned int type.
struct I0E1<P: PrimInt + Unsigned>(P);

impl<F: Float + 'static, P: PrimInt + Unsigned + AsPrimitive<F>> ToFloat<F> for I0E1<P> {
    fn to_float(self) -> F {
        let den = P::max_value().as_() + F::one();
        let num = self.0.as_();

        num / den
    }
}

impl<P: PrimInt + Unsigned + ConstLowerBounded> ConstLowerBounded for I0E1<P> {
    const MIN: Self = Self(P::MIN);
}

impl<P: PrimInt + Unsigned + ConstUpperBounded> ConstUpperBounded for I0E1<P> {
    const MAX: Self = Self(P::MAX);
}

/// Represents a fractional number in the range (0.0, 1.0].
/// 
/// The represented value is intended to be calculated as `(x + 1) / (2**N + 1)`, where `x` is the contained value and `N` is the number of bits in the unsigned int type.
struct E0I1<P: PrimInt + Unsigned>(P);

impl<F: Float + 'static, P: PrimInt + Unsigned + AsPrimitive<F>> ToFloat<F> for E0I1<P> {
    fn to_float(self) -> F {
        let den = P::max_value().as_() + F::one();
        let num = self.0.as_() + F::one();

        num / den
    }
}

impl<P: PrimInt + Unsigned + ConstLowerBounded> ConstLowerBounded for E0I1<P> {
    const MIN: Self = Self(P::MIN);
}

impl<P: PrimInt + Unsigned + ConstUpperBounded> ConstUpperBounded for E0I1<P> {
    const MAX: Self = Self(P::MAX);
}

/// Represents a fractional number in the range [0.0, 1.0].
enum I0I1<P: PrimInt + Unsigned> {
    One,
    Frac(I0E1<P>),
}

impl<F: Float + 'static, P: PrimInt + Unsigned + AsPrimitive<F>> ToFloat<F> for I0I1<P> {
    fn to_float(self) -> F {
        match self {
            Self::One => F::one(),
            Self::Frac(p) => p.to_float(),
        }
    }
}

impl<P: PrimInt + Unsigned + ConstLowerBounded> ConstLowerBounded for I0I1<P> {
    const MIN: Self = Self::Frac(ConstLowerBounded::MIN);
}

impl<P: PrimInt + Unsigned + ConstUpperBounded> ConstUpperBounded for I0I1<P> {
    const MAX: Self = Self::One;
}

/// Represents a fractional number in the range [-1.0, +1.0].
enum IN1IP1<P: PrimInt + Unsigned> {
    Neg(E0I1<P>),
    Zero,
    Pos(E0I1<P>),
}

impl<F: Float + 'static, P: PrimInt + Unsigned + AsPrimitive<F>> ToFloat<F> for IN1IP1<P> {
    fn to_float(self) -> F {
        match self {
            Self::Pos(p) => p.to_float(),
            Self::Zero => F::zero(),
            Self::Neg(p) => -(p.to_float()),
        }
    }
}

impl<P: PrimInt + Unsigned + ConstUpperBounded> ConstLowerBounded for IN1IP1<P> {
    const MIN: Self = Self::Neg(ConstUpperBounded::MAX);
}

impl<P: PrimInt + Unsigned + ConstUpperBounded> ConstUpperBounded for IN1IP1<P> {
    const MAX: Self = Self::Pos(ConstUpperBounded::MAX);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn to_f32() {
//         let i = InnerIn::<u32>::Max;
//         assert_eq!(i.to_f32(), 1.0);

//         let i = InnerIn::Sub(0u32);
//         assert_eq!(i.to_f32(), 0.0);

//         let i = InnerIn::Sub(1u32 << (u32::BITS - 1));
//         assert_eq!(i.to_f32(), 0.5);

//         let i = InnerIn::Sub(1u32 << (u32::BITS - 2));
//         assert_eq!(i.to_f32(), 0.25);

//         let i = InnerIn::<u64>::Max;
//         assert_eq!(i.to_f32(), 1.0);

//         let i = InnerIn::Sub(0u64);
//         assert_eq!(i.to_f32(), 0.0);

//         let i = InnerIn::Sub(1u64 << (u64::BITS - 1));
//         assert_eq!(i.to_f32(), 0.5);

//         let i = InnerIn::Sub(1u64 << (u64::BITS - 2));
//         assert_eq!(i.to_f32(), 0.25);
//     }

//     #[test]
//     fn to_f64() {
//         let i = InnerIn::<u32>::Max;
//         assert_eq!(i.to_f64(), 1.0);

//         let i = InnerIn::Sub(0u32);
//         assert_eq!(i.to_f64(), 0.0);

//         let i = InnerIn::Sub(1u32 << (u32::BITS - 1));
//         assert_eq!(i.to_f64(), 0.5);

//         let i = InnerIn::Sub(1u32 << (u32::BITS - 2));
//         assert_eq!(i.to_f64(), 0.25);

//         let i = InnerIn::<u128>::Max;
//         assert_eq!(i.to_f64(), 1.0);

//         let i = InnerIn::Sub(0u128);
//         assert_eq!(i.to_f64(), 0.0);

//         let i = InnerIn::Sub(1u128 << (u128::BITS - 1));
//         assert_eq!(i.to_f64(), 0.5);

//         let i = InnerIn::Sub(1u128 << (u128::BITS - 2));
//         assert_eq!(i.to_f64(), 0.25);
//     }
// }
