#[doc = "Register `MODE0` reader"]
pub struct R(crate::R<MODE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE0` writer"]
pub struct W(crate::W<MODE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MODE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_MODE` reader - "]
pub type READ_MODE_R = crate::BitReader<READ_MODE_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_MODE_A {
    #[doc = "0: The Read operation is controlled by the NCS signal."]
    NCS_CTRL = 0,
    #[doc = "1: The Read operation is controlled by the NRD signal."]
    NRD_CTRL = 1,
}
impl From<READ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_MODE_A {
        match self.bits {
            false => READ_MODE_A::NCS_CTRL,
            true => READ_MODE_A::NRD_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == READ_MODE_A::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NRD_CTRL`"]
    #[inline(always)]
    pub fn is_nrd_ctrl(&self) -> bool {
        *self == READ_MODE_A::NRD_CTRL
    }
}
#[doc = "Field `READ_MODE` writer - "]
pub type READ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, READ_MODE_A, O>;
impl<'a, const O: u8> READ_MODE_W<'a, O> {
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(READ_MODE_A::NCS_CTRL)
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn nrd_ctrl(self) -> &'a mut W {
        self.variant(READ_MODE_A::NRD_CTRL)
    }
}
#[doc = "Field `WRITE_MODE` reader - "]
pub type WRITE_MODE_R = crate::BitReader<WRITE_MODE_A>;
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_MODE_A {
    #[doc = "0: The Write operation is controller by the NCS signal."]
    NCS_CTRL = 0,
    #[doc = "1: The Write operation is controlled by the NWE signal."]
    NWE_CTRL = 1,
}
impl From<WRITE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_MODE_A {
        match self.bits {
            false => WRITE_MODE_A::NCS_CTRL,
            true => WRITE_MODE_A::NWE_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == WRITE_MODE_A::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NWE_CTRL`"]
    #[inline(always)]
    pub fn is_nwe_ctrl(&self) -> bool {
        *self == WRITE_MODE_A::NWE_CTRL
    }
}
#[doc = "Field `WRITE_MODE` writer - "]
pub type WRITE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, WRITE_MODE_A, O>;
impl<'a, const O: u8> WRITE_MODE_W<'a, O> {
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODE_A::NCS_CTRL)
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn nwe_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODE_A::NWE_CTRL)
    }
}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type EXNW_MODE_R = crate::FieldReader<u8, EXNW_MODE_A>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Frozen Mode"]
    FROZEN = 2,
    #[doc = "3: Ready Mode"]
    READY = 3,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        variant as _
    }
}
impl EXNW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXNW_MODE_A> {
        match self.bits {
            0 => Some(EXNW_MODE_A::DISABLED),
            2 => Some(EXNW_MODE_A::FROZEN),
            3 => Some(EXNW_MODE_A::READY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODE_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODE_A::READY
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type EXNW_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE0_SPEC, u8, EXNW_MODE_A, 2, O>;
impl<'a, const O: u8> EXNW_MODE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::READY)
    }
}
#[doc = "Field `BAT` reader - Byte Access Type"]
pub type BAT_R = crate::BitReader<bool>;
#[doc = "Field `BAT` writer - Byte Access Type"]
pub type BAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, bool, O>;
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::BitReader<DBW_A>;
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBW_A {
    #[doc = "0: 8-bit bus"]
    BIT_8 = 0,
    #[doc = "1: 16-bit bus"]
    BIT_16 = 1,
}
impl From<DBW_A> for bool {
    #[inline(always)]
    fn from(variant: DBW_A) -> Self {
        variant as u8 != 0
    }
}
impl DBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBW_A {
        match self.bits {
            false => DBW_A::BIT_8,
            true => DBW_A::BIT_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == DBW_A::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_16`"]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        *self == DBW_A::BIT_16
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, DBW_A, O>;
impl<'a, const O: u8> DBW_W<'a, O> {
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DBW_A::BIT_8)
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut W {
        self.variant(DBW_A::BIT_16)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TDF_CYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TDF_CYCLES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TDF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TDF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, bool, O>;
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
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 12) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> READ_MODE_W<0> {
        READ_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WRITE_MODE_W<1> {
        WRITE_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W<4> {
        EXNW_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&mut self) -> BAT_W<8> {
        BAT_W::new(self)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W<12> {
        DBW_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W<16> {
        TDF_CYCLES_W::new(self)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W<20> {
        TDF_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Mode Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode0](index.html) module"]
pub struct MODE0_SPEC;
impl crate::RegisterSpec for MODE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode0::R](R) reader structure"]
impl crate::Readable for MODE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode0::W](W) writer structure"]
impl crate::Writable for MODE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE0 to value 0x1000_0003"]
impl crate::Resettable for MODE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0003
    }
}
