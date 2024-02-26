#[doc = "Register `DEVEPTCFG9` reader"]
pub type R = crate::R<Deveptcfg9Spec>;
#[doc = "Register `DEVEPTCFG9` writer"]
pub type W = crate::W<Deveptcfg9Spec>;
#[doc = "Field `ALLOC` reader - Endpoint Memory Allocate"]
pub type AllocR = crate::BitReader;
#[doc = "Field `ALLOC` writer - Endpoint Memory Allocate"]
pub type AllocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Banks"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Epbk {
    #[doc = "0: Single-bank endpoint"]
    _1Bank = 0,
    #[doc = "1: Double-bank endpoint"]
    _2Bank = 1,
    #[doc = "2: Triple-bank endpoint"]
    _3Bank = 2,
}
impl From<Epbk> for u8 {
    #[inline(always)]
    fn from(variant: Epbk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Epbk {
    type Ux = u8;
}
#[doc = "Field `EPBK` reader - Endpoint Banks"]
pub type EpbkR = crate::FieldReader<Epbk>;
impl EpbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Epbk> {
        match self.bits {
            0 => Some(Epbk::_1Bank),
            1 => Some(Epbk::_2Bank),
            2 => Some(Epbk::_3Bank),
            _ => None,
        }
    }
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == Epbk::_1Bank
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == Epbk::_2Bank
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == Epbk::_3Bank
    }
}
#[doc = "Field `EPBK` writer - Endpoint Banks"]
pub type EpbkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Epbk>;
impl<'a, REG> EpbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Epbk::_1Bank)
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Epbk::_2Bank)
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Epbk::_3Bank)
    }
}
#[doc = "Endpoint Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Epsize {
    #[doc = "0: 8 bytes"]
    _8Byte = 0,
    #[doc = "1: 16 bytes"]
    _16Byte = 1,
    #[doc = "2: 32 bytes"]
    _32Byte = 2,
    #[doc = "3: 64 bytes"]
    _64Byte = 3,
    #[doc = "4: 128 bytes"]
    _128Byte = 4,
    #[doc = "5: 256 bytes"]
    _256Byte = 5,
    #[doc = "6: 512 bytes"]
    _512Byte = 6,
    #[doc = "7: 1024 bytes"]
    _1024Byte = 7,
}
impl From<Epsize> for u8 {
    #[inline(always)]
    fn from(variant: Epsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Epsize {
    type Ux = u8;
}
#[doc = "Field `EPSIZE` reader - Endpoint Size"]
pub type EpsizeR = crate::FieldReader<Epsize>;
impl EpsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epsize {
        match self.bits {
            0 => Epsize::_8Byte,
            1 => Epsize::_16Byte,
            2 => Epsize::_32Byte,
            3 => Epsize::_64Byte,
            4 => Epsize::_128Byte,
            5 => Epsize::_256Byte,
            6 => Epsize::_512Byte,
            7 => Epsize::_1024Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Epsize::_8Byte
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Epsize::_16Byte
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Epsize::_32Byte
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == Epsize::_64Byte
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == Epsize::_128Byte
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == Epsize::_256Byte
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == Epsize::_512Byte
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == Epsize::_1024Byte
    }
}
#[doc = "Field `EPSIZE` writer - Endpoint Size"]
pub type EpsizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Epsize>;
impl<'a, REG> EpsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_8Byte)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_16Byte)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_32Byte)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_64Byte)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_128Byte)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_256Byte)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_512Byte)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Epsize::_1024Byte)
    }
}
#[doc = "Endpoint Direction"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epdir {
    #[doc = "0: The endpoint direction is OUT."]
    Out = 0,
    #[doc = "1: The endpoint direction is IN (nor for control endpoints)."]
    In = 1,
}
impl From<Epdir> for bool {
    #[inline(always)]
    fn from(variant: Epdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EpdirR = crate::BitReader<Epdir>;
impl EpdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epdir {
        match self.bits {
            false => Epdir::Out,
            true => Epdir::In,
        }
    }
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Epdir::Out
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Epdir::In
    }
}
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG, Epdir>;
impl<'a, REG> EpdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Epdir::Out)
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Epdir::In)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AutoswR = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AutoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous"]
    Iso = 1,
    #[doc = "2: Bulk"]
    Blk = 2,
    #[doc = "3: Interrupt"]
    Intrpt = 3,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eptype {
        match self.bits {
            0 => Eptype::Ctrl,
            1 => Eptype::Iso,
            2 => Eptype::Blk,
            3 => Eptype::Intrpt,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == Eptype::Ctrl
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Eptype::Iso
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == Eptype::Blk
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == Eptype::Intrpt
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Eptype>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Ctrl)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Iso)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Blk)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Intrpt)
    }
}
#[doc = "Number of transaction per microframe for isochronous endpoint"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbtrans {
    #[doc = "0: reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0Trans = 0,
    #[doc = "1: default value: one transaction per micro-frame."]
    _1Trans = 1,
    #[doc = "2: 2 transactions per micro-frame. This endpoint should be configured as double-bank."]
    _2Trans = 2,
    #[doc = "3: 3 transactions per micro-frame. This endpoint should be configured as triple-bank."]
    _3Trans = 3,
}
impl From<Nbtrans> for u8 {
    #[inline(always)]
    fn from(variant: Nbtrans) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbtrans {
    type Ux = u8;
}
#[doc = "Field `NBTRANS` reader - Number of transaction per microframe for isochronous endpoint"]
pub type NbtransR = crate::FieldReader<Nbtrans>;
impl NbtransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbtrans {
        match self.bits {
            0 => Nbtrans::_0Trans,
            1 => Nbtrans::_1Trans,
            2 => Nbtrans::_2Trans,
            3 => Nbtrans::_3Trans,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn is_0_trans(&self) -> bool {
        *self == Nbtrans::_0Trans
    }
    #[doc = "default value: one transaction per micro-frame."]
    #[inline(always)]
    pub fn is_1_trans(&self) -> bool {
        *self == Nbtrans::_1Trans
    }
    #[doc = "2 transactions per micro-frame. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn is_2_trans(&self) -> bool {
        *self == Nbtrans::_2Trans
    }
    #[doc = "3 transactions per micro-frame. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn is_3_trans(&self) -> bool {
        *self == Nbtrans::_3Trans
    }
}
#[doc = "Field `NBTRANS` writer - Number of transaction per microframe for isochronous endpoint"]
pub type NbtransW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Nbtrans>;
impl<'a, REG> NbtransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn _0_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtrans::_0Trans)
    }
    #[doc = "default value: one transaction per micro-frame."]
    #[inline(always)]
    pub fn _1_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtrans::_1Trans)
    }
    #[doc = "2 transactions per micro-frame. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn _2_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtrans::_2Trans)
    }
    #[doc = "3 transactions per micro-frame. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn _3_trans(self) -> &'a mut crate::W<REG> {
        self.variant(Nbtrans::_3Trans)
    }
}
impl R {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> AllocR {
        AllocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&self) -> EpbkR {
        EpbkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EpsizeR {
        EpsizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AutoswR {
        AutoswR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Number of transaction per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&self) -> NbtransR {
        NbtransR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> AllocW<Deveptcfg9Spec> {
        AllocW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    #[must_use]
    pub fn epbk(&mut self) -> EpbkW<Deveptcfg9Spec> {
        EpbkW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    #[must_use]
    pub fn epsize(&mut self) -> EpsizeW<Deveptcfg9Spec> {
        EpsizeW::new(self, 4)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EpdirW<Deveptcfg9Spec> {
        EpdirW::new(self, 8)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    #[must_use]
    pub fn autosw(&mut self) -> AutoswW<Deveptcfg9Spec> {
        AutoswW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<Deveptcfg9Spec> {
        EptypeW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Number of transaction per microframe for isochronous endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn nbtrans(&mut self) -> NbtransW<Deveptcfg9Spec> {
        NbtransW::new(self, 13)
    }
}
#[doc = "Device Endpoint Configuration Register (n = 0) 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg9::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deveptcfg9Spec;
impl crate::RegisterSpec for Deveptcfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptcfg9::R`](R) reader structure"]
impl crate::Readable for Deveptcfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`deveptcfg9::W`](W) writer structure"]
impl crate::Writable for Deveptcfg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
