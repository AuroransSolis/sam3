#[allow(clippy::wildcard_imports)]
use crate::{structure::*, write_protect::*};

macro_rules! def_pio_regs {
    (
        pios: $pios:tt,
        regs: $regs:tt,
    ) => {
        def_pio_regs! {
            @defregs $regs
        }

        def_pio_regs! {
            @impls
            pios: $pios,
            regs: $regs,
        }
    };

    (
        @defregs [$($opts:tt),+$(,)?]
    ) => {
        pub trait PioRegisters {
            type Rb: PioRegisters;
            const PTR: *const Self::Rb;
            fn ptr() -> *const Self::Rb {
                <Self as PioRegisters>::PTR
            }
            $(
                def_pio_regs! {
                    @defreg $opts
                }
            )+
        }
    };
    (
        @defreg [$(#[$meta:meta])*$reg:ident, r]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg: BReader;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg;
        }
    };
    (
        @defreg [$(#[$meta:meta])*$reg:ident, w]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg: BWriter;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg;
        }
    };
    (
        @defreg [$(#[$meta:meta])*$reg:ident, wwz]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg: BWriterWithZero;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg;
        }
    };
    (
        @defreg [$(#[$meta:meta])*$reg:ident, mw]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg: BModify + BWriter;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg;
        }
    };
    (
        @defreg [$(#[$meta:meta])*$reg:ident, mwz]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg: BModify;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg;
        }
    };
    (
        @defreg [$(#[$meta:meta])*$reg:ident, $other:ident]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg: $other;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg;
        }
    };

    (
        @impls
        pios: [],
        regs: $regs:tt,
    ) => {};
    (
        @impls
        pios: [$(#[$meta:meta])*$pio:ident$(, $(#[$metarest:meta])*$piorest:ident)*$(,)?],
        regs: $regs:tt,
    ) => {
        def_pio_regs! {
            @implpio
            meta: [$(#[$meta])*],
            pio: $pio,
            regs: $regs,
        }

        def_pio_regs! {
            @impls
            pios: [$($(#[$metarest])*$piorest),*],
            regs: $regs,
        }
    };
    (
        @implpio
        meta: [$(#[$meta:meta])*],
        pio: $pio:ident,
        regs: [$($opts:tt),+$(,)?],
    ) => {
        $(#[$meta])*
        const _: () = {
            paste::paste! {
                impl PioRegisters for crate::pac::$pio {
                    type Rb = crate::pac::[<$pio:lower>]::RegisterBlock;
                    const PTR: *const Self::Rb = crate::pac::$pio::PTR;
                    $(
                        def_pio_regs! {
                            @implpioreg $pio, $opts
                        }
                    )+
                }

                impl PioRegisters for crate::pac::[<$pio:lower>]::RegisterBlock {
                    type Rb = Self;
                    const PTR: *const Self = crate::pac::$pio::PTR;
                    $(
                        def_pio_regs! {
                            @implpioreg $pio, $opts
                        }
                    )+
                }
            }

            $(
                def_pio_regs! {
                    @implpioregfields
                    pio: $pio,
                    reg: $opts,
                }
            )+
        };
    };
    (
        @implpioreg $pio:ident, [$(#[$meta:meta])*$reg:ident, $tag:tt]
    ) => {
        paste::paste! {
            $(#[$meta])*
            type $reg = crate::pac::[<$pio:lower>]::$reg;
            $(#[$meta])*
            fn [<_ $reg:snake>](&self) -> &Self::$reg {
                self.[<$reg:snake>]()
            }
        }
    };
    (
        @implpioregfields
        pio: $pio:ident,
        reg: [$(#[$meta:meta])*$reg:ident, r],
    ) => {
        $(#[$meta])*
        const _: () = {
            paste::paste! {
                impl BReader for crate::pac::[<$pio:lower>]::$reg {
                    type R = crate::pac::[<$pio:lower>]::[<$reg:lower>]::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BReaderFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::R {
                        #(
                            type [<P~N R>] = crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N R>];
                            fn p~N(&self) -> Self::[<P~N R>] {
                                self.p~N()
                            }
                        )*
                        fn bits(&self) -> u32 {
                            self.bits()
                        }
                    }
                }
            }}
        };
    };
    (
        @implpioregfields
        pio: $pio:ident,
        reg: [$(#[$meta:meta])*$reg:ident, w],
    ) => {
        $(#[$meta])*
        const _: () = {
            paste::paste! {
                impl BWriter for crate::pac::[<$pio:lower>]::$reg {
                    fn reset(&self) {
                        self.reset();
                    }
                    type W = crate::pac::[<$pio:lower>]::[<$reg:lower>]::W;
                    fn write<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write(f)
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BWriterFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::W {
                        #(
                            type [<P~N W>]<'a> =
                                crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N W>]<
                                    'a,
                                    crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<$reg Spec>],
                                >;
                            fn p~N(&mut self) -> Self::[<P~N W>]<'_> {
                                self.p~N()
                            }
                        )*
                    }
                }
            }}

            crate::structure::bwrite_impl! {
                ty: $pio,
                reg: $reg,
            }
        };
    };
    (
        @implpioregfields
        pio: $pio:ident,
        reg: [$(#[$meta:meta])*$reg:ident, wwz],
    ) => {
        $(#[$meta])*
        const _: () = {
            paste::paste! {
                impl BWriterWithZero for crate::pac::[<$pio:lower>]::$reg {
                    type W = crate::pac::[<$pio:lower>]::[<$reg:lower>]::W;
                    unsafe fn write_with_zero<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write_with_zero(f)
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BWriterFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::W {
                        #(
                            type [<P~N W>]<'a> =
                                crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N W>]<
                                    'a,
                                    crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<$reg Spec>],
                                >;
                            fn p~N(&mut self) -> Self::[<P~N W>]<'_> {
                                self.p~N()
                            }
                        )*
                        unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                            self.bits(bits)
                        }
                    }
                }
            }}

            crate::structure::bwrite_impl! {
                ty: $pio,
                reg: $reg,
            }
        };
    };
    (
        @implpioregfields
        pio: $pio:ident,
        reg: [$(#[$meta:meta])*$reg:ident, mw],
    ) => {
        $(#[$meta])*
        const _: () = {
            paste::paste! {
                impl BReader for crate::pac::[<$pio:lower>]::$reg {
                    type R = crate::pac::[<$pio:lower>]::[<$reg:lower>]::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BReaderFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::R {
                        #(
                            type [<P~N R>] = crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N R>];
                            fn p~N(&self) -> Self::[<P~N R>] {
                                self.p~N()
                            }
                        )*
                        fn bits(&self) -> u32 {
                            self.bits()
                        }
                    }
                }
            }}

            paste::paste! {
                impl BWriter for crate::pac::[<$pio:lower>]::$reg {
                    fn reset(&self) {
                        self.reset();
                    }
                    type W = crate::pac::[<$pio:lower>]::[<$reg:lower>]::W;
                    fn write<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write(f)
                    }
                }
            }

            paste::paste! {
                impl BWriterWithZero for crate::pac::[<$pio:lower>]::$reg {
                    type W = crate::pac::[<$pio:lower>]::[<$reg:lower>]::W;
                    unsafe fn write_with_zero<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write_with_zero(f)
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BWriterFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::W {
                        #(
                            type [<P~N W>]<'a> =
                                crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N W>]<
                                    'a,
                                    crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<$reg Spec>],
                                >;
                            fn p~N(&mut self) -> Self::[<P~N W>]<'_> {
                                self.p~N()
                            }
                        )*
                        unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                            self.bits(bits)
                        }
                    }
                }
            }}

            crate::structure::bwrite_impl! {
                ty: $pio,
                reg: $reg,
            }

            paste::paste! {
                impl BModify for crate::pac::[<$pio:lower>]::$reg {
                    fn modify<F>(&self, f: F)
                    where
                        for<'w> F: FnOnce(
                            &<Self as BReader>::R,
                            &'w mut <Self as BWriterWithZero>::W,
                        ) -> &'w mut <Self as BWriterWithZero>::W,
                    {
                        self.modify(f);
                    }
                }
            }
        };
    };
    (
        @implpioregfields
        pio: $pio:ident,
        reg: [$(#[$meta:meta])*$reg:ident, mwz],
    ) => {
        $(#[$meta])*
        const _: () = {
            paste::paste! {
                impl BReader for crate::pac::[<$pio:lower>]::$reg {
                    type R = crate::pac::[<$pio:lower>]::[<$reg:lower>]::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BReaderFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::R {
                        #(
                            type [<P~N R>] = crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N R>];
                            fn p~N(&self) -> Self::[<P~N R>] {
                                self.p~N()
                            }
                        )*
                        fn bits(&self) -> u32 {
                            self.bits()
                        }
                    }
                }
            }}

            paste::paste! {
                impl BWriterWithZero for crate::pac::[<$pio:lower>]::$reg {
                    type W = crate::pac::[<$pio:lower>]::[<$reg:lower>]::W;
                    unsafe fn write_with_zero<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write_with_zero(f)
                    }
                }
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    impl BWriterFields for crate::pac::[<$pio:lower>]::[<$reg:lower>]::W {
                        #(
                            type [<P~N W>]<'a> =
                                crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<P~N W>]<
                                    'a,
                                    crate::pac::[<$pio:lower>]::[<$reg:lower>]::[<$reg Spec>],
                                >;
                            fn p~N(&mut self) -> Self::[<P~N W>]<'_> {
                                self.p~N()
                            }
                        )*
                        unsafe fn bits(&mut self, bits: u32) -> &mut Self {
                            self.bits(bits)
                        }
                    }
                }
            }}

            crate::structure::bwrite_impl! {
                ty: $pio,
                reg: $reg,
            }

            paste::paste! {
                impl BModify for crate::pac::[<$pio:lower>]::$reg {
                    fn modify<F>(&self, f: F)
                    where
                        for<'w> F: FnOnce(
                            &<Self as BReader>::R,
                            &'w mut <Self as BWriterWithZero>::W,
                        ) -> &'w mut <Self as BWriterWithZero>::W,
                    {
                        self.modify(f);
                    }
                }
            }
        };
    };
    (
        @implpioregfields
        pio: $pio:ident,
        reg: [$(#[$imeta:meta])*$reg:ident, $other:ident],
    ) => {};
}

def_pio_regs! {
    pios: [
        PIOA,
        PIOB,
        #[cfg(feature = "pioc")]
        PIOC,
        #[cfg(feature = "piod")]
        PIOD,
        #[cfg(feature = "pioe")]
        PIOE,
        #[cfg(feature = "piof")]
        PIOF,
    ],
    regs: [
        [
            #[cfg(any(feature = "3fn", feature = "4fn"))]
            Abcdsr0,
            mwz
        ],
        [
            #[cfg(any(feature = "3fn", feature = "4fn"))]
            Abcdsr1,
            mwz
        ],
        [
            #[cfg(feature = "2fn")]
            Absr,
            mw
        ],
        [Aimdr, wwz],
        [Aimer, wwz],
        [Aimmr, r],
        [Codr, wwz],
        [Elsr, r],
        [Esr, wwz],
        [Fellsr, wwz],
        [Frlhsr, r],
        [Idr, wwz],
        [Ier, wwz],
        [
            #[cfg(any(feature = "sam3a", feature = "sam3u", feature = "sam3x"))]
            Ifdgsr,
            r
        ],
        [Ifdr, wwz],
        [Ifer, wwz],
        [
            #[cfg(any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"))]
            Ifscdr,
            wwz
        ],
        [
            #[cfg(any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"))]
            Ifscer,
            wwz
        ],
        [
            #[cfg(any(feature = "sam3n", feature = "sam3s", feature = "sam3s8"))]
            Ifscsr,
            r
        ],
        [Ifsr, r],
        [Imr, r],
        [Isr, r],
        [Locksr, r],
        [Lsr, wwz],
        [Mddr, wwz],
        [Mder, wwz],
        [Mdsr, r],
        [Odr, wwz],
        [Odsr, mwz],
        [Oer, wwz],
        [Osr, r],
        [Owdr, wwz],
        [Ower, wwz],
        [Owsr, r],
        [Pdr, wwz],
        [Pdsr, r],
        [Per, wwz],
        [
            #[cfg(feature = "ppd")]
            Ppddr,
            wwz
        ],
        [
            #[cfg(feature = "ppd")]
            Ppder,
            wwz
        ],
        [
            #[cfg(feature = "ppd")]
            Ppdsr,
            r
        ],
        [Psr, r],
        [Pudr, wwz],
        [Puer, wwz],
        [Pusr, r],
        [Rehlsr, wwz],
        [Scdr, ScdrModify],
        [
            #[cfg(feature = "schmitt")]
            Schmitt,
            SchmittModify
        ],
        [Sodr, wwz],
        [Wpmr, WpmrModify],
        [Wpsr, WpsrRead],
    ],
}

pub trait ScdrRead {
    type R: ScdrReadFields;
    fn read(&self) -> Self::R;
}

pub trait ScdrReadFields {
    type Div: FRead<u16>;
    fn div(&self) -> Self::Div;
}

pub trait ScdrWrite {
    fn reset(&self);
    type W: ScdrWriteFields;
    fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

pub trait ScdrWriteWithZero {
    type W: ScdrWriteFields;
    unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

pub trait ScdrWriteFields: Sized {
    type Div<'a>: FWrite<'a, 14, u16, Self>
    where
        Self: 'a + Sized;
    fn div(&mut self) -> Self::Div<'_>;
}

pub trait ScdrModify: ScdrRead + ScdrWrite + ScdrWriteWithZero {
    fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(
            &<Self as ScdrRead>::R,
            &'w mut <Self as ScdrWrite>::W,
        ) -> &'w mut <Self as ScdrWrite>::W;
}

#[cfg(feature = "schmitt")]
pub trait SchmittRead {
    type R: SchmittReadFields;
    fn read(&self) -> Self::R;
}

#[cfg(feature = "schmitt")]
seq_macro::seq! {N in 0..32 {
    paste::paste! {
        pub trait SchmittReadFields {
            #(
                type [<Schmitt~N R>]: BRead;
                fn schmitt~N(&self) -> Self::[<Schmitt~N R>];
            )*
        }
    }
}}

#[cfg(feature = "schmitt")]
pub trait SchmittWrite {
    fn reset(&self);
    type W: SchmittWriteFields;
    fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

#[cfg(feature = "schmitt")]
pub trait SchmittWriteWithZero {
    type W: SchmittWriteFields;
    unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

#[cfg(feature = "schmitt")]
seq_macro::seq! {N in 0..32 {
    paste::paste! {
        pub trait SchmittWriteFields: Sized {
            #(
                type [<Schmitt~N W>]<'a>: BWrite<'a, Self>
                where
                    Self: 'a + Sized;
                fn schmitt~N(&mut self) -> Self::[<Schmitt~N W>]<'_>;
            )*
        }
    }
}}

#[cfg(feature = "schmitt")]
pub trait SchmittModify: SchmittRead + SchmittWrite + SchmittWriteWithZero {
    fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(
            &<Self as SchmittRead>::R,
            &'w mut <Self as SchmittWrite>::W,
        ) -> &'w mut <Self as SchmittWrite>::W;
}

macro_rules! other_impls {
    ($($(#[$meta:meta])*$pio:ident),+$(,)?) => {
        $(
            paste::paste! {
                $(#[$meta])*
                const _: () = {
                    impl ScdrRead for crate::pac::[<$pio:lower>]::Scdr {
                        type R = crate::pac::[<$pio:lower>]::scdr::R;
                        fn read(&self) -> Self::R {
                            self.read()
                        }
                    }

                    impl ScdrReadFields for crate::pac::[<$pio:lower>]::scdr::R {
                        type Div = crate::pac::[<$pio:lower>]::scdr::DivR;
                        fn div(&self) -> Self::Div {
                            self.div()
                        }
                    }

                    impl ScdrWrite for crate::pac::[<$pio:lower>]::Scdr {
                        fn reset(&self) {
                            self.reset();
                        }
                        type W = crate::pac::[<$pio:lower>]::scdr::W;
                        fn write<F>(&self, f: F)
                        where
                            F: FnOnce(&mut Self::W) -> &mut Self::W,
                        {
                            self.write(f)
                        }
                    }

                    impl ScdrWriteWithZero for crate::pac::[<$pio:lower>]::Scdr {
                        type W = crate::pac::[<$pio:lower>]::scdr::W;
                        unsafe fn write_with_zero<F>(&self, f: F)
                        where
                            F: FnOnce(&mut Self::W) -> &mut Self::W,
                        {
                            self.write_with_zero(f)
                        }
                    }

                    impl ScdrWriteFields for crate::pac::[<$pio:lower>]::scdr::W {
                        type Div<'a> = crate::pac::[<$pio:lower>]::scdr::DivW<
                            'a,
                            crate::pac::[<$pio:lower>]::scdr::ScdrSpec,
                        >;
                        fn div(&mut self) -> Self::Div<'_> {
                            self.div()
                        }
                    }

                    impl<'a> FWrite<'a, 14, u16, crate::pac::[<$pio:lower>]::scdr::W>
                        for crate::pac::[<$pio:lower>]::scdr::DivW<
                            'a,
                            crate::pac::[<$pio:lower>]::scdr::ScdrSpec,
                        >
                    {
                        const WIDTH: u8 = Self::WIDTH;
                        fn width(&self) -> u8 {
                            self.width()
                        }
                        fn offset(&self) -> u8 {
                            self.offset()
                        }
                        unsafe fn bits(self, value: u16)
                            -> &'a mut crate::pac::[<$pio:lower>]::scdr::W
                        {
                            self.bits(value)
                        }
                    }

                    impl ScdrModify for crate::pac::[<$pio:lower>]::Scdr {
                        fn modify<F>(&self, f: F)
                        where
                            for<'w> F: FnOnce(
                                &<Self as ScdrRead>::R,
                                &'w mut <Self as ScdrWrite>::W,
                            ) -> &'w mut <Self as ScdrWrite>::W,
                        {
                            self.modify(f)
                        }
                    }

                    impl WpmrRead for crate::pac::[<$pio:lower>]::Wpmr {
                        type R = crate::pac::[<$pio:lower>]::wpmr::R;
                        fn read(&self) -> Self::R {
                            self.read()
                        }
                    }

                    impl WpmrReadFields for crate::pac::[<$pio:lower>]::wpmr::R {
                        type Wpen = crate::pac::[<$pio:lower>]::wpmr::WpenR;
                        fn wpen(&self) -> Self::Wpen {
                            self.wpen()
                        }
                        type Wpkey = crate::pac::[<$pio:lower>]::wpmr::WpkeyR;
                        fn wpkey(&self) -> Self::Wpkey {
                            self.wpkey()
                        }
                    }

                    impl WpmrWrite for crate::pac::[<$pio:lower>]::Wpmr {
                        fn reset(&self) {
                            self.reset();
                        }
                        type W = crate::pac::[<$pio:lower>]::wpmr::W;
                        fn write<F>(&self, f: F)
                        where
                            F: FnOnce(&mut Self::W) -> &mut Self::W,
                        {
                            self.write(f)
                        }
                    }

                    impl WpmrWriteWithZero for crate::pac::[<$pio:lower>]::Wpmr {
                        type W = crate::pac::[<$pio:lower>]::wpmr::W;
                        unsafe fn write_with_zero<F>(&self, f: F)
                        where
                            F: FnOnce(&mut Self::W) -> &mut Self::W,
                        {
                            self.write_with_zero(f)
                        }
                    }

                    impl WpmrWriteFields for crate::pac::[<$pio:lower>]::wpmr::W {
                        type Wpen<'a> = crate::pac::[<$pio:lower>]::wpmr::WpenW<
                            'a,
                            crate::pac::[<$pio:lower>]::wpmr::WpmrSpec,
                        >;
                        fn wpen(&mut self) -> Self::Wpen<'_> {
                            self.wpen()
                        }
                        type Wpkey<'a> = crate::pac::[<$pio:lower>]::wpmr::WpkeyW<
                            'a,
                            crate::pac::[<$pio:lower>]::wpmr::WpmrSpec,
                        >;
                        fn wpkey(&mut self) -> Self::Wpkey<'_> {
                            self.wpkey()
                        }
                    }

                    crate::structure::bwrite_impl! {
                        ty: $pio,
                        reg: Wpmr,
                    }

                    impl<'a> FWrite<'a, 24, u32, crate::pac::[<$pio:lower>]::wpmr::W>
                        for crate::pac::[<$pio:lower>]::wpmr::WpkeyW<
                            'a,
                            crate::pac::[<$pio:lower>]::wpmr::WpmrSpec,
                        >
                    {
                        const WIDTH: u8 = Self::WIDTH;
                        fn width(&self) -> u8 {
                            self.width()
                        }
                        fn offset(&self) -> u8 {
                            self.offset()
                        }
                        unsafe fn bits(self, value: u32)
                            -> &'a mut crate::pac::[<$pio:lower>]::wpmr::W
                        {
                            self.bits(value)
                        }
                    }

                    impl WpmrModify for crate::pac::[<$pio:lower>]::Wpmr {
                        fn modify<F>(&self, f: F)
                        where
                            for<'w> F: FnOnce(
                                &<Self as WpmrRead>::R,
                                &'w mut <Self as WpmrWrite>::W,
                            ) -> &'w mut <Self as WpmrWrite>::W,
                        {
                            self.modify(f)
                        }
                    }

                    impl WpsrRead for crate::pac::[<$pio:lower>]::Wpsr {
                        type R = crate::pac::[<$pio:lower>]::wpsr::R;
                        fn read(&self) -> Self::R {
                            self.read()
                        }
                    }

                    impl WpsrReadFields for crate::pac::[<$pio:lower>]::wpsr::R {
                        type Wpvs = crate::pac::[<$pio:lower>]::wpsr::WpvsR;
                        fn wpvs(&self) -> Self::Wpvs {
                            self.wpvs()
                        }
                        type Addr = u16;
                        type Wpvsrc = crate::pac::[<$pio:lower>]::wpsr::WpvsrcR;
                        fn wpvsrc(&self) -> Self::Wpvsrc {
                            self.wpvsrc()
                        }
                    }
                };
            }

            seq_macro::seq! {N in 0..32 {
                paste::paste! {
                    $(#[$meta])*
                    #[cfg(feature = "schmitt")]
                    const _: () = {
                        impl SchmittReadFields for crate::pac::[<$pio:lower>]::schmitt::R {
                            #(
                                type [<Schmitt~N R>] =
                                    crate::pac::[<$pio:lower>]::schmitt::[<Schmitt~N R>];
                                fn schmitt~N(&self) -> Self::[<Schmitt~N R>] {
                                    self.schmitt~N()
                                }
                            )*
                        }

                        impl SchmittWriteFields for crate::pac::[<$pio:lower>]::schmitt::W {
                            #(
                                type [<Schmitt~N W>]<'a> =
                                    crate::pac::[<$pio:lower>]::schmitt::[<Schmitt~N W>]<
                                        'a,
                                        crate::pac::[<$pio:lower>]::schmitt::SchmittSpec,
                                    >;
                                fn schmitt~N(&mut self) -> Self::[<Schmitt~N W>]<'_> {
                                    self.schmitt~N()
                                }
                            )*
                        }

                        crate::structure::bwrite_impl! {
                            ty: $pio,
                            reg: Schmitt,
                        }
                    };
                }
            }}

            paste::paste! {
                $(#[$meta])*
                #[cfg(feature = "schmitt")]
                const _: () = {
                    impl SchmittRead for crate::pac::[<$pio:lower>]::Schmitt {
                        type R = crate::pac::[<$pio:lower>]::schmitt::R;
                        fn read(&self) -> Self::R {
                            self.read()
                        }
                    }

                    impl SchmittWrite for crate::pac::[<$pio:lower>]::Schmitt {
                        fn reset(&self) {
                            self.reset();
                        }
                        type W = crate::pac::[<$pio:lower>]::schmitt::W;
                        fn write<F>(&self, f: F)
                        where
                            F: FnOnce(&mut Self::W) -> &mut Self::W,
                        {
                            self.write(f)
                        }
                    }

                    impl SchmittWriteWithZero for crate::pac::[<$pio:lower>]::Schmitt {
                        type W = crate::pac::[<$pio:lower>]::schmitt::W;
                        unsafe fn write_with_zero<F>(&self, f: F)
                        where
                            F: FnOnce(&mut Self::W) -> &mut Self::W,
                        {
                            self.write_with_zero(f)
                        }
                    }

                    impl SchmittModify for crate::pac::[<$pio:lower>]::Schmitt {
                        fn modify<F>(&self, f: F)
                        where
                            for<'w> F: FnOnce(
                                &<Self as SchmittRead>::R,
                                &'w mut <Self as SchmittWrite>::W,
                            ) -> &'w mut <Self as SchmittWrite>::W,
                        {
                            self.modify(f)
                        }
                    }
                };
            }
        )+
    }
}

other_impls! {
    PIOA,
    PIOB,
    #[cfg(feature = "pioc")]
    PIOC,
    #[cfg(feature = "piod")]
    PIOD,
    #[cfg(feature = "pioe")]
    PIOE,
    #[cfg(feature = "piof")]
    PIOF,
}
