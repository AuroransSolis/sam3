#[doc = "Register `BR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<BR_SPEC>);
#[doc = "Register `BR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<BR_SPEC>);
#[doc = "Field `PHASE2` reader - Phase 2 segment"]
pub type PHASE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHASE2` writer - Phase 2 segment"]
pub type PHASE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PHASE1` reader - Phase 1 segment"]
pub type PHASE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHASE1` writer - Phase 1 segment"]
pub type PHASE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PROPAG` reader - Programming time segment"]
pub type PROPAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROPAG` writer - Programming time segment"]
pub type PROPAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SJW` reader - Re-synchronization jump width"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - Re-synchronization jump width"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BRP` reader - Baudrate Prescaler."]
pub type BRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRP` writer - Baudrate Prescaler."]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SMP` reader - Sampling Mode"]
pub type SMP_R = crate::BitReader<SMP_A>;
#[doc = "Sampling Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMP_A {
    #[doc = "0: The incoming bit stream is sampled once at sample point."]
    Once = 0,
    #[doc = "1: The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    Three = 1,
}
impl From<SMP_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as u8 != 0
    }
}
impl SMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            false => SMP_A::Once,
            true => SMP_A::Three,
        }
    }
    #[doc = "Checks if the value of the field is `Once`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == SMP_A::Once
    }
    #[doc = "Checks if the value of the field is `Three`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == SMP_A::Three
    }
}
#[doc = "Field `SMP` writer - Sampling Mode"]
pub type SMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BR_SPEC, SMP_A, O>;
impl<'a, const O: u8> SMP_W<'a, O> {
    #[doc = "The incoming bit stream is sampled once at sample point."]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(SMP_A::Once)
    }
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(SMP_A::Three)
    }
}
impl R {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    pub fn propag(&self) -> PROPAG_R {
        PROPAG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    #[must_use]
    pub fn phase2(&mut self) -> PHASE2_W<0> {
        PHASE2_W::new(self)
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    #[must_use]
    pub fn phase1(&mut self) -> PHASE1_W<4> {
        PHASE1_W::new(self)
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    #[must_use]
    pub fn propag(&mut self) -> PROPAG_W<8> {
        PROPAG_W::new(self)
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<12> {
        SJW_W::new(self)
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<16> {
        BRP_W::new(self)
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<24> {
        SMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baudrate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](index.html) module"]
pub struct BR_SPEC;
impl crate::RegisterSpec for BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [br::R](R) reader structure"]
impl crate::Readable for BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [br::W](W) writer structure"]
impl crate::Writable for BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BR to value 0"]
impl crate::Resettable for BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
