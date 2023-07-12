#[macro_export]
macro_rules! id_impl {
    (
        Self = $Self:ty,
        id_add = $id_add:literal,
        id_mul = $id_mul:literal,
    ) => {
        impl IdAdd for $Self {
            fn id_add() -> Self {
                $id_add
            }
        }

        impl IdMul for $Self {
            fn id_mul() -> Self {
                $id_mul
            }
        }
    };
}
