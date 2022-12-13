#[doc = "Register `SA4B` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SA4B_SPEC>);
#[doc = "Register `SA4B` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<SA4B_SPEC>);
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SA4B_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address 4 Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa4b](index.html) module"]
pub struct SA4B_SPEC;
impl crate::RegisterSpec for SA4B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa4b::R](R) reader structure"]
impl crate::Readable for SA4B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa4b::W](W) writer structure"]
impl crate::Writable for SA4B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA4B to value 0"]
impl crate::Resettable for SA4B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
