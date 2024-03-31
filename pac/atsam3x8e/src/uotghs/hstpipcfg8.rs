#[doc = "Register `HSTPIPCFG8` reader"]
pub type R = crate::R<Hstpipcfg8Spec>;
#[doc = "Register `HSTPIPCFG8` writer"]
pub type W = crate::W<Hstpipcfg8Spec>;
#[doc = "Field `ALLOC` reader - Pipe Memory Allocate"]
pub type AllocR = crate::BitReader;
#[doc = "Field `ALLOC` writer - Pipe Memory Allocate"]
pub type AllocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pipe Banks"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbk {
    #[doc = "0: Single-bank pipe"]
    _1Bank = 0,
    #[doc = "1: Double-bank pipe"]
    _2Bank = 1,
    #[doc = "2: Triple-bank pipe"]
    _3Bank = 2,
}
impl From<Pbk> for u8 {
    #[inline(always)]
    fn from(variant: Pbk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pbk {
    type Ux = u8;
}
impl crate::IsEnum for Pbk {}
#[doc = "Field `PBK` reader - Pipe Banks"]
pub type PbkR = crate::FieldReader<Pbk>;
impl PbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pbk> {
        match self.bits {
            0 => Some(Pbk::_1Bank),
            1 => Some(Pbk::_2Bank),
            2 => Some(Pbk::_3Bank),
            _ => None,
        }
    }
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == Pbk::_1Bank
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == Pbk::_2Bank
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == Pbk::_3Bank
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub type PbkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pbk>;
impl<'a, REG> PbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Pbk::_1Bank)
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Pbk::_2Bank)
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut crate::W<REG> {
        self.variant(Pbk::_3Bank)
    }
}
#[doc = "Pipe Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psize {
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
impl From<Psize> for u8 {
    #[inline(always)]
    fn from(variant: Psize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psize {
    type Ux = u8;
}
impl crate::IsEnum for Psize {}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub type PsizeR = crate::FieldReader<Psize>;
impl PsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psize {
        match self.bits {
            0 => Psize::_8Byte,
            1 => Psize::_16Byte,
            2 => Psize::_32Byte,
            3 => Psize::_64Byte,
            4 => Psize::_128Byte,
            5 => Psize::_256Byte,
            6 => Psize::_512Byte,
            7 => Psize::_1024Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Psize::_8Byte
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Psize::_16Byte
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Psize::_32Byte
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == Psize::_64Byte
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == Psize::_128Byte
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == Psize::_256Byte
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == Psize::_512Byte
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == Psize::_1024Byte
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Psize, crate::Safe>;
impl<'a, REG> PsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_8Byte)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_16Byte)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_32Byte)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_64Byte)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_128Byte)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_256Byte)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_512Byte)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psize::_1024Byte)
    }
}
#[doc = "Pipe Token"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ptoken {
    #[doc = "0: SETUP"]
    Setup = 0,
    #[doc = "1: IN"]
    In = 1,
    #[doc = "2: OUT"]
    Out = 2,
}
impl From<Ptoken> for u8 {
    #[inline(always)]
    fn from(variant: Ptoken) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptoken {
    type Ux = u8;
}
impl crate::IsEnum for Ptoken {}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PtokenR = crate::FieldReader<Ptoken>;
impl PtokenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ptoken> {
        match self.bits {
            0 => Some(Ptoken::Setup),
            1 => Some(Ptoken::In),
            2 => Some(Ptoken::Out),
            _ => None,
        }
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == Ptoken::Setup
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Ptoken::In
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Ptoken::Out
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PtokenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ptoken>;
impl<'a, REG> PtokenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut crate::W<REG> {
        self.variant(Ptoken::Setup)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Ptoken::In)
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Ptoken::Out)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AutoswR = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AutoswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pipe Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ptype {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous"]
    Iso = 1,
    #[doc = "2: Bulk"]
    Blk = 2,
    #[doc = "3: Interrupt"]
    Intrpt = 3,
}
impl From<Ptype> for u8 {
    #[inline(always)]
    fn from(variant: Ptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptype {
    type Ux = u8;
}
impl crate::IsEnum for Ptype {}
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PtypeR = crate::FieldReader<Ptype>;
impl PtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ptype {
        match self.bits {
            0 => Ptype::Ctrl,
            1 => Ptype::Iso,
            2 => Ptype::Blk,
            3 => Ptype::Intrpt,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == Ptype::Ctrl
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Ptype::Iso
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == Ptype::Blk
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == Ptype::Intrpt
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ptype, crate::Safe>;
impl<'a, REG> PtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Ctrl)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Iso)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Blk)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut crate::W<REG> {
        self.variant(Ptype::Intrpt)
    }
}
#[doc = "Field `PEPNUM` reader - Pipe Endpoint Number"]
pub type PepnumR = crate::FieldReader;
#[doc = "Field `PEPNUM` writer - Pipe Endpoint Number"]
pub type PepnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTFRQ` reader - Pipe Interrupt Request Frequency"]
pub type IntfrqR = crate::FieldReader;
#[doc = "Field `INTFRQ` writer - Pipe Interrupt Request Frequency"]
pub type IntfrqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> AllocR {
        AllocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PbkR {
        PbkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PtokenR {
        PtokenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AutoswR {
        AutoswR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PtypeR {
        PtypeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&self) -> PepnumR {
        PepnumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Pipe Interrupt Request Frequency"]
    #[inline(always)]
    pub fn intfrq(&self) -> IntfrqR {
        IntfrqR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> AllocW<Hstpipcfg8Spec> {
        AllocW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    #[must_use]
    pub fn pbk(&mut self) -> PbkW<Hstpipcfg8Spec> {
        PbkW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PsizeW<Hstpipcfg8Spec> {
        PsizeW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    #[must_use]
    pub fn ptoken(&mut self) -> PtokenW<Hstpipcfg8Spec> {
        PtokenW::new(self, 8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    #[must_use]
    pub fn autosw(&mut self) -> AutoswW<Hstpipcfg8Spec> {
        AutoswW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PtypeW<Hstpipcfg8Spec> {
        PtypeW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn pepnum(&mut self) -> PepnumW<Hstpipcfg8Spec> {
        PepnumW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Pipe Interrupt Request Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn intfrq(&mut self) -> IntfrqW<Hstpipcfg8Spec> {
        IntfrqW::new(self, 24)
    }
}
#[doc = "Host Pipe Configuration Register (n = 0) 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg8::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hstpipcfg8Spec;
impl crate::RegisterSpec for Hstpipcfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipcfg8::R`](R) reader structure"]
impl crate::Readable for Hstpipcfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`hstpipcfg8::W`](W) writer structure"]
impl crate::Writable for Hstpipcfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
