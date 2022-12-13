#[doc = "Register `TIM` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<TIM_SPEC>);
#[doc = "Field `TIMER` reader - Timer"]
pub type TIMER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](index.html) module"]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim::R](R) reader structure"]
impl crate::Readable for TIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
