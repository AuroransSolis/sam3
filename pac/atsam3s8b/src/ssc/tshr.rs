#[doc = "Register `TSHR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<TSHR_SPEC>);
#[doc = "Register `TSHR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<TSHR_SPEC>);
#[doc = "Field `TSDAT` reader - Transmit Synchronization Data"]
pub type TSDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSDAT` writer - Transmit Synchronization Data"]
pub type TSDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSHR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    pub fn tsdat(&self) -> TSDAT_R {
        TSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    #[must_use]
    pub fn tsdat(&mut self) -> TSDAT_W<0> {
        TSDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tshr](index.html) module"]
pub struct TSHR_SPEC;
impl crate::RegisterSpec for TSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tshr::R](R) reader structure"]
impl crate::Readable for TSHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tshr::W](W) writer structure"]
impl crate::Writable for TSHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSHR to value 0"]
impl crate::Resettable for TSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
