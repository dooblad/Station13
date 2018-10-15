extern crate uniq_id;
#[macro_use]
extern crate uniq_id_derive;

#[cfg(test)]
mod tests {
    use uniq_id::UniqId;
    use uniq_id::serde::{Serialize, Deserialize};

    #[test]
    fn increasing_ids_same_group() {
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TSOne;
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TSTwo;
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TSThree;

        assert_eq!(TSOne::id(), 0);
        assert_eq!(TSTwo::id(), 1);
        assert_eq!(TSThree::id(), 2);
    }

    #[test]
    fn same_ids_diff_groups() {
        #[derive(UniqId)]
        #[UniqGroup = "test1"]
        struct TSOne;
        #[derive(UniqId)]
        #[UniqGroup = "test2"]
        struct TSTwo;

        assert_eq!(TSOne::id(), 0);
        assert_eq!(TSTwo::id(), 0);
    }

    #[test]
    fn serialize_primitive() {
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TestStruct {
            x: u32,
        }

        let test_struct = TestStruct { x: 69 };
        assert_eq!(test_struct.serialize(), vec![0u8, 0, 0, 69]);
    }

    #[test]
    fn serialize_two_same_primitive() {
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TestStruct {
            x: u32,
            y: u32,
        }

        let test_struct = TestStruct { x: 69, y: 420 };
        assert_eq!(test_struct.serialize(), vec![0u8, 0, 0, 69, 0, 0, 1, 164]);
    }

    #[test]
    fn serialize_two_diff_primitive() {
        #[derive(UniqId)]
        #[UniqGroup = "test"]
        struct TestStruct {
            x: u8,
            y: u32,
        }

        let test_struct = TestStruct { x: 69, y: 420 };
        assert_eq!(test_struct.serialize(), vec![69u8, 0, 0, 1, 164]); // [1, 164] == 420
    }

    #[test]
    fn serialize_deserialize_float() {
        let test_val: f32 = 69.420;
        assert_eq!(f32::deserialize(&test_val.serialize()), test_val);
    }

    #[test]
    fn serialize_deserialize_int() {
        let test_cases = [69i32, std::i32::MIN, std::i32::MAX];
        for case in test_cases.iter() {
            assert_eq!(i32::deserialize(&case.serialize()), *case);
        }
    }

    #[test]
    fn serialize_deserialize_array() {
        let test_arr = [0u32, 1, 2];
        assert_eq!(<[u32; 3]>::deserialize(&test_arr.serialize()), test_arr);
    }
}
