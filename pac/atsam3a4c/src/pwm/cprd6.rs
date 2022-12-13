#[doc = "Register `CPRD6` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CPRD6_SPEC>);
#[doc = "Register `CPRD6` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CPRD6_SPEC>);
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPRD6_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
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
#[doc = "PWM Channel Period Register (ch_num = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd6](index.html) module"]
pub struct CPRD6_SPEC;
impl crate::RegisterSpec for CPRD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cprd6::R](R) reader structure"]
impl crate::Readable for CPRD6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cprd6::W](W) writer structure"]
impl crate::Writable for CPRD6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRD6 to value 0"]
impl crate::Resettable for CPRD6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
