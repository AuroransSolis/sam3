#[doc = "Register `BANK` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<BANK_SPEC>);
#[doc = "Register `BANK` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<BANK_SPEC>);
#[doc = "Field `BANK` reader - Bank Identifier"]
pub type BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BANK` writer - Bank Identifier"]
pub type BANK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BANK_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Bank Identifier"]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn bank(&mut self) -> BANK_W<0> {
        BANK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Bank Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bank](index.html) module"]
pub struct BANK_SPEC;
impl crate::RegisterSpec for BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bank::R](R) reader structure"]
impl crate::Readable for BANK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bank::W](W) writer structure"]
impl crate::Writable for BANK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BANK to value 0"]
impl crate::Resettable for BANK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
