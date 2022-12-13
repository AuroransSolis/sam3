#[doc = "Register `TTGR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<TTGR_SPEC>);
#[doc = "Register `TTGR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<TTGR_SPEC>);
#[doc = "Field `TG` reader - Timeguard Value"]
pub type TG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TG` writer - Timeguard Value"]
pub type TG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTGR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<0> {
        TG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgr](index.html) module"]
pub struct TTGR_SPEC;
impl crate::RegisterSpec for TTGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttgr::R](R) reader structure"]
impl crate::Readable for TTGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttgr::W](W) writer structure"]
impl crate::Writable for TTGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTGR to value 0"]
impl crate::Resettable for TTGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
