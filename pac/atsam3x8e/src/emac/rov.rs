#[doc = "Register `ROV` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ROV_SPEC>);
#[doc = "Register `ROV` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ROV_SPEC>);
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<0> {
        ROVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Overrun Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rov](index.html) module"]
pub struct ROV_SPEC;
impl crate::RegisterSpec for ROV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rov::R](R) reader structure"]
impl crate::Readable for ROV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rov::W](W) writer structure"]
impl crate::Writable for ROV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROV to value 0"]
impl crate::Resettable for ROV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
