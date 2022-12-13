#[doc = "Register `RC0R` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<RC0R_SPEC>);
#[doc = "Register `RC0R` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<RC0R_SPEC>);
#[doc = "Field `CP0` reader - Receive Compare Data 0"]
pub type CP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CP0` writer - Receive Compare Data 0"]
pub type CP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC0R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<0> {
        CP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Compare 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc0r](index.html) module"]
pub struct RC0R_SPEC;
impl crate::RegisterSpec for RC0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc0r::R](R) reader structure"]
impl crate::Readable for RC0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc0r::W](W) writer structure"]
impl crate::Writable for RC0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC0R to value 0"]
impl crate::Resettable for RC0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
