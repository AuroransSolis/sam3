#[doc = "Register `SEQR2` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SEQR2_SPEC>);
#[doc = "Register `SEQR2` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<SEQR2_SPEC>);
#[doc = "Field `USCH9` reader - User Sequence Number 9"]
pub type USCH9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH9` writer - User Sequence Number 9"]
pub type USCH9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `USCH10` reader - User Sequence Number 10"]
pub type USCH10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH10` writer - User Sequence Number 10"]
pub type USCH10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `USCH11` reader - User Sequence Number 11"]
pub type USCH11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH11` writer - User Sequence Number 11"]
pub type USCH11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `USCH12` reader - User Sequence Number 12"]
pub type USCH12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH12` writer - User Sequence Number 12"]
pub type USCH12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `USCH13` reader - User Sequence Number 13"]
pub type USCH13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH13` writer - User Sequence Number 13"]
pub type USCH13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `USCH14` reader - User Sequence Number 14"]
pub type USCH14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH14` writer - User Sequence Number 14"]
pub type USCH14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `USCH15` reader - User Sequence Number 15"]
pub type USCH15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USCH15` writer - User Sequence Number 15"]
pub type USCH15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 12"]
    #[inline(always)]
    pub fn usch12(&self) -> USCH12_R {
        USCH12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 13"]
    #[inline(always)]
    pub fn usch13(&self) -> USCH13_R {
        USCH13_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 14"]
    #[inline(always)]
    pub fn usch14(&self) -> USCH14_R {
        USCH14_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 15"]
    #[inline(always)]
    pub fn usch15(&self) -> USCH15_R {
        USCH15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 9"]
    #[inline(always)]
    #[must_use]
    pub fn usch9(&mut self) -> USCH9_W<0> {
        USCH9_W::new(self)
    }
    #[doc = "Bits 4:7 - User Sequence Number 10"]
    #[inline(always)]
    #[must_use]
    pub fn usch10(&mut self) -> USCH10_W<4> {
        USCH10_W::new(self)
    }
    #[doc = "Bits 8:11 - User Sequence Number 11"]
    #[inline(always)]
    #[must_use]
    pub fn usch11(&mut self) -> USCH11_W<8> {
        USCH11_W::new(self)
    }
    #[doc = "Bits 12:15 - User Sequence Number 12"]
    #[inline(always)]
    #[must_use]
    pub fn usch12(&mut self) -> USCH12_W<12> {
        USCH12_W::new(self)
    }
    #[doc = "Bits 16:19 - User Sequence Number 13"]
    #[inline(always)]
    #[must_use]
    pub fn usch13(&mut self) -> USCH13_W<16> {
        USCH13_W::new(self)
    }
    #[doc = "Bits 20:23 - User Sequence Number 14"]
    #[inline(always)]
    #[must_use]
    pub fn usch14(&mut self) -> USCH14_W<20> {
        USCH14_W::new(self)
    }
    #[doc = "Bits 24:27 - User Sequence Number 15"]
    #[inline(always)]
    #[must_use]
    pub fn usch15(&mut self) -> USCH15_W<24> {
        USCH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Sequence Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqr2](index.html) module"]
pub struct SEQR2_SPEC;
impl crate::RegisterSpec for SEQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqr2::R](R) reader structure"]
impl crate::Readable for SEQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqr2::W](W) writer structure"]
impl crate::Writable for SEQR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQR2 to value 0"]
impl crate::Resettable for SEQR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
