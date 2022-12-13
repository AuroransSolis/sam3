#[doc = "Register `EMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<EMR_SPEC>);
#[doc = "Register `EMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<EMR_SPEC>);
#[doc = "Field `OFFMODES` reader - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
pub type OFFMODES_R = crate::BitReader<bool>;
#[doc = "Field `OFFMODES` writer - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
pub type OFFMODES_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `OFF_MODE_STARTUP_TIME` reader - Startup Time"]
pub type OFF_MODE_STARTUP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFF_MODE_STARTUP_TIME` writer - Startup Time"]
pub type OFF_MODE_STARTUP_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
    #[inline(always)]
    pub fn offmodes(&self) -> OFFMODES_R {
        OFFMODES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Startup Time"]
    #[inline(always)]
    pub fn off_mode_startup_time(&self) -> OFF_MODE_STARTUP_TIME_R {
        OFF_MODE_STARTUP_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Off Mode if Sleep Bit (ADC12B_MR) = 1"]
    #[inline(always)]
    #[must_use]
    pub fn offmodes(&mut self) -> OFFMODES_W<0> {
        OFFMODES_W::new(self)
    }
    #[doc = "Bits 16:23 - Startup Time"]
    #[inline(always)]
    #[must_use]
    pub fn off_mode_startup_time(&mut self) -> OFF_MODE_STARTUP_TIME_W<16> {
        OFF_MODE_STARTUP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
