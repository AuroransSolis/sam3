#[doc = "Register `CPRD2` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CPRD2_SPEC>);
#[doc = "Register `CPRD2` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CPRD2_SPEC>);
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPRD2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CPRD_W<0> {
        CPRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd2](index.html) module"]
pub struct CPRD2_SPEC;
impl crate::RegisterSpec for CPRD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cprd2::R](R) reader structure"]
impl crate::Readable for CPRD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cprd2::W](W) writer structure"]
impl crate::Writable for CPRD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRD2 to value 0"]
impl crate::Resettable for CPRD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
