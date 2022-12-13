#[doc = "Register `SR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SR_SPEC>);
#[doc = "Field `ALMS` reader - Real-time Alarm Status"]
pub type ALMS_R = crate::BitReader<bool>;
#[doc = "Field `RTTINC` reader - Real-time Timer Increment"]
pub type RTTINC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Real-time Alarm Status"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time Timer Increment"]
    #[inline(always)]
    pub fn rttinc(&self) -> RTTINC_R {
        RTTINC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
