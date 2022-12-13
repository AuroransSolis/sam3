#[doc = "Register `TBQP` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<TBQP_SPEC>);
#[doc = "Register `TBQP` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<TBQP_SPEC>);
#[doc = "Field `ADDR` reader - Transmit buffer queue pointer address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Transmit buffer queue pointer address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBQP_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue pointer address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<2> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Queue Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqp](index.html) module"]
pub struct TBQP_SPEC;
impl crate::RegisterSpec for TBQP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbqp::R](R) reader structure"]
impl crate::Readable for TBQP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbqp::W](W) writer structure"]
impl crate::Writable for TBQP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBQP to value 0"]
impl crate::Resettable for TBQP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
