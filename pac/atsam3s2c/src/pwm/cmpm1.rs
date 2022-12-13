#[doc = "Register `CMPM1` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CMPM1_SPEC>);
#[doc = "Register `CMPM1` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CMPM1_SPEC>);
#[doc = "Field `CEN` reader - Comparison x Enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Comparison x Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPM1_SPEC, bool, O>;
#[doc = "Field `CTR` reader - Comparison x Trigger"]
pub type CTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTR` writer - Comparison x Trigger"]
pub type CTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPR` reader - Comparison x Period"]
pub type CPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPR` writer - Comparison x Period"]
pub type CPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPRCNT` reader - Comparison x Period Counter"]
pub type CPRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPRCNT` writer - Comparison x Period Counter"]
pub type CPRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CUPR` reader - Comparison x Update Period"]
pub type CUPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUPR` writer - Comparison x Update Period"]
pub type CUPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CUPRCNT` reader - Comparison x Update Period Counter"]
pub type CUPRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUPRCNT` writer - Comparison x Update Period Counter"]
pub type CUPRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPM1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    pub fn cprcnt(&self) -> CPRCNT_R {
        CPRCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    pub fn cupr(&self) -> CUPR_R {
        CUPR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    pub fn cuprcnt(&self) -> CUPRCNT_R {
        CUPRCNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<4> {
        CTR_W::new(self)
    }
    #[doc = "Bits 8:11 - Comparison x Period"]
    #[inline(always)]
    #[must_use]
    pub fn cpr(&mut self) -> CPR_W<8> {
        CPR_W::new(self)
    }
    #[doc = "Bits 12:15 - Comparison x Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cprcnt(&mut self) -> CPRCNT_W<12> {
        CPRCNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn cupr(&mut self) -> CUPR_W<16> {
        CUPR_W::new(self)
    }
    #[doc = "Bits 20:23 - Comparison x Update Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cuprcnt(&mut self) -> CUPRCNT_W<20> {
        CUPRCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 1 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpm1](index.html) module"]
pub struct CMPM1_SPEC;
impl crate::RegisterSpec for CMPM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpm1::R](R) reader structure"]
impl crate::Readable for CMPM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpm1::W](W) writer structure"]
impl crate::Writable for CMPM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPM1 to value 0"]
impl crate::Resettable for CMPM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
