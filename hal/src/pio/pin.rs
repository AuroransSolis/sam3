use crate::pio::IsPio;
use super::filter::*;
use super::interrupt::*;
use super::peripheral::*;
use super::pioa::*;
use super::piob::*;
use super::pioc::*;
#[cfg(any(feature = "sam3x4e", feature = "sam3x8e", feature = "sam3x8h"))]
use super::piod::*;
#[cfg(feature = "sam3x8h")]
use super::{pioe::*, piof::*};
use core::marker::PhantomData;
use paste::paste;

pub trait PinId {
    type Controller: IsPio;
    const MASK: u32;
}

pub struct Pin<Pio, Pid, Mdvr, Pupr, Irpt, Filt> {
    _pio: PhantomData<Pio>,
    _pid: PhantomData<Pid>,
    _mdvr: PhantomData<Mdvr>,
    _pupr: PhantomData<Pupr>,
    _irpt: PhantomData<Irpt>,
    _filt: PhantomData<Filt>,
}

pub struct Unconfigured;

pub trait Configured {}

pub trait PullupResistorCfg {}

/// Enable the pull-up resistor on an I/O line.
pub struct PullupEnabled;

impl PullupResistorCfg for PullupEnabled {}

/// Disable the pull-up resistor on an I/O line.
pub struct PullupDisabled;

impl PullupResistorCfg for PullupDisabled {}

// LOOK AT ME GO
// macro_rules! line_cfg_mutations {
//     (
//         @defmuts {
//             [$($done:tt,)*],
//             [[$g:tt, $s0:tt, $f0:tt, $s1:tt, $f1:tt$(, $p0:tt)?], $([$a:tt, $b:tt, $c:tt, $d:tt, $e:tt$(, $f:tt)?],)+],
//         }
//     ) => {
//         line_cfg_mutations! {
//             @fundef $($p0)? $s0, [Pio, Pid, $($done,)* $s0 $(, $a)+], $f0
//         }
//
//         line_cfg_mutations! {
//             @fundef $($p0)? $s1, [Pio, Pid, $($done,)* $s1 $(, $a)+], $f1
//         }
//
//         line_cfg_mutations! {
//             @defmuts {
//                 [$($done,)* $g,],
//                 [$([$a, $b, $c, $d, $e],)+],
//             }
//         }
//     };
//     (
//         @defmuts {
//             [$($done:tt,)*],
//             [[$g:tt, $s0:tt, $f0:tt, $s1:tt, $f1:tt],],
//         }
//     ) => {
//         line_cfg_mutations! {
//             @fundef $s0, [Pio, Pid, $($done,)* $s0], $f0
//         }
//
//         line_cfg_mutations! {
//             @fundef $s1, [Pio, Pid, $($done,)* $s1], $f1
//         }
//     };
//     (@fundef $s:ident, [$($ty:tt),+], $f:tt) => {
//         paste! {
//             pub fn [<$s:snake>](self) -> Pin<$($ty,)+> {
//                 unsafe {
//                     (&*(<Pio as IsPio>::PTR as *const RegisterBlock))
//                         .$f
//                         .write_with_zero(|$f| $f.bits(<Pid as crate::pio::pin::PinId>::MASK));
//                     Pin::new()
//                 }
//             }
//         }
//     };
//     (@fundef wp $s:ident, [$($ty:tt),+], $f:tt) => {
//         paste! {
//             pub fn [<$s:snake>](self) -> Pin<$($ty,)+> {
//                 unsafe {
//                     let rb = &*(<Pio as IsPio>::PTR as *const RegisterBlock);
//                     if rb.wpmr.read().wpen().bit() {
//                         rb.wpmr.write(|wpmr| {
//                             wpmr
//                                 .wpkey().bits(<Pio as crate::write_protect::WriteProtect>::WPKEY)
//                                 .wpen().clear_bit()
//                         });
//                         rb.$f.write_with_zero(|$f| $f.bits(<Pid as crate::pio::pin::PinId>::MASK));
//                         rb.wpmr.write(|wpmr| {
//                             wpmr
//                                 .wpen().set_bit()
//                                 .wpkey().bits(0)
//                         });
//                         Pin::new()
//                     } else {
//                         self.[<$s:snake _unchecked>]()
//                     }
//                 }
//             }
//
//             pub unsafe fn [<$s:snake _unchecked>](self) -> Pin<$($ty,)+> {
//                 (&*(<Pio as IsPio>::PTR as *const crate::pac::pioa::RegisterBlock))
//                     .$f
//                     .write_with_zero(|$f| $f.bits(<Pid as crate::pio::pin::PinId>::MASK));
//                 Pin::new()
//             }
//         }
//     };
// }

impl<Pio, Pid, Mdvr, Pupr, Irpt, Filt> Pin<Pio, Pid, Mdvr, Pupr, Irpt, Filt>
where
    Pio: IsPio,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pupr: PullupResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    // line_cfg_mutations! {
    //     @defmuts {
    //         [],
    //         [
    //             [Line, PeripheralControlled, pdr, PioControlled, per, wp],
    //             [Outw, OutputWriteEnabled, ower, OutputWriteDisabled, owdr, wp],
    //             [Otpt, OutputEnabled, oer, OutputDisabled, odr, wp],
    //             [Pupr, PullUpEnabled, puer, PullUpDisabled, pudr, wp],
    //             [Irpt, InterruptEnabled, ier, InterruptDisabled, idr],
    //             [Mdvr, MultiDriverEnabled, mder, MultiDriverDisabled, mddr, wp],
    //             [Psel, PeripheralB, absr, PeripheralA, absr, wp],
    //             [Odta, SetOutput, sodr, ClearOutput, codr],
    //             [Filt, InputFilterEnabled, ifer, InputFilterDisabled, ifdr, wp],
    //             [Flck, SystemClockGlitchFilter, scifsr, DebouncingFilter, difsr],
    //             [
    //                 Aint,
    //                 AdditionalInterruptModesEnabled,
    //                 aimer,
    //                 AdditionalInterruptModesDisabled,
    //                 aimdr
    //             ],
    //         ],
    //     }
    // }

    pub(crate) unsafe fn new() -> Self {
        Pin {
            _pio: PhantomData,
            _pid: PhantomData,
            _mdvr: PhantomData,
            _pupr: PhantomData,
            _irpt: PhantomData,
            _filt: PhantomData,
        }
    }
}

// pub type PinPselFirst<
//     Psel,
//     Pio,
//     Pid,
//     Line,
//     Outw,
//     Otpt,
//     Pupr,
//     Irpt,
//     Mdvr,
//     Odta,
//     Filt,
//     Aint,
// > = Pin<Pio, Pid, Line, Outw, Otpt, Pupr, Irpt, Mdvr, Psel, Odta, Filt, Aint>;

// macro_rules! def_peripherals {
//     (
//         $(
//             $pio_label:tt {
//                 $(
//                     $line_id:literal {
//                         $(#[doc($($adoc:meta),+)])?
//                         A: $a_sel:ident<$($amarker:tt $aparam:ident),+>,
//                         $(#[doc($($bdoc:meta),+)])?
//                         B: $b_sel:ident<$($bmarker:tt $bparam:ident),+>,
//                     },
//                 )+
//             };
//         )+
//     ) => {
//         $(
//             $(
//                 def_peripherals! {
//                     $(#[doc($($adoc),+)])?
//                     $pio_label, $line_id = $a_sel: {
//                         $($amarker $aparam),+
//                     }
//                 }
//
//                 def_peripherals! {
//                     $(#[doc($($bdoc),+)])?
//                     $pio_label, $line_id = $b_sel: {
//                         $($bmarker $bparam),+
//                     }
//                 }
//             )+
//         )+
//     };
//     (
//         $(
//             $pio_label:tt {
//                 $(
//                     $line_id:literal {
//                         $(#[doc($($adoc:meta),+)])?
//                         A: $a_sel:ident<$($amarker:tt $aparam:ident),+>,
//                         $(#[doc($($bdoc:meta),+)])?
//                         B: $b_sel:ident<$($bmarker:tt $bparam:ident),+>,
//                         $(#[doc($($cdoc:meta),+)])?
//                         C: $c_sel:ident<$($cmarker:tt $cparam:ident),+>,
//                         $(#[doc($($ddoc:meta),+)])?
//                         D: $d_sel:ident<$($dmarker:tt $dparam:ident),+>,
//                     },
//                 )+
//             };
//         )+
//     ) => {
//         $(
//             $(
//                 def_peripherals! {
//                     $(#[doc($($adoc),+)])?
//                     $pio_label, $line_id = $a_sel: {
//                         $($amarker $aparam),+
//                     }
//                 }
//
//                 def_peripherals! {
//                     $(#[doc($($bdoc),+)])?
//                     $pio_label, $line_id = $b_sel: {
//                         $($bmarker $bparam),+
//                     }
//                 }
//
//                 def_peripherals! {
//                     $(#[doc($($cdoc),+)])?
//                     $pio_label, $line_id = $c_sel: {
//                         $($cmarker $cparam),+
//                     }
//                 }
//
//                 def_peripherals! {
//                     $(#[doc($($ddoc),+)])?
//                     $pio_label, $line_id = $d_sel: {
//                         $($dmarker $dparam),+
//                     }
//                 }
//             )+
//         )+
//     };
//     (
//         $(#[doc($($doc:meta),+)])?
//         $pio_label:ident, $line_id:literal = $name:ident: {
//             $($marker:tt $marked:ident),+
//         }
//     ) => {
//         def_peripherals! {
//             $(#[doc($($doc),+)])?
//             $pio_label, $line_id = $name: @build {
//                 input: [$($marker $marked),+],
//                 gen: [],
//                 par: [],
//                 all: [],
//             }
//         }
//     };
//     (
//         $(#[doc($($doc:meta),+)])?
//         $pio_label:ident, $line_id:literal = $name:ident: @build {
//             input: [? $gen:ident$(, $marker:tt $marked:ident)+],
//             gen: [$($genlist:ident),*],
//             par: [$($parlist:ident),*],
//             all: [$($alllist:ident),*],
//         }
//     ) => {
//         def_peripherals! {
//             $(#[doc($($doc),+)])?
//             $pio_label, $line_id = $name: @build {
//                 input: [$($marker $marked),+],
//                 gen: [$($genlist, )*$gen],
//                 par: [$($parlist),*],
//                 all: [$($alllist, )*$gen],
//             }
//         }
//     };
//     (
//         $(#[doc($($doc:meta),+)])?
//         $pio_label:ident, $line_id:literal = $name:ident: @build {
//             input: [! $par:ident$(, $marker:tt $marked:ident)+],
//             gen: [$($genlist:ident),*],
//             par: [$($parlist:ident),*],
//             all: [$($alllist:ident),*],
//         }
//     ) => {
//         def_peripherals! {
//             $(#[doc($($doc),+)])?
//             $pio_label, $line_id = $name: @build {
//                 input: [$($marker $marked),+],
//                 gen: [$($genlist),*],
//                 par: [$($parlist, )*$par],
//                 all: [$($alllist, )*$par],
//             }
//         }
//     };
//     (
//         $(#[doc($($doc:meta),+)])?
//         $pio_label:ident, $line_id:literal = $name:ident: @build {
//             input: [? $genlast:ident],
//             gen: [$($genlist:ident),*],
//             par: [$($parlist:ident),*],
//             all: [$($alllist:ident),*],
//         }
//     ) => {
//         def_peripherals! {
//             $(#[doc($($doc),+)])?
//             $pio_label, $line_id = $name: @typedef {
//                 gen: [$($genlist, )*$genlast],
//                 par: [$($parlist),*],
//                 all: [$($alllist, )*$genlast],
//             }
//         }
//     };
//     (
//         $(#[doc($($doc:meta),+)])?
//         $pio_label:ident, $line_id:literal = $name:ident: @build {
//             input: [! $parlast:ident],
//             gen: [$($genlist:ident),*],
//             par: [$($parlist:ident),*],
//             all: [$($alllist:ident),*],
//         }
//     ) => {
//         def_peripherals! {
//             $(#[doc($($doc),+)])?
//             $pio_label, $line_id = $name: @typedef {
//                 gen: [$($genlist),*],
//                 par: [$($parlist, )*$parlast],
//                 all: [$($alllist, )*$parlast],
//             }
//         }
//     };
//     (
//         $(#[doc($($doc:meta),+)])?
//         $pio_label:ident, $line_id:literal = $name:ident: @typedef {
//             gen: [$($genlist:ident),+],
//             par: [$($parlist:ident),+],
//             all: [$($alllist:ident),+],
//         }
//     ) => {
//         paste::paste! {
//             type $name<$($genlist),+> = crate::pio::pin::PinPselFirst<
//                 [<Peripheral $pio_label>],
//                 [<Pio $pio_label>],
//                 [<P $pio_label:lower $line_id>]$(, $alllist)+
//             >;
//
//             impl<Line, Outw, Otpt, Pupr, Irpt, Mdvr, Psel, Odta, Filt, Flck, Aint, Edlv, Frlh>
//                 Pin<
//                     [<Pio $pio_label>],
//                     [<P $pio_label:lower $line_id>],
//                     Line,
//                     Outw,
//                     Otpt,
//                     Pupr,
//                     Irpt,
//                     Mdvr,
//                     Psel,
//                     Odta,
//                     Filt,
//                     Flck,
//                     Aint,
//                     Edlv,
//                     Frlh,
//                 >
//             where
//                 Line: crate::pio::pin::LineCfg,
//                 Outw: crate::pio::pin::OutputWriteCfg,
//                 Otpt: crate::pio::pin::OutputCfg,
//                 Pupr: crate::pio::pin::PullupResistorCfg,
//                 Irpt: crate::pio::pin::InterruptCfg,
//                 Mdvr: crate::pio::pin::MultiDriverCfg,
//                 Psel: crate::pio::pin::ABSelectCfg,
//                 Odta: crate::pio::pin::OutputDataCfg,
//                 Filt: crate::pio::pin::InputFilterCfg,
//                 Flck: crate::pio::pin::InputFilterClockCfg,
//                 Aint: crate::pio::interrupt::AdditionalInterruptModesCfg,
//             {
//                 $(#[doc($($doc),+)])?
//                 pub fn [<$name:lower>](self) -> $name<$($genlist),+> {
//                     self
//                         .[<peripheral_ $pio_label:lower>]()
//                         $(.[<$parlist:snake>]())+
//                 }
//             }
//         }
//     };
// }
//
// pub(crate) use def_peripherals;
