use crate::pac::generic::{BitReader, FieldReader, FieldSpec, IsEnum};

pub trait BReader {
    type R: BReaderFields;
    fn read(&self) -> Self::R;
}

seq_macro::seq! {N in 0..32 {
    paste::paste! {
        pub trait BReaderFields {
            #(
                type [<P~N R>]: BRead;
                fn p~N(&self) -> Self::[<P~N R>];
            )*
            fn bits(&self) -> u32;
        }
    }
}}

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

pub trait BWriter {
    fn reset(&self);
    type W: BWriterFields;
    fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

seq_macro::seq! {N in 0..32 {
    paste::paste! {
        pub trait BWriterFields: Sized {
            #(
                type [<P~N W>]<'a>: BWrite<'a, Self>
                where
                    Self: 'a + Sized;
                fn p~N(&mut self) -> Self::[<P~N W>]<'_>;
            )*
            unsafe fn bits(&mut self, bits: u32) -> &mut Self;
        }
    }
}}

pub trait BWrite<'a, W> {
    const WIDTH: u8 = 1u8;
    fn width(&self) -> u8;
    fn offset(&self) -> u8;
    fn bit(self, value: bool) -> &'a mut W;
    fn variant(self, variant: bool) -> &'a mut W;
    fn set_bit(self) -> &'a mut W;
    fn clear_bit(self) -> &'a mut W;
}

pub trait BWriterWithZero {
    type W: BWriterFields;
    unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

pub trait BModify: BReader + BWriterWithZero {
    fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(
            &<Self as BReader>::R,
            &'w mut <Self as BWriterWithZero>::W,
        ) -> &'w mut <Self as BWriterWithZero>::W;
}

pub trait FRead<FI: FieldSpec> {
    fn bits(&self) -> FI::Ux;
}

pub trait FWrite<'a, const WI: u8, FI: FieldSpec, W> {
    const WIDTH: u8 = WI;
    fn width(&self) -> u8;
    fn offset(&self) -> u8;
    unsafe fn bits(self, value: FI::Ux) -> &'a mut W;
}

pub trait FWriteVariant<'a, const WI: u8, FI: IsEnum, W>: FWrite<'a, WI, FI, W> {
    fn variant(self, variant: FI) -> &'a mut W;
}

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
