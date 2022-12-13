#[doc = "Register `MATRIX_WPMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MATRIX_WPMR_SPEC>);
#[doc = "Register `MATRIX_WPMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MATRIX_WPMR_SPEC>);
#[doc = "Field `WPEN` reader - Write Protect ENable"]
pub type WPEN_R = crate::BitReader<bool>;
#[doc = "Field `WPEN` writer - Write Protect ENable"]
pub type WPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MATRIX_WPMR_SPEC, bool, O>;
#[doc = "Field `WPKEY` reader - Write Protect KEY (Write-only)"]
pub type WPKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WPKEY` writer - Write Protect KEY (Write-only)"]
pub type WPKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATRIX_WPMR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<0> {
        WPEN_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpmr](index.html) module"]
pub struct MATRIX_WPMR_SPEC;
impl crate::RegisterSpec for MATRIX_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_wpmr::R](R) reader structure"]
impl crate::Readable for MATRIX_WPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_wpmr::W](W) writer structure"]
impl crate::Writable for MATRIX_WPMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MATRIX_WPMR to value 0"]
impl crate::Resettable for MATRIX_WPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
