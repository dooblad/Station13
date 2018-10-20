extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde::UniqId;

    #[test]
    fn increasing_ids_same_group() {
        #[derive(Serde)]
        #[IdGroup = "test"]
        struct TSOne;
        #[derive(Serde)]
        #[IdGroup = "test"]
        struct TSTwo;
        #[derive(Serde)]
        #[IdGroup = "test"]
        struct TSThree;

        assert_eq!(TSOne::id(), 0);
        assert_eq!(TSTwo::id(), 1);
        assert_eq!(TSThree::id(), 2);
    }

    #[test]
    fn same_ids_diff_groups() {
        #[derive(Serde)]
        #[IdGroup = "test1"]
        struct TSOne;
        #[derive(Serde)]
        #[IdGroup = "test2"]
        struct TSTwo;

        assert_eq!(TSOne::id(), 0);
        assert_eq!(TSTwo::id(), 0);
    }

    #[test]
    fn serde_uint() {
        let test_val = 69u32;
        assert_eq!(u32::deserialize(&test_val.serialize()).1, test_val);
    }

    #[test]
    fn serde_float() {
        let test_val: f32 = 69.420;
        assert_eq!(f32::deserialize(&test_val.serialize()).1, test_val);
    }

    #[test]
    fn serde_int() {
        let test_cases = [69i32, std::i32::MIN, std::i32::MAX];
        for case in test_cases.iter() {
            assert_eq!(i32::deserialize(&case.serialize()).1, *case);
        }
    }

    #[test]
    fn serde_array() {
        let test_arr = [0u32, 1, 2];
        assert_eq!(<[u32; 3]>::deserialize(&test_arr.serialize()).1, test_arr);
    }

    #[test]
    fn serde_struct_with_primitive() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct {
            x: u32,
        }

        let test_struct = TestStruct { x: 69 };
        assert_eq!(
            TestStruct::deserialize(&test_struct.serialize()).1,
            test_struct
        );
    }

    #[test]
    fn serde_struct_two_same_primitive() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct {
            x: u32,
            y: u32,
        }

        let test_struct = TestStruct { x: 69, y: 420 };
        assert_eq!(
            TestStruct::deserialize(&test_struct.serialize()).1,
            test_struct
        );
    }

    #[test]
    fn serde_struct_two_diff_primitive() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct {
            x: u8,
            y: u32,
        }

        let test_struct = TestStruct { x: 69, y: 420 };
        assert_eq!(
            TestStruct::deserialize(&test_struct.serialize()).1,
            test_struct
        );
    }

    #[test]
    fn serde_string() {
        let test_val = String::from("farts");
        assert_eq!(String::deserialize(&test_val.serialize()).1, test_val);
    }

    #[test]
    fn serde_vec() {
        let test_val = vec![0i32, 1, 2, 3, 4, 5];
        assert_eq!(<Vec<i32>>::deserialize(&test_val.serialize()).1, test_val);
    }

    #[test]
    fn serde_dyn_sized_struct() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct {
            x: u8,
            y: Vec<usize>,
            z: String,
        }

        let test_struct = TestStruct {
            x: 69,
            y: vec![420],
            z: String::from("farts"),
        };
        assert_eq!(
            TestStruct::deserialize(&test_struct.serialize()).1,
            test_struct
        );
    }

    #[test]
    fn serde_enum() {
        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            Up,
            Down,
            Left,
            Right
        }

        let test_enum = TestEnum::Up;

        assert_eq!(
            TestEnum::deserialize(&test_enum.serialize()).1,
            test_enum
        );
    }
}
