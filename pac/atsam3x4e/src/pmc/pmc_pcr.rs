#[doc = "Register `PMC_PCR` reader"]
pub struct R(crate::R<PMC_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_PCR` writer"]
pub struct W(crate::W<PMC_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCR_SPEC>;
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
impl From<crate::W<PMC_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - Peripheral ID"]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - Peripheral ID"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CMD` reader - Command"]
pub struct CMD_R(crate::FieldReader<bool, bool>);
impl CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Divisor Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: Peripheral clock is MCK"]
    PERIPH_DIV_MCK = 0,
    #[doc = "1: Peripheral clock is MCK/2"]
    PERIPH_DIV2_MCK = 1,
    #[doc = "2: Peripheral clock is MCK/4"]
    PERIPH_DIV4_MCK = 2,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIV` reader - Divisor Value"]
pub struct DIV_R(crate::FieldReader<u8, DIV_A>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIV_A> {
        match self.bits {
            0 => Some(DIV_A::PERIPH_DIV_MCK),
            1 => Some(DIV_A::PERIPH_DIV2_MCK),
            2 => Some(DIV_A::PERIPH_DIV4_MCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV_MCK`"]
    #[inline(always)]
    pub fn is_periph_div_mck(&self) -> bool {
        **self == DIV_A::PERIPH_DIV_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV2_MCK`"]
    #[inline(always)]
    pub fn is_periph_div2_mck(&self) -> bool {
        **self == DIV_A::PERIPH_DIV2_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV4_MCK`"]
    #[inline(always)]
    pub fn is_periph_div4_mck(&self) -> bool {
        **self == DIV_A::PERIPH_DIV4_MCK
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, DIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Divisor Value"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral clock is MCK"]
    #[inline(always)]
    pub fn periph_div_mck(self) -> &'a mut W {
        self.variant(DIV_A::PERIPH_DIV_MCK)
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline(always)]
    pub fn periph_div2_mck(self) -> &'a mut W {
        self.variant(DIV_A::PERIPH_DIV2_MCK)
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline(always)]
    pub fn periph_div4_mck(self) -> &'a mut W {
        self.variant(DIV_A::PERIPH_DIV4_MCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `EN` reader - Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcr](index.html) module"]
pub struct PMC_PCR_SPEC;
impl crate::RegisterSpec for PMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pcr::R](R) reader structure"]
impl crate::Readable for PMC_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcr::W](W) writer structure"]
impl crate::Writable for PMC_PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_PCR to value 0"]
impl crate::Resettable for PMC_PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
