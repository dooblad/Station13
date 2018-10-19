use std::mem;

/// Data type used to represent unique IDs.
pub type Id = u8;

/// Used to provide a unique ID number.
pub trait UniqId {
    fn id() -> Id;
}

pub trait Serialize {
    /// Serializes the type into a vector of bytes.
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    // TODO: Make return type `io::Result`.
    /// Attempts to deserialize an instance of `Self` from a vector of bytes.  Returns the number of
    /// bytes read, along with the constructed `Self` instance.
    fn deserialize(data: &[u8]) -> (usize, Self);
}

// Primitive/Useful Type Trait Implementations

macro_rules! impl_integral {
    ( $ty: ty, $num_bytes: expr ) => {
        impl Serialize for $ty {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::with_capacity($num_bytes);
                for shift in (0..$num_bytes).map(|v| v * 8).rev() {
                    result.push(((*self >> shift) & 0xff) as u8);
                }
                result
            }
        }

        impl Deserialize for $ty {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                let mut result = 0;
                let shift_iter = (0..$num_bytes).map(|v| v * 8).rev();
                for (byte, shift) in data.iter().zip(shift_iter) {
                    result += (*byte as $ty) << shift;
                }
                ($num_bytes, result)
            }
        }
    };
}

impl_integral!(u8, 1);
impl_integral!(u16, 2);
impl_integral!(u32, 4);
impl_integral!(u64, 8);

// We leech off of the unsigned integral implementations above to work with signed integer and float
// types.
//
// Unfortunately, we need two separate arguments for specifying which integral type's implementation
// to use, because in the macro, one of them needs to be a type, and one of them needs to be an
// identifier.
macro_rules! impl_leech {
    ( $ty: ty, $uint_ty: ty, $uint_ident: ident ) => {
        impl Serialize for $ty {
            fn serialize(&self) -> Vec<u8> {
                let as_uint: $uint_ty = unsafe { ::std::mem::transmute(*self) };
                as_uint.serialize()
            }
        }

        impl Deserialize for $ty {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                let (bytes_read, as_uint) = $uint_ident::deserialize(data);
                let as_ty = unsafe { ::std::mem::transmute(as_uint) };
                (bytes_read, as_ty)
            }
        }
    };
}

impl_leech!(f32, u32, u32);
impl_leech!(f64, u64, u64);

impl_leech!(i8, u8, u8);
impl_leech!(i16, u16, u16);
impl_leech!(i32, u32, u32);
impl_leech!(i64, u64, u64);

impl Serialize for usize {
    fn serialize(&self) -> Vec<u8> {
        // Hecking WHAT?!  Did you just assume my computer's word size?
        (*self as u64).serialize()
    }
}

impl Deserialize for usize {
    fn deserialize(data: &[u8]) -> (usize, Self) {
        // Yes I did.
        let (bytes_read, int_result) = <u64>::deserialize(data);
        (bytes_read, int_result as usize)
    }
}

macro_rules! impl_array {
    ( $len: expr ) => {
        impl<T: Serialize> Serialize for [T; $len] {
            fn serialize(&self) -> Vec<u8> {
                let mut result = vec![];
                for val in self.iter() {
                    result.append(&mut val.serialize());
                }
                result
            }
        }

        impl<T: Deserialize + Default> Deserialize for [T; $len] {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                let mut result: [T; $len] = unsafe { mem::uninitialized() };
                let mut bytes_read = 0;
                for i in 0..$len {
                    let deser_data = T::deserialize(&data[bytes_read..]);
                    bytes_read += deser_data.0;
                    result[i] = deser_data.1;
                }
                (bytes_read, result)
            }
        }
    };
}

// No type-level integers, so we have to choose a set of array lengths that we might want to use
// and implement the traits for all of those lengths.
impl_array!(0);
impl_array!(1);
impl_array!(2);
impl_array!(3);
impl_array!(4);
impl_array!(5);
impl_array!(6);
impl_array!(7);
impl_array!(8);
impl_array!(9);
impl_array!(10);
impl_array!(11);
impl_array!(12);
impl_array!(13);
impl_array!(14);
impl_array!(15);
impl_array!(16);
impl_array!(17);
impl_array!(18);
impl_array!(19);
impl_array!(20);
impl_array!(21);
impl_array!(22);
impl_array!(23);
impl_array!(24);
impl_array!(25);
impl_array!(26);
impl_array!(27);
impl_array!(28);
impl_array!(29);
impl_array!(30);
impl_array!(31);
impl_array!(32);

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.len().serialize();
        result.append(&mut self.bytes().collect());
        result
    }
}

impl Deserialize for String {
    fn deserialize(data: &[u8]) -> (usize, Self) {
        let (bytes_read, size) = usize::deserialize(data);
        let result = std::str::from_utf8(&data[bytes_read..])
            .expect("failed to deserialize string")
            .to_string();
        assert_eq!(result.len(), size);
        (bytes_read + size, result)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.len().serialize();
        for val in self.iter() {
            result.append(&mut val.serialize());
        }
        result
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(data: &[u8]) -> (usize, Self) {
        let (mut bytes_read, size) = usize::deserialize(data);
        let mut result = Vec::new();
        for _ in 0..size {
            let deser_data = T::deserialize(&data[bytes_read..]);
            bytes_read += deser_data.0;
            result.push(deser_data.1);
        }
        assert_eq!(result.len(), size);
        (bytes_read, result)
    }
}
