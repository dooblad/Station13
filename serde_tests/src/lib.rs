extern crate serde;
extern crate serde_derive;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_derive::Serde;

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
    fn serde_empty_struct() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct {}

        let test_struct = TestStruct {};
        assert_eq!(
            TestStruct::deserialize(&test_struct.serialize()).1,
            test_struct
        );
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
    fn serde_tuple_struct() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct(u32, String);

        let test_struct = TestStruct(0, String::from("ayy"));
        assert_eq!(
            TestStruct::deserialize(&test_struct.serialize()).1,
            test_struct
        );
    }

    #[test]
    fn serde_enum_no_fields() {
        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            Up,
            Down,
            Left,
            Right,
        }

        let test_enum = TestEnum::Up;

        assert_eq!(TestEnum::deserialize(&test_enum.serialize()).1, test_enum);
    }

    #[test]
    fn serde_enum_with_fields() {
        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            A { x: u32, y: u32 },
            B { s: String },
        }

        let test_enum = TestEnum::B {
            s: String::from("ayy lmao"),
        };

        assert_eq!(TestEnum::deserialize(&test_enum.serialize()).1, test_enum);
    }

    #[test]
    fn serde_tuple_enum_variant() {
        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            A(u32),
        }

        let test_enum = TestEnum::A(69);
        assert_eq!(TestEnum::deserialize(&test_enum.serialize()).1, test_enum);
    }

    #[test]
    fn serde_tuple_enum_variant_to_struct() {
        #[derive(Debug, PartialEq, Serde)]
        struct TestStruct {
            x: u32,
        }

        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            A(TestStruct),
            B(TestStruct),
        }

        let test_enum = TestEnum::B(TestStruct { x: 1337 });
        assert_eq!(TestEnum::deserialize(&test_enum.serialize()).1, test_enum);
    }

    #[test]
    fn enum_tag() {
        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            A(u32),
            B(u32),
        }

        assert_eq!(TestEnum::A(66).enum_tag(), 0);
        assert_eq!(TestEnum::B(66).enum_tag(), 1);
    }

    #[test]
    fn num_variants() {
        #[derive(Debug, PartialEq, Serde)]
        enum TestEnum {
            A(u32),
            B(u32),
            C(u32),
        }

        assert_eq!(TestEnum::num_variants(), 3);
    }
}
