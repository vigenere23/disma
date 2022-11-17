pub trait OptionEq<U> {
    fn option_eq(&self, other: &Option<U>) -> bool;
}

impl<T, U> OptionEq<U> for Option<T>
where
    T: PartialEq<U>,
{
    fn option_eq(&self, other: &Option<U>) -> bool {
        match (self, other) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(t), Some(u)) => t == u,
        }
    }
}
