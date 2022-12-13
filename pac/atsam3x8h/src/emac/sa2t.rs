#[doc = "Register `SA2T` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SA2T_SPEC>);
#[doc = "Register `SA2T` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<SA2T_SPEC>);
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SA2T_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
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
#[doc = "Specific Address 2 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa2t](index.html) module"]
pub struct SA2T_SPEC;
impl crate::RegisterSpec for SA2T_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa2t::R](R) reader structure"]
impl crate::Readable for SA2T_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa2t::W](W) writer structure"]
impl crate::Writable for SA2T_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA2T to value 0"]
impl crate::Resettable for SA2T_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
