#[allow(clippy::wildcard_imports)]
use crate::{pac::generic::FieldSpec, structure::*};

pub trait WriteProtectKey {
    /// `const` value to insert into the `WPKEY` field of the write protection register of a
    /// peripheral with write protection capability.
    const WPKEY: u32;
}

pub trait WpmrWpsrRegs {
    type Wpmr: WpmrModify;
    fn _wpmr(&self) -> &Self::Wpmr;
    type Wpsr: WpsrRead;
    fn _wpsr(&self) -> &Self::Wpsr;
}

pub trait WriteProtect: WriteProtectKey + WpmrWpsrRegs {
    fn enable_writeprotect(&mut self) {
        self._wpmr()
            .write(|w| unsafe { w.wpkey().bits(Self::WPKEY).wpen().set_bit() })
    }

    fn disable_writeprotect(&mut self) {
        self._wpmr()
            .write(|w| unsafe { w.wpkey().bits(Self::WPKEY).wpen().clear_bit() });
    }

    fn writeprotect_enabled(&self) -> bool {
        self._wpmr().read().wpen().bit()
    }

    /// Check if a peripheral with write protection capability is indicating a write protection
    /// error.
    fn writeprotect_error(&self) -> bool {
        self._wpsr().read().wpvs().bit()
    }

    /// Check if the write protect status register of a peripheral with write protection capability
    /// is indicating that a write protection error has occurred, and if it has, return the address
    /// of the register write request that generated the error. Otherwise, `None` is returned.
    fn writeprotect_error_addr(
        &self,
    ) -> Option<<<Self::Wpsr as WpsrRead>::R as WpsrReadFields>::Addr> {
        if self.writeprotect_error() {
            unsafe { Some(self.writeprotect_error_addr_unchecked()) }
        } else {
            None
        }
    }

    /// Read the value from the `WPROTADDR` field of the write protection status register of a
    /// peripheral with write protection capability without checking the `WPROTERR` flag.
    ///
    /// # Safety
    ///
    /// The value held in this register may be nonsensical if the `WPROTERR` flag is not indicating
    /// that a write protection error has occured, thus it is recommended to use the
    /// [`writeprotect_error_addr`](crate::write_protect::WriteProtect::writeprotect_error_addr)
    /// method instead.
    unsafe fn writeprotect_error_addr_unchecked(
        &self,
    ) -> <<Self::Wpsr as WpsrRead>::R as WpsrReadFields>::Addr {
        self._wpsr().read().wpvsrc().bits()
    }
}

pub trait WpmrRead {
    type R: WpmrReadFields;
    /// See [`Reg::read`](crate::pac::generic::Reg::read)
    fn read(&self) -> Self::R;
}

pub trait WpmrReadFields {
    type Wpen: BRead;
    fn wpen(&self) -> Self::Wpen;
    type Wpkey: FRead<u32>;
    fn wpkey(&self) -> Self::Wpkey;
}

pub trait WpmrWrite {
    /// See [`Reg::reset`](crate::pac::generic::Reg::reset).
    fn reset(&self);
    type W: WpmrWriteFields;
    /// See [`Reg::write`](crate::pac::generic::Reg::write).
    fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

pub trait WpmrWriteWithZero {
    type W: WpmrWriteFields;
    #[allow(clippy::missing_safety_doc)]
    /// See [`Reg::write_with_zero`](crate::pac::generic::Reg::write_with_zero).
    unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

pub trait WpmrWriteFields: Sized {
    type Wpen<'a>: BWrite<'a, Self>
    where
        Self: 'a + Sized;
    fn wpen(&mut self) -> Self::Wpen<'_>;
    type Wpkey<'a>: FWrite<'a, 24, u32, Self>
    where
        Self: 'a + Sized;
    fn wpkey(&mut self) -> Self::Wpkey<'_>;
}

pub trait WpmrModify: WpmrRead + WpmrWrite + WpmrWriteWithZero {
    /// See [`Reg::modify`](crate::pac::generic::Reg::modify).
    fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(
            &<Self as WpmrRead>::R,
            &'w mut <Self as WpmrWrite>::W,
        ) -> &'w mut <Self as WpmrWrite>::W;
}

pub trait WpsrRead {
    type R: WpsrReadFields;
    /// See [`Reg::read`](crate::pac::generic::Reg::read).
    fn read(&self) -> Self::R;
}

pub trait WpsrReadFields {
    type Wpvs: BRead;
    fn wpvs(&self) -> Self::Wpvs;
    type Addr: FieldSpec<Ux = Self::Addr>;
    type Wpvsrc: FRead<Self::Addr>;
    fn wpvsrc(&self) -> Self::Wpvsrc;
}

macro_rules! wpmr_wpsr_impl {
    (
        $ty:ty: {
            key: $key:literal,
            addr: $addr:ty,
            wpvs: $wpvs:ident,
            wpvsrc: $wpvsrc:ident,
        }
    ) => {
        paste::paste! {
            const _: () = {
                use crate::{
                    pac::[<$ty:lower>]::{
                        wpmr::{self, WpenR, WpenW, WpkeyR, WpkeyW, WpmrSpec},
                        wpsr,
                        Wpmr,
                        Wpsr,
                    },
                    structure::FWrite,
                    write_protect::{
                        WriteProtectKey,
                        WpmrModify,
                        WpmrRead,
                        WpmrReadFields,
                        WpmrWpsrRegs,
                        WpmrWrite,
                        WpmrWriteFields,
                        WpmrWriteWithZero,
                        WpsrRead,
                        WpsrReadFields,
                    },
                };

                impl WriteProtectKey for $ty {
                    const WPKEY: u32 = $key;
                }

                impl WpmrWpsrRegs for $ty {
                    type Wpmr = Wpmr;
                    fn _wpmr(&self) -> &Self::Wpmr {
                        self.wpmr()
                    }
                    type Wpsr = Wpsr;
                    fn _wpsr(&self) -> &Self::Wpsr {
                        self.wpsr()
                    }
                }

                impl WpmrRead for Wpmr {
                    type R = wpmr::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }

                impl WpmrReadFields for wpmr::R {
                    type Wpen = WpenR;
                    fn wpen(&self) -> Self::Wpen {
                        self.wpen()
                    }
                    type Wpkey = WpkeyR;
                    fn wpkey(&self) -> Self::Wpkey {
                        self.wpkey()
                    }
                }

                impl WpmrWrite for Wpmr {
                    fn reset(&self) {
                        self.reset()
                    }
                    type W = wpmr::W;
                    fn write<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W
                    {
                        self.write(f)
                    }
                }

                impl WpmrWriteWithZero for Wpmr {
                    type W = wpmr::W;
                    unsafe fn write_with_zero<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write_with_zero(f)
                    }
                }

                impl WpmrWriteFields for wpmr::W {
                    type Wpen<'a> = WpenW<'a, WpmrSpec>;
                    fn wpen(&mut self) -> Self::Wpen<'_> {
                        self.wpen()
                    }
                    type Wpkey<'a> = WpkeyW<'a, WpmrSpec>;
                    fn wpkey(&mut self) -> Self::Wpkey<'_> {
                        self.wpkey()
                    }
                }

                crate::structure::bwrite_impl! {
                    ty: $ty,
                    reg: Wpmr,
                }

                impl<'a> FWrite<'a, 24, u32, wpmr::W> for WpkeyW<'a, WpmrSpec> {
                    const WIDTH: u8 = Self::WIDTH;
                    fn width(&self) -> u8 {
                        self.width()
                    }
                    fn offset(&self) -> u8 {
                        self.offset()
                    }
                    unsafe fn bits(self, value: u32) -> &'a mut wpmr::W {
                        self.bits(value)
                    }
                }

                impl WpmrModify for Wpmr {
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

                impl WpsrRead for Wpsr {
                    type R = wpsr::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }

                impl WpsrReadFields for wpsr::R {
                    type Wpvs = wpsr::[<$wpvs R>];
                    fn wpvs(&self) -> Self::Wpvs {
                        self.[<$wpvs:lower>]()
                    }
                    type Addr = $addr;
                    type Wpvsrc = wpsr::[<$wpvsrc R>];
                    fn wpvsrc(&self) -> Self::Wpvsrc {
                        self.[<$wpvsrc:lower>]()
                    }
                }
            };
        }
    };
}

pub(crate) use wpmr_wpsr_impl;

// macro_rules! wp_impl {
//     ($(
//         $(#[$fields:meta])+
//         $type:ty => $field:ident($err:ident, $addr:ident<$addrty:ty>): $key:literal
//     ),+$(,)?) => {$(
//         paste::paste! {
//             #[doc = "`" $type "` has write protection for the following registers:"]
//             #[doc = ""]
//             $(#[$fields])+
//             impl crate::write_protect::WriteProtect for $type {
//                 const WPKEY: u32 = {
//                     let [b0, b1, b2] = *$key;
//                     u32::from_be_bytes([0, b0, b1, b2])
//                 };
//
//                 type AddrType = $addrty;
//
//                 #[rustfmt::skip]
//                 fn enable_writeprotect(&mut self) {
//                     self.$field.wpmr().write(|wpmr| unsafe {
//                         wpmr
//                             .wpkey().bits(Self::WPKEY)
//                             .wpen().set_bit()
//                     });
//                 }
//
//                 #[rustfmt::skip]
//                 fn disable_writeprotect(&mut self) {
//                     self.$field.wpmr().write(|wpmr| unsafe {
//                         wpmr
//                             .wpkey().bits(Self::WPKEY)
//                             .wpen().clear_bit()
//                     });
//                 }
//
//                 fn writeprotect_enabled(&self) -> bool {
//                     self.$field.wpmr().read().wpen().bit()
//                 }
//
//                 fn writeprotect_error(&self) -> bool {
//                     self.$field.wpsr().read().$err().bit()
//                 }
//
//                 /// Read the value from the `WPROTADDR` field of the write protection status register of a
//                 /// peripheral with write protection capability without checking the `WPROTERR` flag.
//                 ///
//                 /// # Safety
//                 ///
//                 /// The value held in this register may be nonsensical if the `WPROTERR` flag is not indicating
//                 /// that a write protection error has occured, thus it is recommended to use the
//                 /// [`writeprotect_error_addr`](crate::write_protect::WriteProtect::writeprotect_error_addr)
//                 /// method instead.
//                 unsafe fn writeprotect_error_addr_unchecked(&self) -> $addrty {
//                     self.$field.wpsr().read().$addr().bits()
//                 }
//             }
//         }
//     )+};
// }
//
// pub(crate) use wp_impl;
