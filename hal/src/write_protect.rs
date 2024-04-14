pub trait WriteProtectKey {
    /// `const` value to insert into the `WPKEY` field of the write protection register of a
    /// peripheral with write protection capability.
    const WPKEY: u32;
}

pub trait WriteProtect: WriteProtectKey {
    fn enable_writeprotect(&mut self);
    fn disable_writeprotect(&mut self);
    fn writeprotect_enabled(&self) -> bool;
    /// Check if a peripheral with write protection capability is indicating a write protection
    /// error.
    fn writeprotect_error(&self) -> bool;

    // type AddrType;
    /// Read the value from the `WPROTADDR` field of the write protection status register of a
    /// peripheral with write protection capability without checking the `WPROTERR` flag.
    ///
    /// # Safety
    ///
    /// The value held in this register may be nonsensical if the `WPROTERR` flag is not indicating
    /// that a write protection error has occured, thus it is recommended to use the
    /// [`writeprotect_error_addr`](crate::write_protect::WriteProtect::writeprotect_error_addr)
    /// method instead.
    unsafe fn writeprotect_error_addr_unchecked(&self) -> u16;

    /// Check if the write protect status register of a peripheral with write protection capability
    /// is indicating that a write protection error has occurred, and if it has, return the address
    /// of the register write request that generated the error. Otherwise, `None` is returned.
    fn writeprotect_error_addr(&self) -> Option<u16> {
        if self.writeprotect_error() {
            unsafe { Some(self.writeprotect_error_addr_unchecked()) }
        } else {
            None
        }
    }
}

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
