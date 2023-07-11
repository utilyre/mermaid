pub trait IdAdd {
    fn id_add() -> Self;
}

macro_rules! impl_id_add {
    ($t: ty, $v: expr) => {
        impl IdAdd for $t {
            fn id_add() -> Self {
                $v
            }
        }
    };
}

impl_id_add!(u8, 0);
impl_id_add!(u16, 0);
impl_id_add!(u32, 0);
impl_id_add!(u64, 0);
impl_id_add!(usize, 0);
impl_id_add!(i8, 0);
impl_id_add!(i16, 0);
impl_id_add!(i32, 0);
impl_id_add!(i64, 0);
impl_id_add!(isize, 0);
impl_id_add!(f32, 0.0);
impl_id_add!(f64, 0.0);

pub trait IdMul {
    fn id_mul() -> Self;
}

macro_rules! impl_id_mul {
    ($t: ty, $v: expr) => {
        impl IdMul for $t {
            fn id_mul() -> Self {
                $v
            }
        }
    };
}

impl_id_mul!(u8, 1);
impl_id_mul!(u16, 1);
impl_id_mul!(u32, 1);
impl_id_mul!(u64, 1);
impl_id_mul!(usize, 1);
impl_id_mul!(i8, 1);
impl_id_mul!(i16, 1);
impl_id_mul!(i32, 1);
impl_id_mul!(i64, 1);
impl_id_mul!(isize, 1);
impl_id_mul!(f32, 1.0);
impl_id_mul!(f64, 1.0);
