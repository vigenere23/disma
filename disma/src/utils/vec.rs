pub trait Compress
where
    Self: Sized,
{
    fn compress(self) -> Option<Self>;
}

impl<T> Compress for Vec<T> {
    fn compress(self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Compress;

    #[test]
    fn given_elements_in_vec_it_does_not_compress() {
        let vec = vec![1, 2, 3, 4];
        let compressed = vec.compress();

        assert!(compressed.is_some());
    }

    #[test]
    fn given_empty_vec_it_compresses_to_none() {
        let vec: Vec<u8> = vec![];
        let compressed = vec.compress();

        assert!(compressed.is_none());
    }
}
