#![allow(dead_code)]

pub fn compress<T>(items: &Vec<T>) -> Option<&Vec<T>> {
    match items.len() {
        0 => None,
        _ => Some(items),
    }
}

pub fn decompress<T: Clone>(items: Option<&Vec<T>>) -> Vec<T> {
    match items {
        Some(items) => items.to_vec(),
        None => Vec::new(),
    }
}

pub fn map_compress<T, U>(items: &Vec<T>, f: fn(&T) -> U) -> Option<Vec<U>> {
    match items.len() {
        0 => None,
        _ => Some(items.iter().map(f).collect()),
    }
}

pub fn map_decompress<T, U>(items: Option<&Vec<T>>, f: fn(&T) -> U) -> Vec<U> {
    match items {
        Some(items) => items.iter().map(f).collect(),
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    mod compress {
        use crate::utils::vec::compress;

        #[test]
        fn given_empty_array_it_returns_none() {
            let array: Vec<u32> = vec![];

            let compressed = compress(&array);

            assert!(compressed.is_none());
        }

        #[test]
        fn given_non_empty_array_it_returns_some_array() {
            let array: Vec<u32> = vec![1, 2, 3];

            let compressed = compress(&array);

            assert!(compressed.is_some() && compressed.unwrap().eq(&array));
        }
    }

    mod map_compress {
        use crate::utils::vec::map_compress;

        #[test]
        fn given_empty_array_it_returns_none() {
            let array: Vec<u32> = vec![];
            let function: fn(&u32) -> u32 = |item| item + 1;

            let compressed = map_compress(&array, function);

            assert!(compressed.is_none());
        }

        #[test]
        fn given_non_empty_array_it_returns_some_array_mapped() {
            let array: Vec<u32> = vec![1, 2, 3];
            let function: fn(&u32) -> u32 = |item| item + 1;

            let compressed = map_compress(&array, function);

            let expected_array: Vec<u32> = vec![2, 3, 4];
            assert!(compressed.is_some() && compressed.unwrap().eq(&expected_array));
        }
    }

    mod decompress {
        use crate::utils::vec::decompress;

        #[test]
        fn given_none_it_returns_empty_array() {
            let items: Option<&Vec<u32>> = None;

            let decompressed = decompress(items);

            assert_eq!(decompressed.len(), 0);
        }

        #[test]
        fn given_some_array_it_returns_contained_array() {
            let array: Vec<u32> = vec![1, 2, 3];
            let items = Some(&array);

            let decompressed = decompress(items);

            assert_eq!(decompressed, array);
        }
    }

    mod map_decompress {
        use crate::utils::vec::map_decompress;

        #[test]
        fn given_none_it_returns_empty_array() {
            let items: Option<&Vec<u32>> = None;
            let function: fn(&u32) -> u32 = |item| item + 1;

            let decompressed = map_decompress(items, function);

            assert_eq!(decompressed.len(), 0);
        }

        #[test]
        fn given_some_array_it_returns_contained_array_mapped() {
            let array: Vec<u32> = vec![1, 2, 3];
            let function: fn(&u32) -> u32 = |item| item + 1;
            let items = Some(&array);

            let decompressed = map_decompress(items, function);

            let expected_array: Vec<u32> = vec![2, 3, 4];
            assert_eq!(decompressed, expected_array);
        }
    }
}
