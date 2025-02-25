use try_reserve::error::{TryReserveError, TryReserveErrorKind};
use try_reserve::TryReserve;

// Implementation tests for all standard library types
#[cfg(not(feature = "no_std"))]
mod impl_tests {

    use super::*;
    use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::string::String;
    use std::vec::Vec;

    // Test implementation for Vec
    #[test]
    fn test_vec_impl() {
        let mut vec: Vec<i32> = Vec::new();
        assert!(vec.try_reserve(10).is_ok());

        let result = vec.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for String
    #[test]
    fn test_string_impl() {
        let mut string = String::new();
        assert!(string.try_reserve(10).is_ok());

        let result = string.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for HashMap
    #[test]
    fn test_hashmap_impl() {
        let mut map: HashMap<i32, i32> = HashMap::new();
        assert!(map.try_reserve(10).is_ok());

        let result = map.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for HashSet
    #[test]
    fn test_hashset_impl() {
        let mut set: HashSet<i32> = HashSet::new();
        assert!(set.try_reserve(10).is_ok());

        let result = set.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for VecDeque
    #[test]
    fn test_vecdeque_impl() {
        let mut deque: VecDeque<i32> = VecDeque::new();
        assert!(deque.try_reserve(10).is_ok());

        let result = deque.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for BinaryHeap
    #[test]
    fn test_binaryheap_impl() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.try_reserve(10).is_ok());

        let result = heap.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for OsString
    #[test]
    fn test_osstring_impl() {
        let mut os_string = OsString::new();
        assert!(os_string.try_reserve(10).is_ok());

        let result = os_string.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }

    // Test implementation for PathBuf
    #[test]
    fn test_pathbuf_impl() {
        let mut path_buf = PathBuf::new();
        assert!(path_buf.try_reserve(10).is_ok());

        let result = path_buf.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                _ => panic!("Expected CapacityOverflow error"),
            }
        }
    }
}

// Test error conversions
mod error_tests {
    use super::*;

    #[test]
    fn test_error_kind_conversion() {
        let error_kind = TryReserveErrorKind::CapacityOverflow;
        let error: TryReserveError = error_kind.into();

        assert!(matches!(
            error.kind(),
            TryReserveErrorKind::CapacityOverflow
        ));
    }

    #[test]
    fn test_layout_error_conversion() {
        use core::alloc::Layout;

        let layout_result = Layout::from_size_align(usize::MAX, 8);

        if let Err(layout_err) = layout_result {
            let error_kind = TryReserveErrorKind::from(layout_err);
            assert!(matches!(error_kind, TryReserveErrorKind::CapacityOverflow));

            let error: TryReserveError = error_kind.into();
            assert!(matches!(
                error.kind(),
                TryReserveErrorKind::CapacityOverflow
            ));
        } else {
            panic!("Expected layout error");
        }
    }

    #[cfg(not(feature = "no_std"))]
    #[test]
    fn test_std_error_conversion() {
        let mut vec: Vec<i32> = Vec::new();
        let std_result = vec.try_reserve(usize::MAX);

        if let Err(std_err) = std_result {
            let our_err = TryReserveError::from(std_err);
            assert!(matches!(
                our_err.kind(),
                TryReserveErrorKind::CapacityOverflow
            ));

            let _: std::collections::TryReserveError = our_err.into();
        } else {
            panic!("Expected std error");
        }
    }

    #[test]
    fn test_error_display() {
        let overflow_error = TryReserveErrorKind::CapacityOverflow;
        let error: TryReserveError = overflow_error.into();
        assert_eq!(
            error.to_string(),
            "memory allocation failed because the computed capacity exceeded the collection's maximum"
        );

        use core::alloc::Layout;
        let layout = Layout::from_size_align(16, 8).unwrap();
        let alloc_error = TryReserveErrorKind::AllocError {
            layout,
            non_exhaustive: (),
        };
        let error: TryReserveError = alloc_error.into();
        assert_eq!(
            error.to_string(),
            "memory allocation failed because the memory allocator returned an error"
        );
    }
}

// Test custom collection implementations
#[cfg(not(feature = "no_std"))]
mod custom_impl_tests {
    use super::*;

    // A simple custom mockup collection
    struct MyCollection<T> {
        data: Vec<T>,
    }

    impl<T> MyCollection<T> {
        fn new() -> Self {
            MyCollection { data: Vec::new() }
        }
    }

    impl<T> TryReserve for MyCollection<T> {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.data
                .try_reserve(additional)
                .map_err(TryReserveError::from)
        }
    }

    #[test]
    fn test_custom_collection() {
        let mut collection = MyCollection::<i32>::new();

        assert!(collection.try_reserve(10).is_ok());

        let result = collection.try_reserve(usize::MAX);
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(matches!(err.kind(), TryReserveErrorKind::CapacityOverflow));
        }
    }

    struct FixedSizeCollection<T> {
        data: Vec<T>,
        max_capacity: usize,
    }

    impl<T> FixedSizeCollection<T> {
        fn new(max_capacity: usize) -> Self {
            FixedSizeCollection {
                data: Vec::new(),
                max_capacity,
            }
        }
    }

    impl<T> TryReserve for FixedSizeCollection<T> {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            let new_capacity = self.data.len() + additional;

            if new_capacity > self.max_capacity {
                Err(TryReserveErrorKind::CapacityOverflow.into())
            } else {
                self.data
                    .try_reserve(additional)
                    .map_err(TryReserveError::from)
            }
        }
    }

    #[test]
    fn test_fixed_size_collection() {
        let mut collection = FixedSizeCollection::<i32>::new(50);

        assert!(collection.try_reserve(50).is_ok());

        assert!(collection.try_reserve(51).is_err());
    }
}

// Test edge cases
#[cfg(not(feature = "no_std"))]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_zero_reservation() {
        // Reserving 0 should always succeed
        let mut vec: Vec<i32> = Vec::new();
        assert!(vec.try_reserve(0).is_ok());

        let mut string = String::new();
        assert!(string.try_reserve(0).is_ok());

        let mut map: HashMap<i32, i32> = HashMap::new();
        assert!(map.try_reserve(0).is_ok());
    }

    #[test]
    fn test_with_preexisting_capacity() {
        // Create a vector with some initial capacity
        let mut vec: Vec<i32> = Vec::with_capacity(100);

        assert!(vec.try_reserve(50).is_ok());

        for i in 0..100 {
            vec.push(i);
        }

        assert!(vec.try_reserve(1).is_ok());
    }

    #[test]
    fn test_near_capacity_limit() {
        // This test tries to get close to the capacity limit without exceeding it

        let large_capacity = if usize::MAX > 1_000_000_000 {
            1_000_000_000
        } else {
            usize::MAX / 2
        };

        let mut vec: Vec<u8> = Vec::new();
        let result = vec.try_reserve(large_capacity);

        match result {
            Ok(_) => {
                if large_capacity > 0 {
                    vec.push(1);
                }
            }
            Err(err) => match TryReserveError::from(err).kind() {
                TryReserveErrorKind::CapacityOverflow => {}
                TryReserveErrorKind::AllocError { .. } => {}
            },
        }
    }
}

// Tests for feature flag no_std
#[cfg(feature = "no_std")]
mod no_std_tests {
    use try_reserve::error::{TryReserveError, TryReserveErrorKind};

    #[test]
    fn test_no_std_error_creation() {
        // Create errors without std
        let overflow_error = TryReserveErrorKind::CapacityOverflow;
        let error: TryReserveError = overflow_error.into();

        assert!(matches!(
            error.kind(),
            TryReserveErrorKind::CapacityOverflow
        ));
    }
}
