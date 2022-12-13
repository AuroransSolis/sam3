#[doc = "Register `ECOL` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<ECOL_SPEC>);
#[doc = "Register `ECOL` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<ECOL_SPEC>);
#[doc = "Field `EXCOL` reader - Excessive Collisions"]
pub type EXCOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXCOL` writer - Excessive Collisions"]
pub type EXCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECOL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    #[must_use]
    pub fn excol(&mut self) -> EXCOL_W<0> {
        EXCOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecol](index.html) module"]
pub struct ECOL_SPEC;
impl crate::RegisterSpec for ECOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecol::R](R) reader structure"]
impl crate::Readable for ECOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecol::W](W) writer structure"]
impl crate::Writable for ECOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECOL to value 0"]
impl crate::Resettable for ECOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
