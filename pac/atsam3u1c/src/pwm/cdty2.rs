#[doc = "Register `CDTY2` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CDTY2_SPEC>);
#[doc = "Register `CDTY2` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CDTY2_SPEC>);
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub type CDTY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub type CDTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDTY2_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R {
        CDTY_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cdty(&mut self) -> CDTY_W<0> {
        CDTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty2](index.html) module"]
pub struct CDTY2_SPEC;
impl crate::RegisterSpec for CDTY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdty2::R](R) reader structure"]
impl crate::Readable for CDTY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdty2::W](W) writer structure"]
impl crate::Writable for CDTY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDTY2 to value 0"]
impl crate::Resettable for CDTY2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
