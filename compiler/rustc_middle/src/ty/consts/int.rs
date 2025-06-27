use std::fmt;
use std::num::{NonZero, NonZeroU8};

use rustc_abi::Size;
use rustc_apfloat::Float;
use rustc_apfloat::ieee::{Double, Half, Quad, Single};
use rustc_errors::{DiagArgValue, IntoDiagArg};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

use crate::ty::TyCtxt;

#[derive(Copy, Clone)]
/// A type for representing any integer. Only used for printing.
pub struct ConstInt {
    /// The "untyped" variant of `ConstInt`.
    int: ScalarInt,
    /// Whether the value is of a signed integer type.
    signed: bool,
    /// Whether the value is a `usize` or `isize` type.
    is_ptr_sized_integral: bool,
}

impl ConstInt {
    pub fn new(int: ScalarInt, signed: bool, is_ptr_sized_integral: bool) -> Self {
        Self { int, signed, is_ptr_sized_integral }
    }
}

impl std::fmt::Debug for ConstInt {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { int, signed, is_ptr_sized_integral } = *self;
        let size = int.data_size().bytes();
        let raw = int.data;
        if signed {
            let bit_size = size * 8;
            let min = 1u128 << (bit_size - 1);
            let max = min - 1;
            if raw == min {
                match (size, is_ptr_sized_integral) {
                    (_, true) => write!(fmt, "isize::MIN"),
                    (1, _) => write!(fmt, "i8::MIN"),
                    (2, _) => write!(fmt, "i16::MIN"),
                    (4, _) => write!(fmt, "i32::MIN"),
                    (8, _) => write!(fmt, "i64::MIN"),
                    (16, _) => write!(fmt, "i128::MIN"),
                    _ => bug!("ConstInt 0x{:x} with size = {} and signed = {}", raw, size, signed),
                }
            } else if raw == max {
                match (size, is_ptr_sized_integral) {
                    (_, true) => write!(fmt, "isize::MAX"),
                    (1, _) => write!(fmt, "i8::MAX"),
                    (2, _) => write!(fmt, "i16::MAX"),
                    (4, _) => write!(fmt, "i32::MAX"),
                    (8, _) => write!(fmt, "i64::MAX"),
                    (16, _) => write!(fmt, "i128::MAX"),
                    _ => bug!("ConstInt 0x{:x} with size = {} and signed = {}", raw, size, signed),
                }
            } else {
                match size {
                    1 => write!(fmt, "{}", raw as i8)?,
                    2 => write!(fmt, "{}", raw as i16)?,
                    4 => write!(fmt, "{}", raw as i32)?,
                    8 => write!(fmt, "{}", raw as i64)?,
                    16 => write!(fmt, "{}", raw as i128)?,
                    _ => bug!("ConstInt 0x{:x} with size = {} and signed = {}", raw, size, signed),
                }
                if fmt.alternate() {
                    match (size, is_ptr_sized_integral) {
                        (_, true) => write!(fmt, "_isize")?,
                        (1, _) => write!(fmt, "_i8")?,
                        (2, _) => write!(fmt, "_i16")?,
                        (4, _) => write!(fmt, "_i32")?,
                        (8, _) => write!(fmt, "_i64")?,
                        (16, _) => write!(fmt, "_i128")?,
                        (sz, _) => bug!("unexpected int size i{sz}"),
                    }
                }
                Ok(())
            }
        } else {
            let max = Size::from_bytes(size).truncate(u128::MAX);
            if raw == max {
                match (size, is_ptr_sized_integral) {
                    (_, true) => write!(fmt, "usize::MAX"),
                    (1, _) => write!(fmt, "u8::MAX"),
                    (2, _) => write!(fmt, "u16::MAX"),
                    (4, _) => write!(fmt, "u32::MAX"),
                    (8, _) => write!(fmt, "u64::MAX"),
                    (16, _) => write!(fmt, "u128::MAX"),
                    _ => bug!("ConstInt 0x{:x} with size = {} and signed = {}", raw, size, signed),
                }
            } else {
                match size {
                    1 => write!(fmt, "{}", raw as u8)?,
                    2 => write!(fmt, "{}", raw as u16)?,
                    4 => write!(fmt, "{}", raw as u32)?,
                    8 => write!(fmt, "{}", raw as u64)?,
                    16 => write!(fmt, "{}", raw as u128)?,
                    _ => bug!("ConstInt 0x{:x} with size = {} and signed = {}", raw, size, signed),
                }
                if fmt.alternate() {
                    match (size, is_ptr_sized_integral) {
                        (_, true) => write!(fmt, "_usize")?,
                        (1, _) => write!(fmt, "_u8")?,
                        (2, _) => write!(fmt, "_u16")?,
                        (4, _) => write!(fmt, "_u32")?,
                        (8, _) => write!(fmt, "_u64")?,
                        (16, _) => write!(fmt, "_u128")?,
                        (sz, _) => bug!("unexpected unsigned int size u{sz}"),
                    }
                }
                Ok(())
            }
        }
    }
}

impl IntoDiagArg for ConstInt {
    // FIXME this simply uses the Debug impl, but we could probably do better by converting both
    // to an inherent method that returns `Cow`.
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(format!("{self:?}").into())
    }
}

/// The raw bytes of a simple value.
///
/// This is a packed struct in order to allow this type to be optimally embedded in enums
/// (like Scalar).
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(packed)]
pub struct ScalarInt {
    /// The first `data_size` bytes of `data` are the value.
    /// Do not try to read less or more bytes than that. The remaining bytes must be 0.
    data: u128,
    /// Size of data component of value in bytes.
    data_size: NonZeroU8,
    /// Size of entire value in bytes, including any metadata that is only
    /// present at run time.
    /// This appears on targets like CHERI, which uses the extra space for
    /// bounds information on pointers.
    memrepr_size: NonZeroU8,
}

// Cannot derive these, as the derives take references to the fields, and we
// can't take references to fields of packed structs.
impl<CTX> crate::ty::HashStable<CTX> for ScalarInt {
    fn hash_stable(&self, hcx: &mut CTX, hasher: &mut crate::ty::StableHasher) {
        // Using a block `{self.data}` here to force a copy instead of using `self.data`
        // directly, because `hash_stable` takes `&self` and would thus borrow `self.data`.
        // Since `Self` is a packed struct, that would create a possibly unaligned reference,
        // which is UB.
        { self.data }.hash_stable(hcx, hasher);
        self.data_size.get().hash_stable(hcx, hasher);
        self.memrepr_size.get().hash_stable(hcx, hasher);
    }
}

impl<S: Encoder> Encodable<S> for ScalarInt {
    fn encode(&self, s: &mut S) {
        let data_size = self.data_size.get();
        s.emit_u8(data_size);
        s.emit_u8(self.memrepr_size.get());
        s.emit_raw_bytes(&self.data.to_le_bytes()[..data_size as usize]);
    }
}

impl<D: Decoder> Decodable<D> for ScalarInt {
    fn decode(d: &mut D) -> ScalarInt {
        let mut data = [0u8; 16];
        let data_size = d.read_u8();
        let memrepr_size = d.read_u8();
        data[..data_size as usize].copy_from_slice(d.read_raw_bytes(data_size as usize));
        ScalarInt {
            data: u128::from_le_bytes(data),
            data_size: NonZeroU8::new(data_size).unwrap(),
            memrepr_size: NonZeroU8::new(memrepr_size).unwrap(),
        }
    }
}

impl ScalarInt {
    pub const TRUE: ScalarInt = ScalarInt {
        data: 1_u128,
        data_size: NonZeroU8::new(1).unwrap(),
        memrepr_size: NonZeroU8::new(1).unwrap(),
    };

    pub const FALSE: ScalarInt = ScalarInt {
        data: 0_u128,
        data_size: NonZeroU8::new(1).unwrap(),
        memrepr_size: NonZeroU8::new(1).unwrap(),
    };

    fn raw(data: u128, data_size: Size, memrepr_size: Size) -> Self {
        Self {
            data,
            data_size: NonZero::new(data_size.bytes() as u8).unwrap(),
            memrepr_size: NonZero::new(memrepr_size.bytes() as u8).unwrap(),
        }
    }

    #[inline]
    pub fn data_size(self) -> Size {
        Size::from_bytes(self.data_size.get())
    }

    #[inline]
    pub fn memrepr_size(self) -> Size {
        Size::from_bytes(self.memrepr_size.get())
    }

    /// Make sure the `data` fits in `size`.
    /// This is guaranteed by all constructors here, but having had this check saved us from
    /// bugs many times in the past, so keeping it around is definitely worth it.
    #[inline(always)]
    fn check_data(self) {
        // Using a block `{self.data}` here to force a copy instead of using `self.data`
        // directly, because `debug_assert_eq` takes references to its arguments and formatting
        // arguments and would thus borrow `self.data`. Since `Self`
        // is a packed struct, that would create a possibly unaligned reference, which
        // is UB.
        debug_assert_eq!(
            self.data_size().truncate(self.data),
            { self.data },
            "Scalar value {:#x} exceeds size of {} bytes",
            { self.data },
            self.data_size
        );
    }

    #[inline]
    pub fn null(data_size: Size, memrepr_size: Size) -> Self {
        Self::raw(0, data_size, memrepr_size)
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self.data == 0
    }

    #[inline]
    pub fn try_from_uint(i: impl Into<u128>, data_size: Size, memrepr_size: Size) -> Option<Self> {
        let (r, overflow) = Self::truncate_from_uint(i, data_size, memrepr_size);
        if overflow { None } else { Some(r) }
    }

    /// Returns the truncated result, and whether truncation changed the value.
    #[inline]
    pub fn truncate_from_uint(
        i: impl Into<u128>,
        data_size: Size,
        memrepr_size: Size,
    ) -> (Self, bool) {
        let data = i.into();
        let r = Self::raw(data_size.truncate(data), data_size, memrepr_size);
        (r, r.data != data)
    }

    #[inline]
    pub fn try_from_int(i: impl Into<i128>, data_size: Size, memrepr_size: Size) -> Option<Self> {
        let (r, overflow) = Self::truncate_from_int(i, data_size, memrepr_size);
        if overflow { None } else { Some(r) }
    }

    /// Returns the truncated result, and whether truncation changed the value.
    #[inline]
    pub fn truncate_from_int(
        i: impl Into<i128>,
        data_size: Size,
        memrepr_size: Size,
    ) -> (Self, bool) {
        let data = i.into();
        // `into` performed sign extension, we have to truncate
        let r = Self::raw(data_size.truncate(data as u128), data_size, memrepr_size);
        (r, data_size.sign_extend(r.data) != data)
    }

    #[inline]
    pub fn try_from_target_usize(i: impl Into<u128>, tcx: TyCtxt<'_>) -> Option<Self> {
        Self::try_from_uint(i, tcx.data_layout.pointer_data_size, tcx.data_layout.pointer_data_size)
    }

    /// Try to convert this ScalarInt to the raw underlying bits.
    /// Fails if the size is wrong. Generally a wrong size should lead to a panic,
    /// but Miri sometimes wants to be resilient to size mismatches,
    /// so the interpreter will generally use this `try` method.
    #[inline]
    pub fn try_to_bits(self, target_size: Size) -> Result<u128, (Size, Size)> {
        assert_ne!(target_size.bytes(), 0, "you should never look at the bits of a ZST");
        if target_size.bytes() == u64::from(self.data_size.get()) {
            self.check_data();
            Ok(self.data)
        } else {
            Err((self.data_size(), self.memrepr_size()))
        }
    }

    #[inline]
    pub fn to_bits(self, target_size: Size) -> u128 {
        self.try_to_bits(target_size).unwrap_or_else(|(data_size, memory_size)| {
            bug!(
                "expected int of size {}, but got size {} ({} in memory)",
                target_size.bytes(),
                data_size.bytes(),
                memory_size.bytes()
            )
        })
    }

    /// Extracts the bits from the scalar without checking the size.
    #[inline]
    pub fn to_bits_unchecked(self) -> u128 {
        self.check_data();
        self.data
    }

    /// Converts the `ScalarInt` to an unsigned integer of the given size.
    /// Panics if the size of the `ScalarInt` is not equal to `size`.
    #[inline]
    pub fn to_uint(self, size: Size) -> u128 {
        self.to_bits(size)
    }

    /// Converts the `ScalarInt` to `u8`.
    /// Panics if the `size` of the `ScalarInt`in not equal to 1 byte.
    #[inline]
    pub fn to_u8(self) -> u8 {
        self.to_uint(Size::from_bits(8)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to `u16`.
    /// Panics if the size of the `ScalarInt` in not equal to 2 bytes.
    #[inline]
    pub fn to_u16(self) -> u16 {
        self.to_uint(Size::from_bits(16)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to `u32`.
    /// Panics if the `size` of the `ScalarInt` in not equal to 4 bytes.
    #[inline]
    pub fn to_u32(self) -> u32 {
        self.to_uint(Size::from_bits(32)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to `u64`.
    /// Panics if the `size` of the `ScalarInt` in not equal to 8 bytes.
    #[inline]
    pub fn to_u64(self) -> u64 {
        self.to_uint(Size::from_bits(64)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to `u128`.
    /// Panics if the `size` of the `ScalarInt` in not equal to 16 bytes.
    #[inline]
    pub fn to_u128(self) -> u128 {
        self.to_uint(Size::from_bits(128))
    }

    #[inline]
    pub fn to_target_usize(&self, tcx: TyCtxt<'_>) -> u64 {
        self.to_uint(tcx.data_layout.pointer_data_size).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to `bool`.
    /// Panics if the `size` of the `ScalarInt` is not equal to 1 byte.
    /// Errors if it is not a valid `bool`.
    #[inline]
    pub fn try_to_bool(self) -> Result<bool, ()> {
        match self.to_u8() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }

    /// Converts the `ScalarInt` to a signed integer of the given size.
    /// Panics if the size of the `ScalarInt` is not equal to `size`.
    #[inline]
    pub fn to_int(self, size: Size) -> i128 {
        let b = self.to_bits(size);
        size.sign_extend(b)
    }

    /// Converts the `ScalarInt` to i8.
    /// Panics if the size of the `ScalarInt` is not equal to 1 byte.
    pub fn to_i8(self) -> i8 {
        self.to_int(Size::from_bits(8)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to i16.
    /// Panics if the size of the `ScalarInt` is not equal to 2 bytes.
    pub fn to_i16(self) -> i16 {
        self.to_int(Size::from_bits(16)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to i32.
    /// Panics if the size of the `ScalarInt` is not equal to 4 bytes.
    pub fn to_i32(self) -> i32 {
        self.to_int(Size::from_bits(32)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to i64.
    /// Panics if the size of the `ScalarInt` is not equal to 8 bytes.
    pub fn to_i64(self) -> i64 {
        self.to_int(Size::from_bits(64)).try_into().unwrap()
    }

    /// Converts the `ScalarInt` to i128.
    /// Panics if the size of the `ScalarInt` is not equal to 16 bytes.
    pub fn to_i128(self) -> i128 {
        self.to_int(Size::from_bits(128))
    }

    #[inline]
    pub fn to_target_isize(&self, tcx: TyCtxt<'_>) -> i64 {
        self.to_int(tcx.data_layout.pointer_data_size).try_into().unwrap()
    }

    #[inline]
    pub fn to_float<F: Float>(self) -> F {
        // Going through `to_uint` to check size and truncation.
        F::from_bits(self.to_bits(Size::from_bits(F::BITS)))
    }

    #[inline]
    pub fn to_f16(self) -> Half {
        self.to_float()
    }

    #[inline]
    pub fn to_f32(self) -> Single {
        self.to_float()
    }

    #[inline]
    pub fn to_f64(self) -> Double {
        self.to_float()
    }

    #[inline]
    pub fn to_f128(self) -> Quad {
        self.to_float()
    }
}

macro_rules! from_x_for_scalar_int {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for ScalarInt {
                #[inline]
                fn from(u: $ty) -> Self {
                    let size = NonZero::new(size_of::<$ty>() as u8).unwrap();
                    Self {
                        data: u128::from(u),
                        data_size: size,
                        memrepr_size: size,
                    }
                }
            }
        )*
    }
}

macro_rules! from_scalar_int_for_x {
    ($($ty:ty),*) => {
        $(
            impl From<ScalarInt> for $ty {
                #[inline]
                fn from(int: ScalarInt) -> Self {
                    // The `unwrap` cannot fail because to_bits (if it succeeds)
                    // is guaranteed to return a value that fits into the size.
                    int.to_bits(Size::from_bytes(size_of::<$ty>()))
                       .try_into().unwrap()
                }
            }
        )*
    }
}

from_x_for_scalar_int!(u8, u16, u32, u64, u128, bool);
from_scalar_int_for_x!(u8, u16, u32, u64, u128);

impl TryFrom<ScalarInt> for bool {
    type Error = ();
    #[inline]
    fn try_from(int: ScalarInt) -> Result<Self, ()> {
        int.try_to_bool()
    }
}

impl From<char> for ScalarInt {
    #[inline]
    fn from(c: char) -> Self {
        (c as u32).into()
    }
}

/// Error returned when a conversion from ScalarInt to char fails.
#[derive(Debug)]
pub struct CharTryFromScalarInt;

impl TryFrom<ScalarInt> for char {
    type Error = CharTryFromScalarInt;

    #[inline]
    fn try_from(int: ScalarInt) -> Result<Self, Self::Error> {
        match char::from_u32(int.to_u32()) {
            Some(c) => Ok(c),
            None => Err(CharTryFromScalarInt),
        }
    }
}

impl From<Half> for ScalarInt {
    #[inline]
    fn from(f: Half) -> Self {
        // We trust apfloat to give us properly truncated data.
        let size = NonZero::new((Half::BITS / 8) as u8).unwrap();
        Self { data: f.to_bits(), data_size: size, memrepr_size: size }
    }
}

impl From<ScalarInt> for Half {
    #[inline]
    fn from(int: ScalarInt) -> Self {
        Self::from_bits(int.to_bits(Size::from_bytes(2)))
    }
}

impl From<Single> for ScalarInt {
    #[inline]
    fn from(f: Single) -> Self {
        // We trust apfloat to give us properly truncated data.
        let size = NonZero::new((Single::BITS / 8) as u8).unwrap();
        Self { data: f.to_bits(), data_size: size, memrepr_size: size }
    }
}

impl From<ScalarInt> for Single {
    #[inline]
    fn from(int: ScalarInt) -> Self {
        Self::from_bits(int.to_bits(Size::from_bytes(4)))
    }
}

impl From<Double> for ScalarInt {
    #[inline]
    fn from(f: Double) -> Self {
        // We trust apfloat to give us properly truncated data.
        let size = NonZero::new((Double::BITS / 8) as u8).unwrap();
        Self { data: f.to_bits(), data_size: size, memrepr_size: size }
    }
}

impl From<ScalarInt> for Double {
    #[inline]
    fn from(int: ScalarInt) -> Self {
        Self::from_bits(int.to_bits(Size::from_bytes(8)))
    }
}

impl From<Quad> for ScalarInt {
    #[inline]
    fn from(f: Quad) -> Self {
        // We trust apfloat to give us properly truncated data.
        let size = NonZero::new((Quad::BITS / 8) as u8).unwrap();
        Self { data: f.to_bits(), data_size: size, memrepr_size: size }
    }
}

impl From<ScalarInt> for Quad {
    #[inline]
    fn from(int: ScalarInt) -> Self {
        Self::from_bits(int.to_bits(Size::from_bytes(16)))
    }
}

impl fmt::Debug for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Dispatch to LowerHex below.
        write!(f, "0x{self:x}")
    }
}

impl fmt::LowerHex for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.check_data();
        if f.alternate() {
            // Like regular ints, alternate flag adds leading `0x`.
            write!(f, "0x")?;
        }
        // Format as hex number wide enough to fit any value of the given `size`.
        // So data=20, size=1 will be "0x14", but with size=4 it'll be "0x00000014".
        // Using a block `{self.data}` here to force a copy instead of using `self.data`
        // directly, because `write!` takes references to its formatting arguments and
        // would thus borrow `self.data`. Since `Self`
        // is a packed struct, that would create a possibly unaligned reference, which
        // is UB.
        write!(f, "{:01$x}", { self.data }, self.data_size.get() as usize * 2)
    }
}

impl fmt::UpperHex for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.check_data();
        // Format as hex number wide enough to fit any value of the given `size`.
        // So data=20, size=1 will be "0x14", but with size=4 it'll be "0x00000014".
        // Using a block `{self.data}` here to force a copy instead of using `self.data`
        // directly, because `write!` takes references to its formatting arguments and
        // would thus borrow `self.data`. Since `Self`
        // is a packed struct, that would create a possibly unaligned reference, which
        // is UB.
        write!(f, "{:01$X}", { self.data }, self.data_size.get() as usize * 2)
    }
}

impl fmt::Display for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.check_data();
        write!(f, "{}", { self.data })
    }
}
