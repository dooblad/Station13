pub trait Serialize {
    /// Serializes the type into a vector of bytes.
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    // TODO: Make return type `io::Result`.
    /// Attempts to deserialize the type into a vector of bytes.
    fn deserialize(data: &[u8]) -> Self;
    /// Returns the number of bytes needed to deserialize this type (must be deterministic).
    fn required_bytes() -> usize;
}

// Primitive Type Trait Implementations

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
            fn deserialize(data: &[u8]) -> Self {
                let mut result = 0;
                let shift_iter = (0..$num_bytes).map(|v| v * 8).rev();
                for (byte, shift) in data.iter().zip(shift_iter) {
                    result += (*byte as $ty) << shift;
                }
                result
            }
            fn required_bytes() -> usize { $num_bytes }
        }
    }
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
            fn deserialize(data: &[u8]) -> Self {
                let as_uint = $uint_ident::deserialize(data);
                unsafe { ::std::mem::transmute(as_uint) }
            }

            fn required_bytes() -> usize { $uint_ident::required_bytes() }
        }
    }
}

impl_leech!(f32, u32, u32);
impl_leech!(f64, u64, u64);

impl_leech!(i8, u8, u8);
impl_leech!(i16, u16, u16);
impl_leech!(i32, u32, u32);
impl_leech!(i64, u64, u64);

macro_rules! impl_array {
    ( $len: expr ) => {
        impl<T: Serialize> Serialize for [T; $len] {
            fn serialize(&self) -> Vec<u8> {
                let mut result = vec![];
                for i in 0..self.len() {
                    result.append(&mut self[i].serialize());
                }
                result
            }
        }

        impl<T: Deserialize + Default> Deserialize for [T; $len] {
            fn deserialize(data: &[u8]) -> Self {
                // TODO: Need to be able to initialize an array and iterate over it to fill the
                // entries.  Or we could push it all to a vec and convert it to an array at the
                // end.
                let mut result: [T; $len] = unsafe { ::std::mem::uninitialized() };
                let mut offs = 0;
                let stride = T::required_bytes();
                for i in 0..$len {
                    result[i] = T::deserialize(&data[offs..offs+stride]);
                    offs += stride;
                }
                result
            }

            fn required_bytes() -> usize { T::required_bytes() * $len }
        }
    }
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

