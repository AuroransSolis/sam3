#[doc = "Register `CWGR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CWGR_SPEC>);
#[doc = "Register `CWGR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CWGR_SPEC>);
#[doc = "Field `CLDIV` reader - Clock Low Divider"]
pub type CLDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLDIV` writer - Clock Low Divider"]
pub type CLDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CHDIV` reader - Clock High Divider"]
pub type CHDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHDIV` writer - Clock High Divider"]
pub type CHDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKDIV` reader - Clock Divider"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - Clock Divider"]
pub type CKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&self) -> CLDIV_R {
        CLDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&self) -> CHDIV_R {
        CHDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cldiv(&mut self) -> CLDIV_W<0> {
        CLDIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    #[must_use]
    pub fn chdiv(&mut self) -> CHDIV_W<8> {
        CHDIV_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckdiv(&mut self) -> CKDIV_W<16> {
        CKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Waveform Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwgr](index.html) module"]
pub struct CWGR_SPEC;
impl crate::RegisterSpec for CWGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwgr::R](R) reader structure"]
impl crate::Readable for CWGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwgr::W](W) writer structure"]
impl crate::Writable for CWGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWGR to value 0"]
impl crate::Resettable for CWGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
