#[doc = "Register `FMR` reader"]
pub struct R(crate::R<FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMR` writer"]
pub struct W(crate::W<FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMR_SPEC>;
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
impl From<crate::W<FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDY` reader - Ready Interrupt Enable"]
pub type FRDY_R = crate::BitReader<bool>;
#[doc = "Field `FRDY` writer - Ready Interrupt Enable"]
pub type FRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMR_SPEC, bool, O>;
#[doc = "Field `FWS` reader - Flash Wait State"]
pub type FWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FWS` writer - Flash Wait State"]
pub type FWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCOD` reader - Sequential Code Optimization Disable"]
pub type SCOD_R = crate::BitReader<bool>;
#[doc = "Field `SCOD` writer - Sequential Code Optimization Disable"]
pub type SCOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMR_SPEC, bool, O>;
#[doc = "Field `FAM` reader - Flash Access Mode"]
pub type FAM_R = crate::BitReader<bool>;
#[doc = "Field `FAM` writer - Flash Access Mode"]
pub type FAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&self) -> SCOD_R {
        SCOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Flash Access Mode"]
    #[inline(always)]
    pub fn fam(&self) -> FAM_R {
        FAM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FRDY_W<0> {
        FRDY_W::new(self)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W<8> {
        FWS_W::new(self)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&mut self) -> SCOD_W<16> {
        SCOD_W::new(self)
    }
    #[doc = "Bit 24 - Flash Access Mode"]
    #[inline(always)]
    pub fn fam(&mut self) -> FAM_W<24> {
        FAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEFC Flash Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](index.html) module"]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmr::R](R) reader structure"]
impl crate::Readable for FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmr::W](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
