#[doc = "Register `DTF` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<DTF_SPEC>);
#[doc = "Register `DTF` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<DTF_SPEC>);
#[doc = "Field `DTF` reader - Deferred Transmission Frames"]
pub type DTF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTF` writer - Deferred Transmission Frames"]
pub type DTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Deferred Transmission Frames"]
    #[inline(always)]
    pub fn dtf(&self) -> DTF_R {
        DTF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Deferred Transmission Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dtf(&mut self) -> DTF_W<0> {
        DTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtf](index.html) module"]
pub struct DTF_SPEC;
impl crate::RegisterSpec for DTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtf::R](R) reader structure"]
impl crate::Readable for DTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtf::W](W) writer structure"]
impl crate::Writable for DTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTF to value 0"]
impl crate::Resettable for DTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
