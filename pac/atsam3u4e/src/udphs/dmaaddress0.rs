#[doc = "Register `DMAADDRESS0` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<DMAADDRESS0_SPEC>);
#[doc = "Register `DMAADDRESS0` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<DMAADDRESS0_SPEC>);
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub type BUFF_ADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub type BUFF_ADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAADDRESS0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BUFF_ADD_R {
        BUFF_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    #[must_use]
    pub fn buff_add(&mut self) -> BUFF_ADD_W<0> {
        BUFF_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDPHS DMA Channel Address Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddress0](index.html) module"]
pub struct DMAADDRESS0_SPEC;
impl crate::RegisterSpec for DMAADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaaddress0::R](R) reader structure"]
impl crate::Readable for DMAADDRESS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaaddress0::W](W) writer structure"]
impl crate::Writable for DMAADDRESS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAADDRESS0 to value 0"]
impl crate::Resettable for DMAADDRESS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
