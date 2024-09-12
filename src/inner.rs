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
}
