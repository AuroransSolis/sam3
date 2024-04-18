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
    fn read(&self) -> Self::R;
}

pub trait WpmrReadFields {
    type Wpen: BRead;
    fn wpen(&self) -> Self::Wpen;
    type Wpkey: FRead<u32>;
    fn wpkey(&self) -> Self::Wpkey;
}

pub trait WpmrWrite {
    fn reset(&self);
    type W: WpmrWriteFields;
    fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut Self::W) -> &mut Self::W;
}

pub trait WpmrWriteWithZero {
    type W: WpmrWriteFields;
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
    fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(
            &<Self as WpmrRead>::R,
            &'w mut <Self as WpmrWrite>::W,
        ) -> &'w mut <Self as WpmrWrite>::W;
}

pub trait WpsrRead {
    type R: WpsrReadFields;
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
                impl crate::write_protect::WriteProtectKey for $ty {
                    const WPKEY: u32 = $key;
                }

                impl crate::write_protect::WpmrWpsrRegs for $ty {
                    type Wpmr = crate::pac::[<$ty:lower>]::Wpmr;
                    fn _wpmr(&self) -> &Self::Wpmr {
                        self.wpmr()
                    }
                    type Wpsr = crate::pac::[<$ty:lower>]::Wpsr;
                    fn _wpsr(&self) -> &Self::Wpsr {
                        self.wpsr()
                    }
                }

                impl crate::write_protect::WpmrRead for crate::pac::[<$ty:lower>]::Wpmr {
                    type R = crate::pac::[<$ty:lower>]::wpmr::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }

                impl crate::write_protect::WpmrReadFields for crate::pac::[<$ty:lower>]::wpmr::R {
                    type Wpen = crate::pac::[<$ty:lower>]::wpmr::WpenR;
                    fn wpen(&self) -> Self::Wpen {
                        self.wpen()
                    }
                    type Wpkey = crate::pac::[<$ty:lower>]::wpmr::WpkeyR;
                    fn wpkey(&self) -> Self::Wpkey {
                        self.wpkey()
                    }
                }

                impl crate::write_protect::WpmrWrite for crate::pac::[<$ty:lower>]::Wpmr {
                    fn reset(&self) {
                        self.reset()
                    }
                    type W = crate::pac::[<$ty:lower>]::wpmr::W;
                    fn write<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W
                    {
                        self.write(f)
                    }
                }

                impl crate::write_protect::WpmrWriteWithZero for crate::pac::[<$ty:lower>]::Wpmr {
                    type W = crate::pac::[<$ty:lower>]::wpmr::W;
                    unsafe fn write_with_zero<F>(&self, f: F)
                    where
                        F: FnOnce(&mut Self::W) -> &mut Self::W,
                    {
                        self.write_with_zero(f)
                    }
                }

                impl crate::write_protect::WpmrWriteFields for crate::pac::[<$ty:lower>]::wpmr::W {
                    type Wpen<'a> = crate::pac::[<$ty:lower>]::wpmr::WpenW<
                        'a,
                        crate::pac::[<$ty:lower>]::wpmr::WpmrSpec,
                    >;
                    fn wpen(&mut self) -> Self::Wpen<'_> {
                        self.wpen()
                    }
                    type Wpkey<'a> = crate::pac::[<$ty:lower>]::wpmr::WpkeyW<
                        'a,
                        crate::pac::[<$ty:lower>]::wpmr::WpmrSpec,
                    >;
                    fn wpkey(&mut self) -> Self::Wpkey<'_> {
                        self.wpkey()
                    }
                }

                crate::structure::bwrite_impl! {
                    ty: $ty,
                    reg: Wpmr,
                }

                impl<'a> crate::structure::FWrite<'a, 24, u32, crate::pac::[<$ty:lower>]::wpmr::W>
                    for crate::pac::[<$ty:lower>]::wpmr::WpkeyW<
                        'a,
                        crate::pac::[<$ty:lower>]::wpmr::WpmrSpec,
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
                        -> &'a mut crate::pac::[<$ty:lower>]::wpmr::W
                    {
                        self.bits(value)
                    }
                }

                impl crate::write_protect::WpmrModify for crate::pac::[<$ty:lower>]::Wpmr {
                    fn modify<F>(&self, f: F)
                    where
                        for<'w> F: FnOnce(
                            &<Self as crate::write_protect::WpmrRead>::R,
                            &'w mut <Self as crate::write_protect::WpmrWrite>::W,
                        ) -> &'w mut <Self as crate::write_protect::WpmrWrite>::W,
                    {
                        self.modify(f)
                    }
                }

                impl crate::write_protect::WpsrRead for crate::pac::[<$ty:lower>]::Wpsr {
                    type R = crate::pac::[<$ty:lower>]::wpsr::R;
                    fn read(&self) -> Self::R {
                        self.read()
                    }
                }

                impl crate::write_protect::WpsrReadFields for crate::pac::[<$ty:lower>]::wpsr::R {
                    type Wpvs = crate::pac::[<$ty:lower>]::wpsr::[<$wpvs R>];
                    fn wpvs(&self) -> Self::Wpvs {
                        self.[<$wpvs:lower>]()
                    }
                    type Addr = $addr;
                    type Wpvsrc = crate::pac::[<$ty:lower>]::wpsr::[<$wpvsrc R>];
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
