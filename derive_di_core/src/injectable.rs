/// Injectable trait which used for `Default` trait implementation of derive `Container` macro.
pub trait Injectable: Sized {
    fn get_service() -> Self;
}
