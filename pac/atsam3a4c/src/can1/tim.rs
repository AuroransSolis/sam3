#[doc = "Register `TIM` reader"]
pub type R = crate::R<TimSpec>;
#[doc = "Field `TIMER` reader - Timer"]
pub type TimerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimSpec;
impl crate::RegisterSpec for TimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim::R`](R) reader structure"]
impl crate::Readable for TimSpec {}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TimSpec {
    const RESET_VALUE: u32 = 0;
}
