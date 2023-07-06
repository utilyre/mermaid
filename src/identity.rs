pub trait AdditiveIdentity {
    fn additive_identity() -> Self;
}

macro_rules! impl_additive_identity {
    ($t: ty, $v: expr) => {
        impl AdditiveIdentity for $t {
            fn additive_identity() -> Self {
                $v
            }
        }
    };
}

impl_additive_identity!(u8, 0);
impl_additive_identity!(u16, 0);
impl_additive_identity!(u32, 0);
impl_additive_identity!(u64, 0);
impl_additive_identity!(usize, 0);
impl_additive_identity!(i8, 0);
impl_additive_identity!(i16, 0);
impl_additive_identity!(i32, 0);
impl_additive_identity!(i64, 0);
impl_additive_identity!(isize, 0);
impl_additive_identity!(f32, 0.0);
impl_additive_identity!(f64, 0.0);

pub trait MultiplicativeIdentity {
    fn multiplicative_identity() -> Self;
}

macro_rules! impl_multiplicative_identity {
    ($t: ty, $v: expr) => {
        impl MultiplicativeIdentity for $t {
            fn multiplicative_identity() -> Self {
                $v
            }
        }
    };
}

impl_multiplicative_identity!(u8, 1);
impl_multiplicative_identity!(u16, 1);
impl_multiplicative_identity!(u32, 1);
impl_multiplicative_identity!(u64, 1);
impl_multiplicative_identity!(usize, 1);
impl_multiplicative_identity!(i8, 1);
impl_multiplicative_identity!(i16, 1);
impl_multiplicative_identity!(i32, 1);
impl_multiplicative_identity!(i64, 1);
impl_multiplicative_identity!(isize, 1);
impl_multiplicative_identity!(f32, 1.0);
impl_multiplicative_identity!(f64, 1.0);
