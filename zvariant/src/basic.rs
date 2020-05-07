use crate::{Signature, Type};

/// Trait for basic types.
///
/// All basic types are also [`Type`] implementers.
///
/// [`Type`]: trait.Type.html
/// [`Value`]: enum.Value.html
pub trait Basic {
    const SIGNATURE_CHAR: char;
    const SIGNATURE_STR: &'static str;
    const ALIGNMENT: usize;
}

macro_rules! impl_type {
    ($for:ty) => {
        impl Type for $for {
            fn signature() -> Signature<'static> {
                Signature::from_str_unchecked(<$for>::SIGNATURE_STR)
            }
        }
    };
}

impl Basic for u8 {
    const SIGNATURE_CHAR: char = 'y';
    const SIGNATURE_STR: &'static str = "y";
    const ALIGNMENT: usize = 1;
}
impl_type!(u8);

// No i8 type in D-Bus/GVariant, let's pretend it's i16
impl Basic for i8 {
    const SIGNATURE_CHAR: char = i16::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = i16::SIGNATURE_STR;
    const ALIGNMENT: usize = i16::ALIGNMENT;
}
impl_type!(i8);

impl Basic for bool {
    const SIGNATURE_CHAR: char = 'b';
    const SIGNATURE_STR: &'static str = "b";
    const ALIGNMENT: usize = 4;
}
impl_type!(bool);

impl Basic for i16 {
    const SIGNATURE_CHAR: char = 'n';
    const SIGNATURE_STR: &'static str = "n";
    const ALIGNMENT: usize = 2;
}
impl_type!(i16);

impl Basic for u16 {
    const SIGNATURE_CHAR: char = 'q';
    const SIGNATURE_STR: &'static str = "q";
    const ALIGNMENT: usize = 2;
}
impl_type!(u16);

impl Basic for i32 {
    const SIGNATURE_CHAR: char = 'i';
    const SIGNATURE_STR: &'static str = "i";
    const ALIGNMENT: usize = 4;
}
impl_type!(i32);

impl Basic for u32 {
    const SIGNATURE_CHAR: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
    const ALIGNMENT: usize = 4;
}
impl_type!(u32);

impl Basic for i64 {
    const SIGNATURE_CHAR: char = 'x';
    const SIGNATURE_STR: &'static str = "x";
    const ALIGNMENT: usize = 8;
}
impl_type!(i64);

impl Basic for u64 {
    const SIGNATURE_CHAR: char = 't';
    const SIGNATURE_STR: &'static str = "t";
    const ALIGNMENT: usize = 8;
}
impl_type!(u64);

// No f32 type in D-Bus/GVariant, let's pretend it's f64
impl Basic for f32 {
    const SIGNATURE_CHAR: char = f64::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = f64::SIGNATURE_STR;
    const ALIGNMENT: usize = f64::ALIGNMENT;
}
impl_type!(f32);

impl Basic for f64 {
    const SIGNATURE_CHAR: char = 'd';
    const SIGNATURE_STR: &'static str = "d";
    const ALIGNMENT: usize = 8;
}
impl_type!(f64);

impl Basic for &str {
    const SIGNATURE_CHAR: char = 's';
    const SIGNATURE_STR: &'static str = "s";
    const ALIGNMENT: usize = 4;
}
impl_type!(&str);

impl Basic for String {
    const SIGNATURE_CHAR: char = 's';
    const SIGNATURE_STR: &'static str = "s";
    const ALIGNMENT: usize = 4;
}
impl_type!(String);

impl Basic for char {
    const SIGNATURE_CHAR: char = <&str>::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = <&str>::SIGNATURE_STR;
    const ALIGNMENT: usize = <&str>::ALIGNMENT;
}
impl_type!(char);
