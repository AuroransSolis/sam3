use crate::pac::generic::{BitReader, FieldReader, FieldSpec, IsEnum};

/// [`Reg::read`][rr] as a trait.
///
/// [rr]: crate::pac::generic::Reg::read
pub trait BReader {
    type R: BReaderFields;
    fn read(&self) -> Self::R;
}

seq_macro::seq! {N in 0..32 {
    paste::paste! {
        /// Trait providing `.p0()`, `.p1()`, ..., `.p31()`, and [`.bits()`][bits] methods as
        /// readers as found on most PIO registers.
        ///
        /// [bits]: crate::pac::generic::R#method.bits
        pub trait BReaderFields {
            #(
                type [<P~N R>]: BRead;
                fn p~N(&self) -> Self::[<P~N R>];
            )*
            fn bits(&self) -> u32;
        }
    }
}}

/// [`BitReader`] methods as a trait.
pub trait BRead {
    fn bit(&self) -> bool;
    fn bit_is_clear(&self) -> bool;
    fn bit_is_set(&self) -> bool;
}

impl BRead for BitReader {
    fn bit(&self) -> bool {
        self.bit()
    }

    fn bit_is_clear(&self) -> bool {
        self.bit_is_clear()
    }

    fn bit_is_set(&self) -> bool {
        self.bit_is_set()
    }
}

/// [`Reg::reset`][rr] and [`Reg::write`][rw] as a trait.
///
/// [rr]: crate::pac::generic::Reg::reset
/// [rw]: crate::pac::generic::Reg::write
pub trait BWriter {
    fn reset(&self);
    type W: BWriterFields;
    fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

seq_macro::seq! {N in 0..32 {
    paste::paste! {
        /// Trait providing `.p0()`, `.p1()`, ..., `.p31()`, and [`.bits(bits)`][bits] methods as
        /// writers as found on most PIO registers.
        ///
        /// [bits]: crate::pac::generic::W#method.bits
        pub trait BWriterFields: Sized {
            #(
                type [<P~N W>]<'a>: BWrite<'a, Self>
                where
                    Self: 'a + Sized;
                fn p~N(&mut self) -> Self::[<P~N W>]<'_>;
            )*
            #[allow(clippy::missing_safety_doc)]
            /// See [`W::bits`][bits].
            ///
            /// [bits]: crate::pac::generic::W#method.bits
            unsafe fn bits(&mut self, bits: u32) -> &mut Self;
        }
    }
}}

/// [`BitWriter`][bw] methods as a trait.
///
/// [bw]: crate::pac::generic::BitWriter
pub trait BWrite<'a, W> {
    const WIDTH: u8 = 1u8;
    fn width(&self) -> u8;
    fn offset(&self) -> u8;
    fn bit(self, value: bool) -> &'a mut W;
    fn variant(self, variant: bool) -> &'a mut W;
    fn set_bit(self) -> &'a mut W;
    fn clear_bit(self) -> &'a mut W;
}

/// [`Reg::write_with_zero`][rwwz] as a trait.
///
/// [rwwz]: crate::pac::generic::Reg::write_with_zero
pub trait BWriterWithZero {
    type W: BWriterFields;
    #[allow(clippy::missing_safety_doc)]
    /// See [`Reg::write_with_zero`](crate::pac::generic::Reg::write_with_zero).
    unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

/// [`Reg::modify`][rm] as a trait.
///
/// [rm]: crate::pac::generic::Reg::modify
pub trait BModify: BReader + BWriterWithZero {
    fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(
            &<Self as BReader>::R,
            &'w mut <Self as BWriterWithZero>::W,
        ) -> &'w mut <Self as BWriterWithZero>::W;
}

/// [`FieldReader::bits`][frb] as a trait.
///
/// [frb]: crate::pac::generic::FieldReader#method.bits
pub trait FRead<FI: FieldSpec> {
    fn bits(&self) -> FI::Ux;
}

/// [`FieldWriter`][fw] methods as a trait.
///
/// [fw]: crate::pac::generic::FieldWriter
pub trait FWrite<'a, const WI: u8, FI: FieldSpec, W> {
    /// See [`FieldWriter::WIDTH`][width].
    ///
    /// [width]: crate::pac::generic::FieldWriter#associatedconstant.WIDTH
    const WIDTH: u8 = WI;
    /// See [`FieldWriter::width`][width].
    ///
    /// [width]: crate::pac::generic::FieldWriter#method.width
    fn width(&self) -> u8;
    /// See [`FieldWriter::offset`][offset].
    ///
    /// [offset]: crate::pac::generic::FieldWriter#method.offset
    fn offset(&self) -> u8;
    #[allow(clippy::missing_safety_doc)]
    /// See [`FieldWriter::bits`][bits].
    ///
    /// [bits]: crate::pac::generic::FieldWriter#method.bits
    unsafe fn bits(self, value: FI::Ux) -> &'a mut W;
}

/// [`FieldWriter::variant`][fwv] as a trait.
///
/// [fwv]: crate::pac::generic::FieldWriter#method.variant
pub trait FWriteVariant<'a, const WI: u8, FI: IsEnum, W>: FWrite<'a, WI, FI, W> {
    fn variant(self, variant: FI) -> &'a mut W;
}

/// [`FieldWriter::set`][fws] as a trait.
///
/// [fws]: crate::pac::generic::FieldWriter#method.set
pub trait FWriteSafe<'a, const WI: u8, FI: FieldSpec, W>: FWrite<'a, WI, FI, W> {
    fn set(self, value: FI::Ux) -> &'a mut W;
}

impl<FI: FieldSpec> FRead<FI> for FieldReader<FI> {
    fn bits(&self) -> FI::Ux {
        self.bits()
    }
}

macro_rules! bwrite_impl {
    (
        $(meta: [$(#[$meta:meta])*],)?
        ty: $ty:ident,
        reg: $reg:ident,
    ) => {
        paste::paste! {
            $($(#[$meta])*)?
            impl<'a> crate::structure::BWrite<'a, crate::pac::[<$ty:lower>]::[<$reg:lower>]::W>
                for crate::pac::generic::BitWriter<'a, crate::pac::[<$ty:lower>]::[<$reg:lower>]::[<$reg Spec>]>
            {
                const WIDTH: u8 = Self::WIDTH;
                fn width(&self) -> u8 {
                    self.width()
                }
                fn offset(&self) -> u8 {
                    self.offset()
                }
                fn bit(self, value: bool) -> &'a mut crate::pac::[<$ty:lower>]::[<$reg:lower>]::W {
                    self.bit(value)
                }
                fn variant(
                    self,
                    variant: bool,
                ) -> &'a mut crate::pac::[<$ty:lower>]::[<$reg:lower>]::W {
                    self.variant(variant)
                }
                fn set_bit(self) -> &'a mut crate::pac::[<$ty:lower>]::[<$reg:lower>]::W {
                    self.set_bit()
                }
                fn clear_bit(self) -> &'a mut crate::pac::[<$ty:lower>]::[<$reg:lower>]::W {
                    self.clear_bit()
                }
            }
        }
    }
}

pub(crate) use bwrite_impl;
