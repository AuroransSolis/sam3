#[doc = "Register `MODE0` reader"]
pub type R = crate::R<MODE0_SPEC>;
#[doc = "Register `MODE0` writer"]
pub type W = crate::W<MODE0_SPEC>;
#[doc = "Field `READ_MODE` reader - "]
pub type READ_MODE_R = crate::BitReader;
#[doc = "Field `READ_MODE` writer - "]
pub type READ_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MODE` reader - "]
pub type WRITE_MODE_R = crate::BitReader;
#[doc = "Field `WRITE_MODE` writer - "]
pub type WRITE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type EXNW_MODE_R = crate::FieldReader<EXNW_MODE_A>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "2: Frozen Mode"]
    Frozen = 2,
    #[doc = "3: Ready Mode"]
    Ready = 3,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXNW_MODE_A {
    type Ux = u8;
}
impl EXNW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXNW_MODE_A> {
        match self.bits {
            0 => Some(EXNW_MODE_A::Disabled),
            2 => Some(EXNW_MODE_A::Frozen),
            3 => Some(EXNW_MODE_A::Ready),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODE_A::Disabled
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODE_A::Frozen
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODE_A::Ready
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type EXNW_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXNW_MODE_A>;
impl<'a, REG> EXNW_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXNW_MODE_A::Disabled)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(EXNW_MODE_A::Frozen)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(EXNW_MODE_A::Ready)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TDF_CYCLES_R = crate::FieldReader;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TDF_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TDF_MODE_R = crate::BitReader;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TDF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub type PMEN_R = crate::BitReader;
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub type PMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Page Size"]
pub type PS_R = crate::FieldReader<PS_A>;
#[doc = "Page Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: 4-byte page"]
    _4Byte = 0,
    #[doc = "1: 8-byte page"]
    _8Byte = 1,
    #[doc = "2: 16-byte page"]
    _16Byte = 2,
    #[doc = "3: 32-byte page"]
    _32Byte = 3,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PS_A {
    type Ux = u8;
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_4Byte,
            1 => PS_A::_8Byte,
            2 => PS_A::_16Byte,
            3 => PS_A::_32Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == PS_A::_4Byte
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PS_A::_8Byte
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PS_A::_16Byte
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PS_A::_32Byte
    }
}
#[doc = "Field `PS` writer - Page Size"]
pub type PS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PS_A>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::_4Byte)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::_8Byte)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::_16Byte)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::_32Byte)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> READ_MODE_W<MODE0_SPEC> {
        READ_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn write_mode(&mut self) -> WRITE_MODE_W<MODE0_SPEC> {
        WRITE_MODE_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W<MODE0_SPEC> {
        EXNW_MODE_W::new(self, 4)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W<MODE0_SPEC> {
        TDF_CYCLES_W::new(self, 16)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W<MODE0_SPEC> {
        TDF_MODE_W::new(self, 20)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pmen(&mut self) -> PMEN_W<MODE0_SPEC> {
        PMEN_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<MODE0_SPEC> {
        PS_W::new(self, 28)
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
#[doc = "SMC Mode Register (CS_number = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE0_SPEC;
impl crate::RegisterSpec for MODE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode0::R`](R) reader structure"]
impl crate::Readable for MODE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode0::W`](W) writer structure"]
impl crate::Writable for MODE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE0 to value 0x1000_0003"]
impl crate::Resettable for MODE0_SPEC {
    const RESET_VALUE: u32 = 0x1000_0003;
}
