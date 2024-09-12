enum Inner {
    Max,
    Sub(u32),
}

impl Inner {
    fn to_f32(self) -> f32 {
        match self {
            Self::Max => 1.0,
            Self::Sub(0) => 0.0,
            Self::Sub(n) => {
                let den: f32 = (u32::MAX as f32) + 1.0;
                n as f32 / den
            }
        }
    }

    fn to_f64(self) -> f64 {
        match self {
            Self::Max => 1.0,
            Self::Sub(0) => 0.0,
            Self::Sub(n) => {
                let den: f64 = (u32::MAX as f64) + 1.0;
                n as f64 / den
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_f32() {
        let i = Inner::Max;
        assert_eq!(i.to_f32(), 1.0);

        let i = Inner::Sub(0);
        assert_eq!(i.to_f32(), 0.0);

        let i = Inner::Sub(1 << (u32::BITS - 1));
        assert_eq!(i.to_f32(), 0.5);

        let i = Inner::Sub(1 << (u32::BITS - 2));
        assert_eq!(i.to_f32(), 0.25);
    }

    #[test]
    fn to_f64() {
        let i = Inner::Max;
        assert_eq!(i.to_f64(), 1.0);

        let i = Inner::Sub(0);
        assert_eq!(i.to_f64(), 0.0);

        let i = Inner::Sub(1 << (u32::BITS - 1));
        assert_eq!(i.to_f64(), 0.5);

        let i = Inner::Sub(1 << (u32::BITS - 2));
        assert_eq!(i.to_f64(), 0.25);
    }
}
