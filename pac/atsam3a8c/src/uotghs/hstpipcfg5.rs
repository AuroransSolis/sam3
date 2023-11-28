#[doc = "Register `HSTPIPCFG5` reader"]
pub type R = crate::R<HSTPIPCFG5_SPEC>;
#[doc = "Register `HSTPIPCFG5` writer"]
pub type W = crate::W<HSTPIPCFG5_SPEC>;
#[doc = "Field `ALLOC` reader - Pipe Memory Allocate"]
pub type ALLOC_R = crate::BitReader;
#[doc = "Field `ALLOC` writer - Pipe Memory Allocate"]
pub type ALLOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBK` reader - Pipe Banks"]
pub type PBK_R = crate::FieldReader<PBK_A>;
#[doc = "Pipe Banks"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBK_A {
    #[doc = "0: Single-bank pipe"]
    _1Bank = 0,
    #[doc = "1: Double-bank pipe"]
    _2Bank = 1,
    #[doc = "2: Triple-bank pipe"]
    _3Bank = 2,
}
impl From<PBK_A> for u8 {
    #[inline(always)]
    fn from(variant: PBK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PBK_A {
    type Ux = u8;
}
impl PBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PBK_A> {
        match self.bits {
            0 => Some(PBK_A::_1Bank),
            1 => Some(PBK_A::_2Bank),
            2 => Some(PBK_A::_3Bank),
            _ => None,
        }
    }
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == PBK_A::_1Bank
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == PBK_A::_2Bank
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == PBK_A::_3Bank
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub type PBK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PBK_A>;
impl<'a, REG> PBK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut crate::W<REG> {
        self.variant(PBK_A::_1Bank)
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut crate::W<REG> {
        self.variant(PBK_A::_2Bank)
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut crate::W<REG> {
        self.variant(PBK_A::_3Bank)
    }
}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub type PSIZE_R = crate::FieldReader<PSIZE_A>;
#[doc = "Pipe Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE_A {
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
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSIZE_A {
    type Ux = u8;
}
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::_8Byte,
            1 => PSIZE_A::_16Byte,
            2 => PSIZE_A::_32Byte,
            3 => PSIZE_A::_64Byte,
            4 => PSIZE_A::_128Byte,
            5 => PSIZE_A::_256Byte,
            6 => PSIZE_A::_512Byte,
            7 => PSIZE_A::_1024Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PSIZE_A::_8Byte
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PSIZE_A::_16Byte
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PSIZE_A::_32Byte
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == PSIZE_A::_64Byte
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == PSIZE_A::_128Byte
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == PSIZE_A::_256Byte
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == PSIZE_A::_512Byte
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == PSIZE_A::_1024Byte
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PSIZE_A>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_8Byte)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_16Byte)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_32Byte)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_64Byte)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_128Byte)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_256Byte)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_512Byte)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::_1024Byte)
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PTOKEN_R = crate::FieldReader<PTOKEN_A>;
#[doc = "Pipe Token"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTOKEN_A {
    #[doc = "0: SETUP"]
    Setup = 0,
    #[doc = "1: IN"]
    In = 1,
    #[doc = "2: OUT"]
    Out = 2,
}
impl From<PTOKEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOKEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTOKEN_A {
    type Ux = u8;
}
impl PTOKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTOKEN_A> {
        match self.bits {
            0 => Some(PTOKEN_A::Setup),
            1 => Some(PTOKEN_A::In),
            2 => Some(PTOKEN_A::Out),
            _ => None,
        }
    }
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == PTOKEN_A::Setup
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == PTOKEN_A::In
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == PTOKEN_A::Out
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PTOKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PTOKEN_A>;
impl<'a, REG> PTOKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut crate::W<REG> {
        self.variant(PTOKEN_A::Setup)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(PTOKEN_A::In)
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(PTOKEN_A::Out)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AUTOSW_R = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AUTOSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PTYPE_R = crate::FieldReader<PTYPE_A>;
#[doc = "Pipe Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous"]
    Iso = 1,
    #[doc = "2: Bulk"]
    Blk = 2,
    #[doc = "3: Interrupt"]
    Intrpt = 3,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTYPE_A {
    type Ux = u8;
}
impl PTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTYPE_A {
        match self.bits {
            0 => PTYPE_A::Ctrl,
            1 => PTYPE_A::Iso,
            2 => PTYPE_A::Blk,
            3 => PTYPE_A::Intrpt,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == PTYPE_A::Ctrl
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == PTYPE_A::Iso
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == PTYPE_A::Blk
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == PTYPE_A::Intrpt
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PTYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PTYPE_A>;
impl<'a, REG> PTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Ctrl)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Iso)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Blk)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut crate::W<REG> {
        self.variant(PTYPE_A::Intrpt)
    }
}
#[doc = "Field `PEPNUM` reader - Pipe Endpoint Number"]
pub type PEPNUM_R = crate::FieldReader;
#[doc = "Field `PEPNUM` writer - Pipe Endpoint Number"]
pub type PEPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INTFRQ` reader - Pipe Interrupt Request Frequency"]
pub type INTFRQ_R = crate::FieldReader;
#[doc = "Field `INTFRQ` writer - Pipe Interrupt Request Frequency"]
pub type INTFRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PBK_R {
        PBK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&self) -> PEPNUM_R {
        PEPNUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Pipe Interrupt Request Frequency"]
    #[inline(always)]
    pub fn intfrq(&self) -> INTFRQ_R {
        INTFRQ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> ALLOC_W<HSTPIPCFG5_SPEC> {
        ALLOC_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    #[must_use]
    pub fn pbk(&mut self) -> PBK_W<HSTPIPCFG5_SPEC> {
        PBK_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<HSTPIPCFG5_SPEC> {
        PSIZE_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    #[must_use]
    pub fn ptoken(&mut self) -> PTOKEN_W<HSTPIPCFG5_SPEC> {
        PTOKEN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    #[must_use]
    pub fn autosw(&mut self) -> AUTOSW_W<HSTPIPCFG5_SPEC> {
        AUTOSW_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PTYPE_W<HSTPIPCFG5_SPEC> {
        PTYPE_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn pepnum(&mut self) -> PEPNUM_W<HSTPIPCFG5_SPEC> {
        PEPNUM_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Pipe Interrupt Request Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn intfrq(&mut self) -> INTFRQ_W<HSTPIPCFG5_SPEC> {
        INTFRQ_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Pipe Configuration Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipcfg5::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstpipcfg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPCFG5_SPEC;
impl crate::RegisterSpec for HSTPIPCFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipcfg5::R`](R) reader structure"]
impl crate::Readable for HSTPIPCFG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstpipcfg5::W`](W) writer structure"]
impl crate::Writable for HSTPIPCFG5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
