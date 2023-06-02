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

#[cfg(test)]
mod tests {
    use super::OptionEq;

    fn assert_eq<T: PartialEq>(value1: &Option<T>, value2: &Option<T>) {
        assert!(value1.option_eq(value2));
        assert!(value2.option_eq(value1));
    }

    fn assert_ne<T: PartialEq>(value1: &Option<T>, value2: &Option<T>) {
        assert!(!value1.option_eq(value2));
        assert!(!value2.option_eq(value1));
    }

    #[test]
    fn given_none_options_they_are_equal() {
        let value1: Option<u32> = None;
        let value2: Option<u32> = None;

        assert_eq(&value1, &value2);
    }

    #[test]
    fn given_none_and_some_options_they_are_not_equal() {
        let value1 = Some(32);
        let value2: Option<u32> = None;

        assert_ne(&value1, &value2);
    }

    #[test]
    fn given_some_options_with_different_values_they_are_not_equal() {
        let value1 = Some(32);
        let value2 = Some(33);

        assert_ne(&value1, &value2);
    }

    #[test]
    fn given_some_options_with_same_values_they_are_equal() {
        let value1 = Some(32);
        let value2 = Some(32);

        assert_eq(&value1, &value2);
    }
}
