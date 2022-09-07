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
pub type PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID` writer - Peripheral ID"]
pub type PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_PCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::BitReader<bool>;
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_PCR_SPEC, bool, O>;
#[doc = "Field `DIV` reader - Divisor Value"]
pub type DIV_R = crate::FieldReader<u8, DIV_A>;
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
impl DIV_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DIV_A::PERIPH_DIV_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV2_MCK`"]
    #[inline(always)]
    pub fn is_periph_div2_mck(&self) -> bool {
        *self == DIV_A::PERIPH_DIV2_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV4_MCK`"]
    #[inline(always)]
    pub fn is_periph_div4_mck(&self) -> bool {
        *self == DIV_A::PERIPH_DIV4_MCK
    }
}
#[doc = "Field `DIV` writer - Divisor Value"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_PCR_SPEC, u8, DIV_A, 2, O>;
impl<'a, const O: u8> DIV_W<'a, O> {
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
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_PCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W<0> {
        PID_W::new(self)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W<12> {
        CMD_W::new(self)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<28> {
        EN_W::new(self)
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
