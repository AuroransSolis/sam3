#[doc = "Register `OCMS` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<OCMS_SPEC>);
#[doc = "Register `OCMS` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<OCMS_SPEC>);
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub type SMSE_R = crate::BitReader<bool>;
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub type SMSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
#[doc = "Field `SRSE` reader - SRAM Scrambling Enable"]
pub type SRSE_R = crate::BitReader<bool>;
#[doc = "Field `SRSE` writer - SRAM Scrambling Enable"]
pub type SRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    pub fn srse(&self) -> SRSE_R {
        SRSE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smse(&mut self) -> SMSE_W<0> {
        SMSE_W::new(self)
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srse(&mut self) -> SRSE_W<1> {
        SRSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](index.html) module"]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocms::R](R) reader structure"]
impl crate::Readable for OCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocms::W](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
