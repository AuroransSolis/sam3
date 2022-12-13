#[doc = "Register `CKGR_PLLBR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CKGR_PLLBR_SPEC>);
#[doc = "Register `CKGR_PLLBR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CKGR_PLLBR_SPEC>);
#[doc = "Field `DIVB` reader - Divider"]
pub type DIVB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVB` writer - Divider"]
pub type DIVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLBR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PLLBCOUNT` reader - PLLB Counter"]
pub type PLLBCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLBCOUNT` writer - PLLB Counter"]
pub type PLLBCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLBR_SPEC, u8, u8, 6, O>;
#[doc = "Field `MULB` reader - PLLB Multiplier"]
pub type MULB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MULB` writer - PLLB Multiplier"]
pub type MULB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLBR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PLLBCOUNT_R {
        PLLBCOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MULB_R {
        MULB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divider"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DIVB_W<0> {
        DIVB_W::new(self)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pllbcount(&mut self) -> PLLBCOUNT_W<8> {
        PLLBCOUNT_W::new(self)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn mulb(&mut self) -> MULB_W<16> {
        MULB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_pllbr](index.html) module"]
pub struct CKGR_PLLBR_SPEC;
impl crate::RegisterSpec for CKGR_PLLBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_pllbr::R](R) reader structure"]
impl crate::Readable for CKGR_PLLBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_pllbr::W](W) writer structure"]
impl crate::Writable for CKGR_PLLBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKGR_PLLBR to value 0x3f00"]
impl crate::Resettable for CKGR_PLLBR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f00;
}
