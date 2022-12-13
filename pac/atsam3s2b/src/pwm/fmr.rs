#[doc = "Register `FMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<FMR_SPEC>);
#[doc = "Register `FMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<FMR_SPEC>);
#[doc = "Field `FPOL` reader - Fault Polarity (fault input bit varies from 0 to 5)"]
pub type FPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPOL` writer - Fault Polarity (fault input bit varies from 0 to 5)"]
pub type FPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FMOD` reader - Fault Activation Mode (fault input bit varies from 0 to 5)"]
pub type FMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMOD` writer - Fault Activation Mode (fault input bit varies from 0 to 5)"]
pub type FMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FFIL` reader - Fault Filtering (fault input bit varies from 0 to 5)"]
pub type FFIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFIL` writer - Fault Filtering (fault input bit varies from 0 to 5)"]
pub type FFIL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Fault Polarity (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpol(&self) -> FPOL_R {
        FPOL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fmod(&self) -> FMOD_R {
        FMOD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn ffil(&self) -> FFIL_R {
        FFIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Polarity (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fpol(&mut self) -> FPOL_W<0> {
        FPOL_W::new(self)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn fmod(&mut self) -> FMOD_W<8> {
        FMOD_W::new(self)
    }
    #[doc = "Bits 16:23 - Fault Filtering (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn ffil(&mut self) -> FFIL_W<16> {
        FFIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
