pub trait Injectable: Sized {
    fn get_service() -> Self;
}
