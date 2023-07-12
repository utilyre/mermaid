#[macro_use]
mod macros;

pub trait IdAdd {
    fn id_add() -> Self;
}

pub trait IdMul {
    fn id_mul() -> Self;
}

id_impl! {
    Self = u8,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = u16,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = u32,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = u64,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = usize,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = i8,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = i16,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = i32,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = i64,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = isize,
    id_add = 0,
    id_mul = 1,
}

id_impl! {
    Self = f32,
    id_add = 0.0,
    id_mul = 1.0,
}

id_impl! {
    Self = f64,
    id_add = 0.0,
    id_mul = 1.0,
}
