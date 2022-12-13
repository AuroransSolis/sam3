#[doc = "Register `RLE` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RLE_SPEC>);
#[doc = "Register `RLE` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RLE_SPEC>);
#[doc = "Field `RLFM` reader - Receive Length Field Mismatch"]
pub type RLFM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLFM` writer - Receive Length Field Mismatch"]
pub type RLFM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&self) -> RLFM_R {
        RLFM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn rlfm(&mut self) -> RLFM_W<0> {
        RLFM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Received Length Field Mismatch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rle](index.html) module"]
pub struct RLE_SPEC;
impl crate::RegisterSpec for RLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rle::R](R) reader structure"]
impl crate::Readable for RLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rle::W](W) writer structure"]
impl crate::Writable for RLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLE to value 0"]
impl crate::Resettable for RLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
